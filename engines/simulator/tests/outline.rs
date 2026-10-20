use simulator::{ Element, Kind, Outline, Point, Segment };


fn pin( id : u32, x : i32, y : i32 ) -> Element
{
	Element { id, kind : Kind::Pin, segment : Segment { from : Point { x, y }, to : Point { x, y } }, mirrored : false }
}


#[test]
fn it_puts_each_pin_on_the_nearest_side_of_the_box()
{
	let inverter = [ pin( 0, 0, 2 ), pin( 1, 8, 2 ), pin( 2, 4, 0 ) ];

	assert_eq!( Outline::of( &inverter ), Outline { width : 4, height : 4, pins : vec![ Point { x : -2, y : 0 }, Point { x : 2, y : 0 }, Point { x : 0, y : -2 } ] } );
}

#[test]
fn it_spreads_the_pins_of_one_side_in_their_order_along_it()
{
	let gate = [ pin( 0, 8, 3 ), pin( 1, 0, 4 ), pin( 2, 0, 0 ), pin( 3, 4, 7 ) ];

	assert_eq!( Outline::of( &gate ), Outline { width : 4, height : 6, pins : vec![ Point { x : 2, y : 0 }, Point { x : -2, y : 1 }, Point { x : -2, y : -1 }, Point { x : 0, y : 3 } ] } );
}

#[test]
fn it_keeps_the_order_of_the_parts_for_pins_level_on_one_side()
{
	let pins = [ pin( 0, 0, 2 ), pin( 1, 1, 2 ), pin( 2, 9, 0 ), pin( 3, 9, 4 ) ];

	assert_eq!( Outline::of( &pins ).pins, vec![ Point { x : -2, y : -1 }, Point { x : -2, y : 1 }, Point { x : 2, y : -1 }, Point { x : 2, y : 1 } ] );
}

#[test]
fn it_draws_a_square_box_without_pins()
{
	let wire = Element { id : 0, kind : Kind::Wire, segment : Segment { from : Point { x : 0, y : 0 }, to : Point { x : 2, y : 0 } }, mirrored : false };

	assert_eq!( Outline::of( &[ wire ] ), Outline { width : 4, height : 4, pins : vec![] } );
}

#[test]
fn it_turns_the_pins_around_the_center_by_quarter_turns()
{
	let outline = Outline::of( &[ pin( 0, 0, 2 ), pin( 1, 8, 2 ), pin( 2, 4, 0 ) ] );
	let center = Point { x : 10, y : 10 };

	assert_eq!( outline.terminals( center, 0 ), vec![ Point { x : 8, y : 10 }, Point { x : 12, y : 10 }, Point { x : 10, y : 8 } ] );
	assert_eq!( outline.terminals( center, 1 ), vec![ Point { x : 10, y : 8 }, Point { x : 10, y : 12 }, Point { x : 12, y : 10 } ] );
	assert_eq!( outline.terminals( center, 2 ), vec![ Point { x : 12, y : 10 }, Point { x : 8, y : 10 }, Point { x : 10, y : 12 } ] );
	assert_eq!( outline.terminals( center, 3 ), vec![ Point { x : 10, y : 12 }, Point { x : 10, y : 8 }, Point { x : 8, y : 10 } ] );
}
