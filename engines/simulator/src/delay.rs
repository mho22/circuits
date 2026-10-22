use crate::union_find::UnionFind;
use crate::{ advance, Device, Element, History, Kind, Logic, Model, Netlist, Plan, DELAY, THRESHOLD };


const SUPPLY : f64 = 5.0;
const RESOLUTION : f64 = 10.0;
const STEPS : usize = 10_000;


struct Graph
{
	weights : Vec<f64>,
	successors : Vec<Vec<usize>>,
	ends : Vec<bool>,
}

impl Graph
{
	fn reach( &self, node : usize, visiting : &mut [ bool ], memo : &mut [ Option<Option<f64>> ] ) -> Option<f64>
	{
		if let Some( known ) = memo[ node ] { return known; }

		visiting[ node ] = true;

		let mut best = self.ends[ node ].then_some( 0.0 );

		for &next in &self.successors[ node ]
		{
			if visiting[ next ] { continue; }

			let Some( length ) = self.reach( next, visiting, memo ) else { continue; };

			best = Some( best.map_or( length, | best : f64 | best.max( length ) ) );
		}

		visiting[ node ] = false;

		let reach = best.map( | best | best + self.weights[ node ] );

		memo[ node ] = Some( reach );

		reach
	}
}


fn stages( elements : &[ Element ], drives : &[ usize ], probes : &[ usize ] ) -> f64
{
	let netlist = Netlist::build( elements );
	let net = | point | netlist.net( point );
	let mut fixed = vec![ false; netlist.count ];

	for ( _, element ) in elements.iter().enumerate().filter( | ( index, element ) | ! drives.contains( index ) && matches!( element.kind, Kind::Ground | Kind::Supply { .. } | Kind::Battery { .. } ) )
	{
		for point in element.terminals() { fixed[ net( point ) ] = true; }
	}

	let transistors : Vec<&Element> = elements.iter().filter( | element | matches!( element.kind, Kind::Npn | Kind::Pnp | Kind::Nmos | Kind::Pmos ) ).collect();
	let passives = elements.iter().filter( | element | matches!( element.kind, Kind::Resistor { .. } | Kind::Diode | Kind::Led { .. } ) );
	let mut forest = UnionFind::new( netlist.count );

	for points in transistors.iter().map( | element | element.terminals()[ 1 .. ].to_vec() ).chain( passives.map( Element::terminals ) )
	{
		let nets : Vec<usize> = points.iter().map( | &point | net( point ) ).filter( | &net | ! fixed[ net ] ).collect();

		if let [ a, b ] = nets[ .. ] { forest.union( a, b ); }
	}

	let count = netlist.count;
	let mut graph = Graph { weights : vec![ 0.0; count ], successors : vec![ Vec::new(); count ], ends : vec![ false; count ] };

	for element in &transistors
	{
		let terminals = element.terminals();
		let control = net( terminals[ 0 ] );

		for channel in terminals[ 1 .. ].iter().map( | &point | net( point ) ).filter( | &net | ! fixed[ net ] )
		{
			let stage = forest.find( channel );

			graph.weights[ stage ] = DELAY;

			if ! fixed[ control ] { graph.successors[ forest.find( control ) ].push( stage ); }
		}
	}

	for &probe in probes { graph.ends[ forest.find( net( elements[ probe ].segment.to ) ) ] = true; }

	let mut visiting = vec![ false; count ];
	let mut memo = vec![ None; count ];

	drives.iter().filter_map( | &drive | graph.reach( forest.find( net( elements[ drive ].segment.to ) ), &mut visiting, &mut memo ) ).fold( 0.0, f64::max )
}

fn measure( elements : &[ Element ], devices : &[ Device ], models : &[ Option<Model> ], drives : &[ usize ], probes : &[ usize ], settled : &[ ( usize, Logic ) ] ) -> Option<f64>
{
	let step = devices.iter().filter_map( | device | models.get( device.layer ).and_then( Option::as_ref ) ).map( | model | model.delay ).fold( f64::INFINITY, f64::min ) / RESOLUTION;
	let history = History::default();
	let mut longest : f64 = 0.0;

	for ( row, seed ) in settled
	{
		for bit in 0 .. drives.len()
		{
			let flipped = row ^ 1 << bit;
			let mut elements = elements.to_vec();

			for ( &drive, index ) in drives.iter().zip( 0 .. ) { elements[ drive ].kind = Kind::Supply { volts : if flipped >> index & 1 == 1 { SUPPLY } else { 0.0 } }; }

			let plan = Plan::build( &elements, devices );
			let highs = | logic : &mut Logic | advance( &elements, devices, &plan, models, &history, step, logic ).ok().map( | solution | probes.iter().map( | &probe | solution.readings[ probe ].potentials[ 0 ] > THRESHOLD ).collect::<Vec<bool>>() );
			let mut logic = seed.clone();
			let mut last = highs( &mut logic )?;
			let start = logic.time;
			let mut changed = start;

			for _ in 0 .. STEPS
			{
				if ! logic.pending( f64::INFINITY ) { break; }

				let now = highs( &mut logic )?;

				if now != last { changed = logic.time; }

				last = now;
			}

			longest = longest.max( ( ( changed - start ) / step ).round() * step );
		}
	}

	Some( longest )
}


pub fn delay( elements : &[ Element ], devices : &[ Device ], models : &[ Option<Model> ], drives : &[ usize ], probes : &[ usize ], settled : &[ ( usize, Logic ) ] ) -> f64
{
	let delay = if devices.is_empty() { stages( elements, drives, probes ) } else { measure( elements, devices, models, drives, probes, settled ).unwrap_or( 0.0 ) };

	delay.max( DELAY )
}
