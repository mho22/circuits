use simulator::{ flatten, Device, Document, Drive, Element, Error, Kind, Model, Operation, Point, Registry, Segment, DELAY, FLOOR };


const SCOPE : i32 = 1 << 16;


fn segment( from : ( i32, i32 ), to : ( i32, i32 ) ) -> Segment
{
	Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }
}

fn layer( parts : &[ ( Kind, ( i32, i32 ), ( i32, i32 ) ) ] ) -> Document
{
	let mut document = Document::default();

	for &( kind, from, to ) in parts { document.apply( Operation::Add { kind, segment : segment( from, to ), mirrored : false } ).unwrap(); }

	document
}

fn resistor() -> Document
{
	let mut document = layer( &[ ( Kind::Output, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] );

	document.apply( Operation::Describe { name : String::new(), drawing : Vec::new(), layout : vec![ Point { x : -2, y : 0 }, Point { x : 2, y : 0 } ] } ).unwrap();

	document
}

fn join( id : u32, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { id, kind : Kind::Switch { closed : true }, segment : segment( from, to ), mirrored : false }
}


#[test]
fn it_keeps_the_parts_of_the_drawing_with_an_id_for_each()
{
	let drawing = layer( &[ ( Kind::Wire, ( 0, 0 ), ( 2, 0 ) ), ( Kind::Output, ( 3, 0 ), ( 2, 0 ) ) ] );

	assert_eq!( flatten( &[ drawing ], &mut Registry::default(), &[], 0, &[] ).map( | ( elements, _ ) | elements ), Ok( vec![
		Element { id : 0, kind : Kind::Wire, segment : segment( ( 0, 0 ), ( 2, 0 ) ), mirrored : false },
		Element { id : 1, kind : Kind::Output, segment : segment( ( 3, 0 ), ( 2, 0 ) ), mirrored : false },
	] ) );
}

#[test]
fn it_replaces_a_symbol_with_its_parts_joined_to_the_box()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ) ] );

	assert_eq!( flatten( &[ drawing, resistor() ], &mut Registry::default(), &[], 0, &[] ).map( | ( elements, _ ) | elements ), Ok( vec![
		join( 0, ( SCOPE, 0 ), ( 8, 10 ) ),
		join( 1, ( SCOPE + 4, 0 ), ( 12, 10 ) ),
		Element { id : 2, kind : Kind::Resistor { ohms : 1000.0 }, segment : segment( ( SCOPE, 0 ), ( SCOPE + 4, 0 ) ), mirrored : false },
	] ) );
}

#[test]
fn it_gives_each_instance_its_own_parts()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ), ( Kind::Symbol { layer : 1, rotation : 1 }, ( 20, 10 ), ( 20, 10 ) ) ] );
	let elements = flatten( &[ drawing, resistor() ], &mut Registry::default(), &[], 0, &[] ).unwrap().0;

	assert_eq!( elements[ 3 .. ], [
		join( 3, ( 2 * SCOPE, 0 ), ( 20, 8 ) ),
		join( 4, ( 2 * SCOPE + 4, 0 ), ( 20, 12 ) ),
		Element { id : 5, kind : Kind::Resistor { ohms : 1000.0 }, segment : segment( ( 2 * SCOPE, 0 ), ( 2 * SCOPE + 4, 0 ) ), mirrored : false },
	] );
}

#[test]
fn it_opens_a_symbol_inside_a_symbol()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ) ] );
	let mut outer = layer( &[ ( Kind::Output, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Symbol { layer : 2, rotation : 0 }, ( 2, 0 ), ( 2, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] );

	outer.apply( Operation::Describe { name : String::new(), drawing : Vec::new(), layout : vec![ Point { x : -2, y : 0 }, Point { x : 2, y : 0 } ] } ).unwrap();

	let elements = flatten( &[ drawing, outer, resistor() ], &mut Registry::default(), &[], 0, &[] ).unwrap().0;

	assert_eq!( elements.len(), 5 );
	assert_eq!( elements[ 2 ], join( 2, ( 2 * SCOPE, 0 ), ( SCOPE, 0 ) ) );
	assert_eq!( elements[ 4 ].segment, segment( ( 2 * SCOPE, 0 ), ( 2 * SCOPE + 4, 0 ) ) );
}

#[test]
fn it_gives_the_same_ids_after_a_change_elsewhere()
{
	let mut registry = Registry::default();
	let mut drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ) ] );

	flatten( &[ layer( &[] ), resistor() ], &mut registry, &[], 0, &[] ).unwrap();
	flatten( &[ layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ) ] ), resistor() ], &mut registry, &[], 0, &[] ).unwrap();

	drawing.apply( Operation::Insert { element : Element { id : 5, kind : Kind::Wire, segment : segment( ( 0, 0 ), ( 2, 0 ) ), mirrored : false }, index : 0 } ).unwrap();

	let elements = flatten( &[ drawing, resistor() ], &mut registry, &[], 0, &[] ).unwrap().0;

	assert_eq!( elements.iter().map( | element | element.id ).collect::<Vec<_>>(), vec![ 3, 0, 1, 2 ] );
}

#[test]
fn it_rejects_a_symbol_that_holds_itself()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ) ] );
	let looped = || layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 0, 0 ), ( 0, 0 ) ) ] );

	assert_eq!( flatten( &[ drawing, looped() ], &mut Registry::default(), &[], 0, &[] ), Err( Error::Recursive ) );
	assert_eq!( flatten( &[ layer( &[ ( Kind::Symbol { layer : 0, rotation : 0 }, ( 0, 0 ), ( 0, 0 ) ) ] ) ], &mut Registry::default(), &[], 0, &[] ), Err( Error::Recursive ) );
	assert_eq!( flatten( &[ layer( &[] ), looped() ], &mut Registry::default(), &[], 0, &[] ), Err( Error::Recursive ) );
}

#[test]
fn it_rejects_a_symbol_without_a_layer()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 7, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ) ] );

	assert_eq!( flatten( &[ drawing ], &mut Registry::default(), &[], 0, &[] ), Err( Error::Layerless ) );
	assert_eq!( flatten( &[ layer( &[] ), layer( &[ ( Kind::Symbol { layer : 7, rotation : 0 }, ( 0, 0 ), ( 0, 0 ) ) ] ) ], &mut Registry::default(), &[], 0, &[] ), Err( Error::Layerless ) );
}

fn input( high : bool ) -> Document
{
	layer( &[ ( Kind::Input { high }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] )
}

fn supply( id : u32, volts : f64, at : ( i32, i32 ) ) -> Element
{
	Element { id, kind : Kind::Supply { volts }, segment : segment( at, at ), mirrored : false }
}

#[test]
fn it_drives_an_input_of_the_drawing_with_its_level_at_the_end_of_its_lead()
{
	let drawing = layer( &[ ( Kind::Input { high : true }, ( 1, 0 ), ( 2, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] );
	let elements = flatten( &[ drawing ], &mut Registry::default(), &[], 0, &[] ).unwrap().0;

	assert_eq!( elements.len(), 3 );
	assert_eq!( elements[ 1 ], supply( 1, 5.0, ( 2, 0 ) ) );
}

#[test]
fn it_turns_a_toggle_into_a_vcc_at_its_level_with_its_id_at_the_end_of_its_lead()
{
	let drawing = layer( &[ ( Kind::Toggle { high : true }, ( 1, 0 ), ( 2, 0 ) ), ( Kind::Toggle { high : false }, ( 3, 0 ), ( 3, 2 ) ) ] );
	let elements = flatten( &[ drawing ], &mut Registry::default(), &[], 0, &[] ).unwrap().0;

	assert_eq!( elements, vec![ supply( 0, 5.0, ( 2, 0 ) ), supply( 1, 0.0, ( 3, 2 ) ) ] );
}

#[test]
fn it_keeps_a_clock_at_the_end_of_its_lead()
{
	let drawing = layer( &[ ( Kind::Clock { hertz : 1e6 }, ( 1, 0 ), ( 3, 0 ) ) ] );
	let elements = flatten( &[ drawing ], &mut Registry::default(), &[], 0, &[] ).unwrap().0;

	assert_eq!( elements, vec![ Element { id : 0, kind : Kind::Clock { hertz : 1e6 }, segment : segment( ( 3, 0 ), ( 3, 0 ) ), mirrored : false } ] );
}

#[test]
fn it_drives_an_input_of_a_symbol_that_nothing_touches_outside()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ), ( Kind::Wire, ( 12, 10 ), ( 14, 10 ) ) ] );
	let elements = flatten( &[ drawing, input( false ) ], &mut Registry::default(), &[], 0, &[] ).unwrap().0;

	assert_eq!( elements[ 1 ], supply( 1, 0.0, ( SCOPE, 0 ) ) );
	assert_eq!( elements.iter().filter( | element | matches!( element.kind, Kind::Supply { .. } ) ).count(), 1 );
}

#[test]
fn it_leaves_an_input_of_a_symbol_to_the_wire_that_touches_it_outside()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ), ( Kind::Wire, ( 6, 10 ), ( 8, 10 ) ) ] );
	let elements = flatten( &[ drawing, input( true ) ], &mut Registry::default(), &[], 0, &[] ).unwrap().0;

	assert!( elements.iter().all( | element | ! matches!( element.kind, Kind::Supply { .. } ) ) );
}

#[test]
fn it_drives_each_input_of_a_symbol_alone_with_its_level_from_its_layer()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ), ( Kind::Wire, ( 6, 10 ), ( 8, 10 ) ) ] );
	let elements = flatten( &[ drawing, input( true ) ], &mut Registry::default(), &[], 1, &[ 0 ] ).unwrap().0;

	assert_eq!( elements.iter().map( | element | element.kind ).collect::<Vec<_>>(), vec![ Kind::Input { high : true }, Kind::Supply { volts : 5.0 }, Kind::Resistor { ohms : 1000.0 }, Kind::Output ] );
	assert_eq!( elements[ 1 ], supply( 1, 5.0, ( 0, 0 ) ) );
}

#[test]
fn it_turns_a_copy_of_a_symbol_with_a_model_into_a_device()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ), ( Kind::Wire, ( 12, 10 ), ( 14, 10 ) ) ] );
	let model = Model { rows : vec![ vec![ Drive { volts : 5.0, ohms : 10.0 } ]; 2 ], next : vec![ 0; 2 ], delay : DELAY };
	let ( elements, devices ) = flatten( &[ drawing, input( true ) ], &mut Registry::default(), &[ None, Some( model ) ], 0, &[] ).unwrap();

	assert_eq!( elements[ .. 3 ], [
		supply( 0, 5.0, ( SCOPE, 0 ) ),
		Element { id : 1, kind : Kind::Resistor { ohms : FLOOR }, segment : segment( ( SCOPE, 0 ), ( 8, 10 ) ), mirrored : false },
		supply( 2, 0.0, ( SCOPE + 1, 0 ) ),
	] );
	assert_eq!( elements[ 3 ].segment, segment( ( SCOPE + 1, 0 ), ( 12, 10 ) ) );
	assert_eq!( devices, vec![ Device { layer : 1, inputs : vec![ 1 ], outputs : vec![ ( 2, 3 ) ] } ] );
}

#[test]
fn it_reads_an_input_that_a_wire_touches_through_an_open_switch()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ), ( Kind::Wire, ( 6, 10 ), ( 8, 10 ) ) ] );
	let model = Model { rows : vec![ vec![ Drive { volts : 5.0, ohms : 10.0 } ]; 2 ], next : vec![ 0; 2 ], delay : DELAY };
	let ( elements, devices ) = flatten( &[ drawing, input( true ) ], &mut Registry::default(), &[ None, Some( model ) ], 0, &[] ).unwrap();

	assert_eq!( elements[ 0 ], Element { id : 0, kind : Kind::Switch { closed : false }, segment : segment( ( SCOPE, 0 ), ( 8, 10 ) ), mirrored : false } );
	assert_eq!( devices[ 0 ].inputs, vec![ 0 ] );
}
