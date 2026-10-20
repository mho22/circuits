use std::collections::HashMap;

use crate::{ companion, distribute, eliminate, limit, Branch, Element, Error, History, Kind, Netlist, Reading, Segment, Solution, UnionFind };


const ITERATIONS : usize = 100;
const TOLERANCE : f64 = 1e-6;


pub fn solve( elements : &[ Element ], history : &History, step : f64 ) -> Result<Solution, Error>
{
	let netlist = Netlist::build( elements );
	let mut islands = UnionFind::new( netlist.count );

	for element in elements
	{
		if matches!( element.kind, Kind::Wire | Kind::Ground | Kind::Switch { .. } ) { continue; }

		islands.union( netlist.net( element.segment.from ), netlist.net( element.segment.to ) );
	}

	let mut references = HashMap::new();

	if let Some( earth ) = netlist.earth { references.insert( islands.find( earth ), earth ); }

	for net in 0 .. netlist.count { references.entry( islands.find( net ) ).or_insert( net ); }

	let mut unknowns = vec![ None; netlist.count ];
	let mut size = 0;

	for net in 0 .. netlist.count
	{
		if references[ &islands.find( net ) ] == net { continue; }

		unknowns[ net ] = Some( size );

		size += 1;
	}

	let nodes = size;
	let batteries : Vec<usize> = ( 0 .. elements.len() ).filter( | &index | matches!( elements[ index ].kind, Kind::Battery { .. } ) ).collect();

	size += batteries.len();

	let assemble = | guesses : &[ f64 ] | -> Result<Vec<f64>, Error>
	{
		let mut matrix = vec![ 0.0; size * size ];
		let mut rhs = vec![ 0.0; size ];

		let mut stamp = | row : Option<usize>, column : Option<usize>, value : f64 |
		{
			if let ( Some( row ), Some( column ) ) = ( row, column ) { matrix[ row * size + column ] += value; }
		};

		for ( element, &guess ) in elements.iter().zip( guesses )
		{
			let Some( ( conductance, source ) ) = companion( element, history, step, guess ) else { continue; };

			let a = unknowns[ netlist.net( element.segment.from ) ];
			let b = unknowns[ netlist.net( element.segment.to ) ];

			stamp( a, a, conductance );
			stamp( b, b, conductance );
			stamp( a, b, - conductance );
			stamp( b, a, - conductance );

			if let Some( a ) = a { rhs[ a ] += source; }

			if let Some( b ) = b { rhs[ b ] -= source; }
		}

		for ( offset, &index ) in batteries.iter().enumerate()
		{
			let Kind::Battery { volts } = elements[ index ].kind else { continue; };

			let row = Some( nodes + offset );
			let negative = unknowns[ netlist.net( elements[ index ].segment.from ) ];
			let positive = unknowns[ netlist.net( elements[ index ].segment.to ) ];

			stamp( negative, row, 1.0 );
			stamp( positive, row, -1.0 );
			stamp( row, positive, 1.0 );
			stamp( row, negative, -1.0 );

			rhs[ nodes + offset ] = volts;
		}

		eliminate( matrix, rhs )
	};

	let mut guesses : Vec<f64> = elements.iter().map( | element | history.get( element.id ) ).collect();
	let mut iteration = 0;

	let solution = loop
	{
		let solution = assemble( &guesses )?;

		let voltage = | net : usize | unknowns[ net ].map_or( 0.0, | unknown : usize | solution[ unknown ] );

		let next : Vec<f64> = elements.iter().zip( &guesses ).map( | ( element, &guess ) | match element.kind
		{
			Kind::Diode => limit( voltage( netlist.net( element.segment.from ) ) - voltage( netlist.net( element.segment.to ) ), guess ),
			_ => guess,
		} ).collect();

		if next.iter().zip( &guesses ).all( | ( a, b ) | ( a - b ).abs() <= TOLERANCE ) { break solution; }

		iteration += 1;

		if iteration == ITERATIONS { return Err( Error::Unsettled ); }

		guesses = next;
	};

	let voltage = | net : usize | unknowns[ net ].map_or( 0.0, | unknown | solution[ unknown ] );

	let mut readings : Vec<Reading> = elements.iter().zip( &guesses ).map( | ( element, &guess ) |
	{
		let from = voltage( netlist.net( element.segment.from ) );
		let to = voltage( netlist.net( element.segment.to ) );

		let current = match ( element.kind, companion( element, history, step, guess ) )
		{
			( _, Some( ( conductance, source ) ) ) => conductance * ( from - to ) - source,
			( Kind::Battery { .. }, _ ) => solution[ nodes + batteries.iter().position( | &index | elements[ index ].id == element.id ).unwrap() ],
			( Kind::Switch { .. }, _ ) => 0.0,
			_ => f64::NAN,
		};

		Reading { from, to, current }
	} ).collect();

	let earth = netlist.points.len();
	let mut injections = vec![ 0.0; earth + 1 ];

	for ( element, reading ) in elements.iter().zip( &readings )
	{
		if matches!( element.kind, Kind::Wire | Kind::Ground | Kind::Switch { .. } ) { continue; }

		injections[ netlist.index( element.segment.to ) ] += reading.current;
		injections[ netlist.index( element.segment.from ) ] -= reading.current;
	}

	let switches : Vec<usize> = ( 0 .. elements.len() ).filter( | &index | elements[ index ].kind == Kind::Switch { closed : true } ).collect();
	let grounds : Vec<usize> = ( 0 .. elements.len() ).filter( | &index | elements[ index ].kind == Kind::Ground ).collect();

	let mut edges = netlist.edges.clone();

	edges.extend( &netlist.links );
	edges.extend( grounds.iter().map( | &index | ( netlist.index( elements[ index ].segment.from ), earth ) ) );

	let currents = distribute( earth + 1, &edges, &injections );

	for ( offset, &index ) in switches.iter().chain( &grounds ).enumerate() { readings[ index ].current = currents[ netlist.edges.len() + offset ]; }

	let branches = netlist.edges.iter().zip( &currents ).map( | ( &( a, b ), &current ) | Branch
	{
		segment : Segment { from : netlist.points[ a ], to : netlist.points[ b ] },
		voltage : voltage( netlist.nets[ a ] ),
		current,
	} ).collect();

	Ok( Solution { readings, branches } )
}
