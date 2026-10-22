use crate::flatten::STRIDE;
use crate::{ delay, flatten, settle, Device, Document, Drive, Element, History, Kind, Logic, Model, Netlist, Plan, Point, Registry, Segment, Solution, FLOOR, THRESHOLD };


const INPUTS : usize = 8;
const LOW : f64 = 1.0;
const HIGH : f64 = 4.0;
const SUPPLY : f64 = 5.0;
const LOAD : f64 = 1000.0;
const CEILING : f64 = 1e9;
const STATES : usize = 64;


fn run( elements : &[ Element ], devices : &[ Device ], models : &[ Option<Model> ], step : f64, seed : &Logic ) -> Option<( Solution, Logic )>
{
	let mut logic = seed.clone();
	let solution = settle( elements, devices, &Plan::build( elements, devices ), models, &History::default(), step, &mut logic ).ok()?;

	Some( ( solution, logic ) )
}

fn load( elements : &[ Element ], point : Point, volts : f64 ) -> Vec<Element>
{
	let hidden = Point { x : - STRIDE, y : 0 };
	let next = elements.iter().map( | element | element.id + 1 ).max().unwrap_or( 0 );
	let supply = Element { id : next, kind : Kind::Supply { volts }, segment : Segment { from : hidden, to : hidden }, mirrored : false };
	let resistor = Element { id : next + 1, kind : Kind::Resistor { ohms : LOAD }, segment : Segment { from : hidden, to : point }, mirrored : false };

	[ elements, &[ supply, resistor ] ].concat()
}

fn cycle( net : usize, edges : &[ Vec<usize> ], fixed : &[ bool ], states : &mut [ u8 ] ) -> bool
{
	if fixed[ net ] || states[ net ] == 2 { return false; }

	if states[ net ] == 1 { return true; }

	states[ net ] = 1;

	if edges[ net ].iter().any( | &next | cycle( next, edges, fixed, states ) ) { return true; }

	states[ net ] = 2;

	false
}

fn looped( elements : &[ Element ], devices : &[ Device ], drives : &[ usize ] ) -> bool
{
	let netlist = Netlist::build( elements );
	let mut fixed = vec![ false; netlist.count ];
	let mut edges = vec![ Vec::new(); netlist.count ];

	for ( _, element ) in elements.iter().enumerate().filter( | ( index, element ) | ! drives.contains( index ) && matches!( element.kind, Kind::Ground | Kind::Supply { .. } | Kind::Battery { .. } ) )
	{
		for point in element.terminals() { fixed[ netlist.net( point ) ] = true; }
	}

	for element in elements.iter().filter( | element | matches!( element.kind, Kind::Npn | Kind::Pnp | Kind::Nmos | Kind::Pmos ) )
	{
		let terminals = element.terminals();

		edges[ netlist.net( terminals[ 0 ] ) ].extend( terminals[ 1 .. ].iter().map( | &point | netlist.net( point ) ) );
	}

	for device in devices
	{
		for &input in &device.inputs
		{
			edges[ netlist.net( elements[ input ].segment.to ) ].extend( device.outputs.iter().map( | &( _, resistor ) | netlist.net( elements[ resistor ].segment.to ) ) );
		}
	}

	let mut states = vec![ 0; netlist.count ];

	( 0 .. netlist.count ).any( | net | cycle( net, &edges, &fixed, &mut states ) )
}

fn number<T : PartialEq>( keys : Vec<T> ) -> Vec<usize>
{
	let mut seen : Vec<&T> = Vec::new();

	keys.iter().map( | key | seen.iter().position( | &other | other == key ).unwrap_or_else( || { seen.push( key ); seen.len() - 1 } ) ).collect()
}

fn minimize( entries : &[ ( Vec<Drive>, usize ) ], width : usize ) -> ( Vec<Vec<Drive>>, Vec<usize> )
{
	let count = entries.len() / width;
	let row = | state : usize | &entries[ state * width .. ( state + 1 ) * width ];
	let mut classes = number( ( 0 .. count ).map( | state | row( state ).iter().map( | ( drives, _ ) | drives.iter().map( Drive::high ).collect::<Vec<bool>>() ).collect::<Vec<_>>() ).collect() );

	loop
	{
		let refined = number( ( 0 .. count ).map( | state | ( classes[ state ], row( state ).iter().map( | &( _, next ) | classes[ next ] ).collect::<Vec<usize>>() ) ).collect() );

		if refined == classes { break; }

		classes = refined;
	}

	let representatives : Vec<usize> = ( 0 .. count ).filter( | &state | classes[ .. state ].iter().all( | &class | class != classes[ state ] ) ).collect();

	representatives.iter().flat_map( | &state | row( state ) ).map( | ( drives, next ) | ( drives.clone(), classes[ *next ] ) ).unzip()
}


pub fn characterize( layers : &[ Document ], models : &[ Option<Model> ], layer : usize, step : f64 ) -> Option<Model>
{
	let pins : Vec<&Element> = layers.get( layer )?.elements().iter().filter( | element | element.kind.pin() ).collect();
	let inputs : Vec<u32> = pins.iter().filter( | pin | pin.kind != Kind::Output ).map( | pin | pin.id ).collect();
	let outputs : Vec<u32> = pins.iter().filter( | pin | pin.kind == Kind::Output ).map( | pin | pin.id ).collect();

	if outputs.is_empty() || inputs.len() > INPUTS { return None; }

	let mut registry = Registry::default();
	let ( mut elements, devices ) = flatten( layers, &mut registry, models, layer, &[] ).ok()?;

	let active = ! devices.is_empty() || elements.iter().any( | element | matches!( element.kind, Kind::Npn | Kind::Pnp | Kind::Nmos | Kind::Pmos ) );
	let timed = elements.iter().any( | element | matches!( element.kind, Kind::Capacitor { .. } | Kind::Inductor { .. } | Kind::Clock { .. } ) );

	let find = | key : &[ u32 ] | registry.find( key ).and_then( | id | elements.iter().position( | element | element.id == id ) );
	let drives : Vec<usize> = inputs.iter().map( | &id | find( &[ id, 0 ] ) ).collect::<Option<_>>()?;
	let probes : Vec<usize> = outputs.iter().map( | &id | find( &[ id ] ) ).collect::<Option<_>>()?;

	if ! active || timed || looped( &elements, &[], &drives ) { return None; }

	let sequential = looped( &elements, &devices, &drives );
	let netlist = Netlist::build( &elements );
	let net = | index : usize | netlist.net( elements[ index ].segment.to );

	if probes.iter().any( | &probe | drives.iter().any( | &drive | net( probe ) == net( drive ) ) ) { return None; }

	let width = 1usize << inputs.len();
	let mut seeds = vec![ Logic::default() ];
	let mut entries : Vec<( Vec<Drive>, usize )> = Vec::new();
	let mut settled : Vec<( usize, Logic )> = Vec::new();

	while entries.len() < seeds.len() * width
	{
		if seeds.len() > STATES { return None; }

		let seed = seeds[ entries.len() / width ].clone();
		let row = entries.len() % width;

		for ( &drive, bit ) in drives.iter().zip( 0 .. ) { elements[ drive ].kind = Kind::Supply { volts : if row >> bit & 1 == 1 { SUPPLY } else { 0.0 } }; }

		let ( open, after ) = run( &elements, &devices, models, step, &seed )?;

		settled.push( ( row, after.clone() ) );

		let outputs = probes.iter().map( | &probe |
		{
			let volts = open.readings[ probe ].potentials[ 0 ];

			if volts > LOW && volts < HIGH { return None; }

			let target = if volts > THRESHOLD { 0.0 } else { SUPPLY };
			let loaded = run( &load( &elements, elements[ probe ].segment.to, target ), &devices, models, step, &after )?.0.readings[ probe ].potentials[ 0 ];
			let ohms = ( ( volts - loaded ) / ( ( loaded - target ) / LOAD ) ).max( FLOOR ).min( CEILING );

			Some( Drive { volts, ohms } )
		} ).collect::<Option<Vec<Drive>>>()?;

		let known = seeds.iter().position( | seed | seed.applied == after.applied && seed.states == after.states );
		let next = match known
		{
			_ if ! sequential => 0,
			Some( state ) => state,
			None => { seeds.push( after ); seeds.len() - 1 },
		};

		entries.push( ( outputs, next ) );
	}

	let ( rows, next ) = minimize( &entries, width );

	Some( Model { rows, next, delay : delay( &elements, &devices, models, &drives, &probes, &settled ) } )
}
