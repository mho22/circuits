use simulator::{ Buses, Document, Error, Kind, Operation, Point, Segment };


type Part = ( Kind, ( i32, i32 ), ( i32, i32 ) );


fn point( x : i32, y : i32 ) -> Point
{
	Point { x, y }
}

fn layer( parts : &[ Part ] ) -> Document
{
	let mut document = Document::default();

	for &( kind, from, to ) in parts { document.apply( Operation::Add { kind, segment : Segment { from : point( from.0, from.1 ), to : point( to.0, to.1 ) }, mirrored : false } ).unwrap(); }

	document
}

fn stacked( count : i32, kind : Kind, x : i32 ) -> Document
{
	let mut document = layer( &( 0 .. count ).map( | row | ( kind, ( -1, row ), ( 0, row ) ) ).collect::<Vec<_>>() );

	document.apply( Operation::Describe { name : String::new(), drawing : Vec::new(), layout : vec![ point( x, 0 ); count as usize ] } ).unwrap();

	document
}

fn outputs( count : i32 ) -> Document
{
	stacked( count, Kind::Output, 2 )
}

fn inputs( count : i32 ) -> Document
{
	stacked( count, Kind::Input { high : false }, -2 )
}

fn symbol( layer : u32, x : i32 ) -> Part
{
	( Kind::Symbol { layer, rotation : 0 }, ( x, 0 ), ( x, 0 ) )
}


#[test]
fn it_finds_no_bus_in_a_drawing_without_a_stack_of_pins()
{
	let layers = [ layer( &[ symbol( 1, 0 ), ( Kind::Wire, ( 2, 0 ), ( 8, 0 ) ) ] ), outputs( 1 ) ];
	let buses = Buses::of( &layers, 0 ).unwrap();

	assert_eq!( buses, Buses::default() );
	assert_eq!( buses.width( point( 2, 0 ) ), 1 );
}

#[test]
fn it_gives_each_point_of_the_wires_from_a_stack_the_width_of_the_stack()
{
	let layers = [ layer( &[ symbol( 1, 0 ), symbol( 2, 10 ), ( Kind::Wire, ( 2, 0 ), ( 4, 0 ) ), ( Kind::Wire, ( 4, 0 ), ( 4, 3 ) ), ( Kind::Wire, ( 4, 3 ), ( 8, 3 ) ), ( Kind::Wire, ( 8, 3 ), ( 8, 0 ) ), ( Kind::Wire, ( 20, 0 ), ( 22, 0 ) ) ] ), outputs( 3 ), inputs( 3 ) ];
	let buses = Buses::of( &layers, 0 ).unwrap();

	assert_eq!( [ ( 2, 0 ), ( 4, 0 ), ( 4, 3 ), ( 8, 3 ), ( 8, 0 ) ].map( | ( x, y ) | buses.width( point( x, y ) ) ), [ 3; 5 ] );
	assert_eq!( [ buses.width( point( 20, 0 ) ), buses.width( point( 22, 0 ) ) ], [ 1, 1 ] );
}

#[test]
fn it_joins_two_stacks_that_touch_without_a_wire()
{
	let layers = [ layer( &[ symbol( 1, 0 ), symbol( 2, 4 ) ] ), outputs( 7 ), inputs( 7 ) ];

	assert_eq!( Buses::of( &layers, 0 ).unwrap().width( point( 2, 0 ) ), 7 );
}

#[test]
fn it_refuses_a_bus_between_stacks_of_two_widths()
{
	let layers = [ layer( &[ symbol( 1, 0 ), symbol( 2, 10 ), ( Kind::Wire, ( 2, 0 ), ( 8, 0 ) ) ] ), outputs( 8 ), inputs( 4 ) ];

	assert_eq!( Buses::of( &layers, 0 ), Err( Error::Bus ) );
}

#[test]
fn it_refuses_a_bus_that_touches_a_part_other_than_a_stack()
{
	let resistor = layer( &[ symbol( 1, 0 ), ( Kind::Wire, ( 2, 0 ), ( 6, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 6, 0 ), ( 10, 0 ) ) ] );
	let single = layer( &[ symbol( 1, 0 ), symbol( 2, 10 ), ( Kind::Wire, ( 2, 0 ), ( 8, 0 ) ) ] );

	assert_eq!( Buses::of( &[ resistor, outputs( 2 ) ], 0 ), Err( Error::Bus ) );
	assert_eq!( Buses::of( &[ single, outputs( 2 ), inputs( 1 ) ], 0 ), Err( Error::Bus ) );
}
