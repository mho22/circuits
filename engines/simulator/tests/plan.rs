use simulator::{ catalog, flatten, Document, Kind, Operation, Plan, Point, Registry, Segment, Source };


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
	let ( models, _ ) = catalog( &layers, &[], &[], STEP );
	let ( elements, devices ) = flatten( &layers, &mut Registry::default(), &models, 0, &[] ).unwrap();

	Plan::build( &elements, &devices )
}

fn symbol( x : i32 ) -> Part
{
	( Kind::Symbol { layer : 1, rotation : 0 }, ( x, 0 ), ( x, 0 ) )
}


#[test]
fn it_keeps_a_net_between_two_gates_out_of_the_analog_circuit()
{
	let plan = plan( layer( &[ symbol( 10 ), symbol( 16 ), ( Kind::Wire, ( 12, 0 ), ( 14, 0 ) ) ] ) );

	assert_eq!( plan.nets.len(), 2 );
	assert_eq!( plan.nets[ 0 ].readers, vec![ 1 ] );
	assert!( plan.nets[ 1 ].readers.is_empty() );
	assert_eq!( plan.nets[ 0 ].pieces, vec![ Segment { from : Point { x : 12, y : 0 }, to : Point { x : 14, y : 0 } } ] );
	assert_eq!( plan.outputs, vec![ vec![ Some( 0 ) ], vec![ Some( 1 ) ] ] );
	assert!( matches!( plan.inputs[ 0 ][ 0 ], Source::Analog( _ ) ) );
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
	let driven = plan( layer( &[ ( Kind::Clock { hertz : 1e6 }, ( 4, 0 ), ( 8, 0 ) ), symbol( 10 ) ] ) );
	let loaded = plan( layer( &[ ( Kind::Clock { hertz : 1e6 }, ( 4, 0 ), ( 8, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 8, 0 ), ( 8, 4 ) ) ] ) );

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
fn it_keeps_an_output_pin_on_the_net_of_a_gate_out_of_the_analog_circuit()
{
	let plan = plan( layer( &[ symbol( 10 ), ( Kind::Output, ( 13, 0 ), ( 12, 0 ) ) ] ) );

	assert_eq!( plan.outputs, vec![ vec![ Some( 0 ) ] ] );
	assert_eq!( plan.nets[ 0 ].elements.len(), 3 );
	assert!( plan.nets[ 0 ].elements.iter().all( | index | ! plan.analog.contains( index ) ) );
}
