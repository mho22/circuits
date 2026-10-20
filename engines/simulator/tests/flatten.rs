use simulator::{ flatten, Document, Element, Error, Kind, Operation, Point, Registry, Segment };


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
	layer( &[ ( Kind::Pin, ( 0, 0 ), ( 0, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) ), ( Kind::Pin, ( 4, 0 ), ( 4, 0 ) ) ] )
}

fn join( id : u32, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { id, kind : Kind::Switch { closed : true }, segment : segment( from, to ), mirrored : false }
}


#[test]
fn it_keeps_the_parts_of_the_drawing_with_an_id_for_each()
{
	let drawing = layer( &[ ( Kind::Wire, ( 0, 0 ), ( 2, 0 ) ), ( Kind::Pin, ( 2, 0 ), ( 2, 0 ) ) ] );

	assert_eq!( flatten( &[ drawing ], &mut Registry::default() ), Ok( vec![
		Element { id : 0, kind : Kind::Wire, segment : segment( ( 0, 0 ), ( 2, 0 ) ), mirrored : false },
		Element { id : 1, kind : Kind::Pin, segment : segment( ( 2, 0 ), ( 2, 0 ) ), mirrored : false },
	] ) );
}

#[test]
fn it_replaces_a_symbol_with_its_parts_joined_to_the_box()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ) ] );

	assert_eq!( flatten( &[ drawing, resistor() ], &mut Registry::default() ), Ok( vec![
		join( 0, ( SCOPE, 0 ), ( 8, 10 ) ),
		join( 1, ( SCOPE + 4, 0 ), ( 12, 10 ) ),
		Element { id : 2, kind : Kind::Resistor { ohms : 1000.0 }, segment : segment( ( SCOPE, 0 ), ( SCOPE + 4, 0 ) ), mirrored : false },
	] ) );
}

#[test]
fn it_gives_each_instance_its_own_parts()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ), ( Kind::Symbol { layer : 1, rotation : 1 }, ( 20, 10 ), ( 20, 10 ) ) ] );
	let elements = flatten( &[ drawing, resistor() ], &mut Registry::default() ).unwrap();

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
	let outer = layer( &[ ( Kind::Pin, ( 0, 0 ), ( 0, 0 ) ), ( Kind::Symbol { layer : 2, rotation : 0 }, ( 2, 0 ), ( 2, 0 ) ), ( Kind::Pin, ( 4, 0 ), ( 4, 0 ) ) ] );
	let elements = flatten( &[ drawing, outer, resistor() ], &mut Registry::default() ).unwrap();

	assert_eq!( elements.len(), 5 );
	assert_eq!( elements[ 2 ], join( 2, ( 2 * SCOPE, 0 ), ( SCOPE, 0 ) ) );
	assert_eq!( elements[ 4 ].segment, segment( ( 2 * SCOPE, 0 ), ( 2 * SCOPE + 4, 0 ) ) );
}

#[test]
fn it_gives_the_same_ids_after_a_change_elsewhere()
{
	let mut registry = Registry::default();
	let mut drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ) ] );

	flatten( &[ layer( &[] ), resistor() ], &mut registry ).unwrap();
	flatten( &[ layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ) ] ), resistor() ], &mut registry ).unwrap();

	drawing.apply( Operation::Insert { element : Element { id : 5, kind : Kind::Wire, segment : segment( ( 0, 0 ), ( 2, 0 ) ), mirrored : false }, index : 0 } ).unwrap();

	let elements = flatten( &[ drawing, resistor() ], &mut registry ).unwrap();

	assert_eq!( elements.iter().map( | element | element.id ).collect::<Vec<_>>(), vec![ 3, 0, 1, 2 ] );
}

#[test]
fn it_rejects_a_symbol_that_holds_itself()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ) ] );
	let looped = || layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 0, 0 ), ( 0, 0 ) ) ] );

	assert_eq!( flatten( &[ drawing, looped() ], &mut Registry::default() ), Err( Error::Recursive ) );
	assert_eq!( flatten( &[ layer( &[ ( Kind::Symbol { layer : 0, rotation : 0 }, ( 0, 0 ), ( 0, 0 ) ) ] ) ], &mut Registry::default() ), Err( Error::Recursive ) );
	assert_eq!( flatten( &[ layer( &[] ), looped() ], &mut Registry::default() ), Err( Error::Recursive ) );
}

#[test]
fn it_rejects_a_symbol_without_a_layer()
{
	let drawing = layer( &[ ( Kind::Symbol { layer : 7, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ) ] );

	assert_eq!( flatten( &[ drawing ], &mut Registry::default() ), Err( Error::Layerless ) );
	assert_eq!( flatten( &[ layer( &[] ), layer( &[ ( Kind::Symbol { layer : 7, rotation : 0 }, ( 0, 0 ), ( 0, 0 ) ) ] ) ], &mut Registry::default() ), Err( Error::Layerless ) );
}
