use crate::{ arrange, companion, eliminate, Branch, Element, Error, History, Kind, Reading, Segment, Solution, Topology };


const ITERATIONS : usize = 100;
const TOLERANCE : f64 = 1e-6;
const RELATIVE : f64 = 1e-5;


pub fn solve( elements : &[ Element ], history : &History, step : f64 ) -> Result<Solution, Error>
{
	compute( elements, &Topology::build( elements ), history, step )
}

pub fn compute( elements : &[ Element ], topology : &Topology, history : &History, step : f64 ) -> Result<Solution, Error>
{
	let Topology { netlist, terminals, unknowns, nodes, batteries, taps, joints, flow, order } = topology;
	let nodes = *nodes;
	let size = nodes + batteries.len();

	let assemble = | guesses : &[ [ f64; 2 ] ] | -> Result<Vec<f64>, Error>
	{
		let mut entries = Vec::with_capacity( elements.len() * 4 );
		let mut rhs = vec![ 0.0; size ];

		let mut stamp = | row : Option<usize>, column : Option<usize>, value : f64 |
		{
			if let ( Some( row ), Some( column ) ) = ( row, column ) { entries.push( ( row, column, value ) ); }
		};

		for ( index, element ) in elements.iter().enumerate()
		{
			let nets = &terminals[ index ];

			for part in companion( element, history, step, guesses[ index ] )
			{
				let p = unknowns[ nets[ part.across.0 ] ];
				let q = unknowns[ nets[ part.across.1 ] ];

				for ( &net, &weight ) in nets.iter().zip( &part.weights )
				{
					let row = unknowns[ net ];

					stamp( row, p, weight * part.conductance );
					stamp( row, q, - weight * part.conductance );

					if let Some( row ) = row { rhs[ row ] += weight * part.source; }
				}
			}
		}

		for ( offset, &index ) in batteries.iter().enumerate()
		{
			let ( Kind::Battery { volts } | Kind::Supply { volts } ) = elements[ index ].kind else { continue; };

			let row = Some( nodes + offset );
			let negative = unknowns[ terminals[ index ][ 0 ] ];
			let positive = unknowns[ terminals[ index ][ 1 ] ];

			stamp( negative, row, 1.0 );
			stamp( positive, row, -1.0 );
			stamp( row, positive, 1.0 );
			stamp( row, negative, -1.0 );

			rhs[ nodes + offset ] = volts;
		}

		eliminate( size, &entries, rhs, order.get_or_init( || arrange( size, &entries ) ) )
	};

	let mut guesses : Vec<[ f64; 2 ]> = elements.iter().map( | element | history.get( element.id ) ).collect();
	let mut iteration = 0;

	let solution = loop
	{
		let solution = assemble( &guesses )?;

		let voltage = | net : usize | unknowns[ net ].map_or( 0.0, | unknown : usize | solution[ unknown ] );

		let next : Vec<[ f64; 2 ]> = elements.iter().zip( terminals ).zip( &guesses ).map( | ( ( element, nets ), &guess ) |
		{
			let mut next = guess;

			for ( index, &( a, b ) ) in element.kind.controls().iter().enumerate()
			{
				let across = voltage( nets[ a ] ) - voltage( nets[ b ] );

				next[ index ] = match element.kind
				{
					Kind::Nmos | Kind::Pmos => across,
					_ => element.kind.junction().limit( across, guess[ index ] ),
				};
			}

			next
		} ).collect();

		if next.iter().flatten().zip( guesses.iter().flatten() ).all( | ( a, b ) | ( a - b ).abs() <= TOLERANCE + RELATIVE * a.abs().max( b.abs() ) ) { break solution; }

		iteration += 1;

		if iteration == ITERATIONS { return Err( Error::Unsettled ); }

		guesses = next;
	};

	let voltage = | net : usize | unknowns[ net ].map_or( 0.0, | unknown | solution[ unknown ] );

	let mut readings : Vec<Reading> = elements.iter().enumerate().map( | ( index, element ) |
	{
		let potentials : Vec<f64> = terminals[ index ].iter().map( | &net | voltage( net ) ).collect();
		let parts = companion( element, history, step, guesses[ index ] );

		let currents = match element.kind
		{
			_ if ! parts.is_empty() => ( 0 .. potentials.len() ).map( | terminal | parts.iter().map( | part | part.weights[ terminal ] * part.current( &potentials ) ).sum() ).collect(),
			Kind::Battery { .. } | Kind::Supply { .. } =>
			{
				let current = solution[ nodes + batteries.iter().position( | &battery | battery == index ).unwrap() ];

				vec![ current, - current ]
			},
			Kind::Switch { .. } => vec![ 0.0, 0.0 ],
			_ => vec![ f64::NAN, f64::NAN ],
		};

		Reading { potentials, currents }
	} ).collect();

	let mut injections = vec![ 0.0; netlist.points.len() + 1 ];

	for ( points, reading ) in taps.iter().zip( &readings )
	{
		for ( &point, current ) in points.iter().zip( &reading.currents ) { injections[ point ] -= current; }
	}

	let currents = flow.distribute( &injections );

	for ( offset, &index ) in joints.iter().enumerate()
	{
		let current = currents[ netlist.edges.len() + offset ];

		readings[ index ].currents = vec![ current, - current ];
	}

	let branches = netlist.edges.iter().zip( &currents ).map( | ( &( a, b ), &current ) | Branch
	{
		segment : Segment { from : netlist.points[ a ], to : netlist.points[ b ] },
		voltage : voltage( netlist.nets[ a ] ),
		current,
	} ).collect();

	Ok( Solution { readings, branches } )
}
