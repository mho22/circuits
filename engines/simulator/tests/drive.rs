use simulator::{ driven, Document, Kind, Operation, Point, Segment };


fn point( x : i32, y : i32 ) -> Point
{
	Point { x, y }
}

fn layer( parts : &[ ( Kind, ( i32, i32 ), ( i32, i32 ) ) ] ) -> Document
{
	let mut document = Document::default();

	for &( kind, from, to ) in parts { document.apply( Operation::Add { kind, segment : Segment { from : point( from.0, from.1 ), to : point( to.0, to.1 ) }, mirrored : false } ).unwrap(); }

	document
}

fn pair() -> Document
{
	layer( &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] )
}


#[test]
fn it_finds_a_point_driven_by_the_end_or_the_middle_of_a_wire()
{
	let layers = [ layer( &[ ( Kind::Wire, ( 0, 0 ), ( 4, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 0, 2 ), ( 4, 2 ) ) ] ) ];

	assert!( driven( &layers, 0, 9, point( 0, 0 ) ) );
	assert!( driven( &layers, 0, 9, point( 2, 0 ) ) );
	assert!( driven( &layers, 0, 9, point( 4, 2 ) ) );
	assert!( ! driven( &layers, 0, 9, point( 2, 2 ) ) );
}

#[test]
fn it_ignores_the_part_that_asks()
{
	let layers = [ layer( &[ ( Kind::Wire, ( 0, 0 ), ( 4, 0 ) ) ] ) ];

	assert!( ! driven( &layers, 0, 0, point( 0, 0 ) ) );
}

#[test]
fn it_finds_a_point_driven_by_the_pin_of_another_symbol()
{
	let layers = [ layer( &[ ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) ), ( Kind::Symbol { layer : 1, rotation : 0 }, ( 14, 10 ), ( 14, 10 ) ) ] ), pair() ];

	assert!( driven( &layers, 0, 0, point( 12, 10 ) ) );
	assert!( ! driven( &layers, 0, 0, point( 8, 10 ) ) );
}
