use simulator::{ flatten, Catalog, Document, Kind, Operation, Plan, Point, Registry, Segment, Source };


const STEP : f64 = 1e-6;


type Part = ( Kind, ( i32, i32 ), ( i32, i32 ) );


fn layer( parts : &[ Part ] ) -> Document
{
	let mut document = Document::default();

	for &( kind, from, to ) in parts { document.apply( Operation::Add { kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }, mirrored : false } ).unwrap(); }

	document
}

fn inverter() -> Document
{
	layer( &[
		( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		( Kind::Output, ( 6, 0 ), ( 5, 0 ) ),
		( Kind::Supply { volts : 5.0 }, ( 3, -1 ), ( 3, -1 ) ),
		( Kind::Wire, ( 0, -2 ), ( 0, 0 ) ),
		( Kind::Wire, ( 0, 0 ), ( 0, 2 ) ),
		( Kind::Pmos, ( 0, -2 ), ( 2, -2 ) ),
		( Kind::Nmos, ( 0, 2 ), ( 2, 2 ) ),
		( Kind::Wire, ( 3, -3 ), ( 5, -3 ) ),
		( Kind::Wire, ( 5, -3 ), ( 5, 0 ) ),
		( Kind::Wire, ( 5, 0 ), ( 5, 1 ) ),
		( Kind::Wire, ( 5, 1 ), ( 3, 1 ) ),
		( Kind::Ground, ( 3, 3 ), ( 3, 3 ) ),
	] )
}

fn plan( drawing : Document ) -> Plan
{
	let layers = [ drawing, inverter() ];
	let mut catalog = Catalog::new( &layers, &[], &[], STEP );

	while catalog.advance( &layers ) {}

	let ( models, _ ) = catalog.finish();
	let ( elements, devices, netlist ) = flatten( &layers, &mut Registry::default(), &models, 0, &[] ).unwrap();

	Plan::build( &elements, &devices, &netlist )
}

fn symbol( x : i32 ) -> Part
{
	( Kind::Symbol { layer : 1, rotation : 0 }, ( x, 0 ), ( x, 0 ) )
}


#[test]
fn it_keeps_a_net_between_two_gates_out_of_the_analog_circuit()
{
	let plan = plan( layer( &[ symbol( 10 ), symbol( 16 ), ( Kind::Wire, ( 12, 0 ), ( 14, 0 ) ) ] ) );

	assert_eq!( plan.nets.len(), 3 );
	assert_eq!( plan.nets[ 0 ].readers, vec![ 1 ] );
	assert!( plan.nets[ 1 ].readers.is_empty() );
	assert_eq!( plan.nets[ 0 ].pieces, vec![ Segment { from : Point { x : 12, y : 0 }, to : Point { x : 14, y : 0 } } ] );
	assert_eq!( plan.outputs, vec![ vec![ Some( 0 ) ], vec![ Some( 1 ) ] ] );
	assert_eq!( plan.inputs[ 1 ], vec![ Source::Net( 0 ) ] );
	assert!( plan.nets[ 0 ].elements.iter().all( | index | ! plan.analog.contains( index ) ) );
}

#[test]
fn it_keeps_a_net_that_a_closed_switch_joins_between_two_gates_out_of_the_analog_circuit()
{
	let plan = plan( layer( &[ symbol( 10 ), symbol( 16 ), ( Kind::Switch { closed : true }, ( 12, 0 ), ( 14, 0 ) ) ] ) );

	assert_eq!( plan.inputs[ 1 ], vec![ Source::Net( 0 ) ] );
	assert!( plan.nets[ 0 ].elements.iter().all( | index | ! plan.analog.contains( index ) ) );
}

#[test]
fn it_leaves_a_net_with_an_analog_part_or_two_outputs_in_the_analog_circuit()
{
	let loaded = plan( layer( &[ symbol( 10 ), symbol( 14 ), ( Kind::Resistor { ohms : 1000.0 }, ( 12, 0 ), ( 12, 4 ) ) ] ) );
	let fought = plan( layer( &[ symbol( 10 ), symbol( 14 ), ( Kind::Wire, ( 12, 0 ), ( 12, 4 ) ), ( Kind::Symbol { layer : 1, rotation : 2 }, ( 14, 4 ), ( 14, 4 ) ) ] ) );

	assert_eq!( loaded.outputs[ 0 ], vec![ None ] );
	assert!( matches!( loaded.inputs[ 1 ][ 0 ], Source::Analog( _ ) ) );
	assert_eq!( [ fought.outputs[ 0 ][ 0 ], fought.outputs[ 2 ][ 0 ] ], [ None, None ] );
	assert!( matches!( fought.inputs[ 1 ][ 0 ], Source::Analog( _ ) ) );
}

#[test]
fn it_keeps_a_net_that_a_clock_drives_alone_out_of_the_analog_circuit()
{
	let driven = plan( layer( &[ ( Kind::Clock { hertz : 1e6, on : true }, ( 4, 0 ), ( 8, 0 ) ), symbol( 10 ) ] ) );
	let loaded = plan( layer( &[ ( Kind::Clock { hertz : 1e6, on : true }, ( 4, 0 ), ( 8, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 8, 0 ), ( 8, 4 ) ) ] ) );

	assert_eq!( driven.generators.len(), 1 );
	assert_eq!( driven.inputs[ 0 ], vec![ Source::Net( driven.generators[ 0 ].1.unwrap() ) ] );
	assert_eq!( driven.nets[ driven.generators[ 0 ].1.unwrap() ].readers, vec![ 0 ] );
	assert!( ! driven.analog.contains( &driven.generators[ 0 ].0 ) );
	assert_eq!( loaded.generators[ 0 ].1, None );
	assert!( loaded.analog.contains( &loaded.generators[ 0 ].0 ) );
}

#[test]
fn it_keeps_a_net_that_a_toggle_drives_alone_out_of_the_analog_circuit()
{
	let driven = plan( layer( &[ ( Kind::Toggle { high : true }, ( 4, 0 ), ( 8, 0 ) ), symbol( 10 ) ] ) );
	let loaded = plan( layer( &[ ( Kind::Toggle { high : true }, ( 4, 0 ), ( 8, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 8, 0 ), ( 8, 4 ) ) ] ) );

	assert_eq!( driven.inputs[ 0 ], vec![ Source::Net( driven.generators[ 0 ].1.unwrap() ) ] );
	assert!( ! driven.analog.contains( &driven.generators[ 0 ].0 ) );
	assert_eq!( loaded.generators[ 0 ].1, None );
	assert!( loaded.analog.contains( &loaded.generators[ 0 ].0 ) );
}

#[test]
fn it_solves_a_clock_a_toggle_or_a_slider_on_the_analog_circuit_as_a_supply()
{
	for kind in [ Kind::Clock { hertz : 1e6, on : true }, Kind::Toggle { high : true }, Kind::Slider { volts : 2.5 } ]
	{
		let loaded = plan( layer( &[ ( kind, ( 4, 0 ), ( 8, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 8, 0 ), ( 8, 4 ) ) ] ) );
		let position = loaded.analog.iter().position( | &index | index == loaded.generators[ 0 ].0 ).unwrap();

		assert_eq!( loaded.topology.batteries, vec![ position ] );
		assert_eq!( loaded.topology.terminals[ position ][ 0 ], loaded.topology.netlist.earth.unwrap() );
	}
}

#[test]
fn it_keeps_an_output_pin_on_the_net_of_a_gate_out_of_the_analog_circuit()
{
	let plan = plan( layer( &[ symbol( 10 ), ( Kind::Output, ( 13, 0 ), ( 12, 0 ) ) ] ) );

	assert_eq!( plan.outputs, vec![ vec![ Some( 0 ) ] ] );
	assert_eq!( plan.nets[ 0 ].elements.len(), 3 );
	assert!( plan.nets[ 0 ].elements.iter().all( | index | ! plan.analog.contains( index ) ) );
}

#[test]
fn it_keeps_a_free_input_of_a_gate_out_of_the_analog_circuit_on_the_net_of_its_supply()
{
	let plan = plan( layer( &[ symbol( 10 ) ] ) );

	assert_eq!( plan.generators.len(), 1 );
	assert_eq!( plan.inputs[ 0 ], vec![ Source::Net( plan.generators[ 0 ].1.unwrap() ) ] );
	assert!( plan.analog.is_empty() );
}

#[test]
fn it_keeps_a_net_that_nothing_drives_out_of_the_analog_circuit()
{
	let plan = plan( layer( &[ symbol( 10 ), ( Kind::Wire, ( 4, 0 ), ( 8, 0 ) ) ] ) );

	assert!( plan.generators.is_empty() );
	assert!( matches!( plan.inputs[ 0 ][ 0 ], Source::Net( _ ) ) );
	assert!( plan.analog.is_empty() );
}

#[test]
fn it_keeps_an_led_between_two_gates_out_of_the_analog_circuit_unless_an_analog_part_touches_one_of_its_nets()
{
	let parts = [ symbol( 10 ), ( Kind::Symbol { layer : 1, rotation : 2 }, ( 16, 4 ), ( 16, 4 ) ), ( Kind::Led { color : 0 }, ( 12, 0 ), ( 12, 4 ) ), ( Kind::Wire, ( 12, 4 ), ( 14, 4 ) ) ];
	let lit = plan( layer( &parts ) );
	let loaded = plan( layer( &[ &parts[ .. ], &[ ( Kind::Resistor { ohms : 1000.0 }, ( 14, 4 ), ( 14, 8 ) ) ] ].concat() ) );

	assert_eq!( lit.lights.len(), 1 );
	assert_eq!( [ lit.lights[ 0 ].1, lit.lights[ 0 ].2 ], [ lit.outputs[ 0 ][ 0 ].unwrap(), lit.outputs[ 1 ][ 0 ].unwrap() ] );
	assert!( lit.analog.is_empty() );
	assert!( loaded.lights.is_empty() );
	assert_eq!( loaded.outputs[ 1 ], vec![ None ] );
	assert!( loaded.analog.contains( &lit.lights[ 0 ].0 ) );
}
