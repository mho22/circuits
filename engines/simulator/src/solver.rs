use std::collections::HashMap;

use crate::{ distribute, eliminate, Branch, Element, Error, Kind, Netlist, Reading, Segment, Solution, UnionFind };


pub fn solve( elements : &[ Element ] ) -> Result<Solution, Error>
{
	let netlist = Netlist::build( elements );
	let mut islands = UnionFind::new( netlist.count );

	for element in elements
	{
		if matches!( element.kind, Kind::Wire | Kind::Ground ) { continue; }

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

	let mut matrix = vec![ 0.0; size * size ];
	let mut rhs = vec![ 0.0; size ];

	let mut stamp = | row : Option<usize>, column : Option<usize>, value : f64 |
	{
		if let ( Some( row ), Some( column ) ) = ( row, column ) { matrix[ row * size + column ] += value; }
	};

	for element in elements
	{
		let Kind::Resistor { ohms } = element.kind else { continue; };

		let a = unknowns[ netlist.net( element.segment.from ) ];
		let b = unknowns[ netlist.net( element.segment.to ) ];
		let conductance = 1.0 / ohms;

		stamp( a, a, conductance );
		stamp( b, b, conductance );
		stamp( a, b, - conductance );
		stamp( b, a, - conductance );
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

	let solution = eliminate( matrix, rhs )?;

	let voltage = | net : usize | unknowns[ net ].map_or( 0.0, | unknown | solution[ unknown ] );

	let mut readings : Vec<Reading> = elements.iter().map( | element |
	{
		let from = voltage( netlist.net( element.segment.from ) );
		let to = voltage( netlist.net( element.segment.to ) );

		let current = match element.kind
		{
			Kind::Resistor { ohms } => ( from - to ) / ohms,
			Kind::Battery { .. } => solution[ nodes + batteries.iter().position( | &index | elements[ index ].id == element.id ).unwrap() ],
			_ => f64::NAN,
		};

		Reading { from, to, current }
	} ).collect();

	let earth = netlist.points.len();
	let mut injections = vec![ 0.0; earth + 1 ];

	for ( element, reading ) in elements.iter().zip( &readings )
	{
		if matches!( element.kind, Kind::Wire | Kind::Ground ) { continue; }

		injections[ netlist.index( element.segment.to ) ] += reading.current;
		injections[ netlist.index( element.segment.from ) ] -= reading.current;
	}

	let grounds : Vec<usize> = ( 0 .. elements.len() ).filter( | &index | elements[ index ].kind == Kind::Ground ).collect();

	let mut edges = netlist.edges.clone();

	edges.extend( grounds.iter().map( | &index | ( netlist.index( elements[ index ].segment.from ), earth ) ) );

	let currents = distribute( earth + 1, &edges, &injections );

	for ( offset, &index ) in grounds.iter().enumerate() { readings[ index ].current = currents[ netlist.edges.len() + offset ]; }

	let branches = netlist.edges.iter().zip( &currents ).map( | ( &( a, b ), &current ) | Branch
	{
		segment : Segment { from : netlist.points[ a ], to : netlist.points[ b ] },
		voltage : voltage( netlist.nets[ a ] ),
		current,
	} ).collect();

	Ok( Solution { readings, branches } )
}
