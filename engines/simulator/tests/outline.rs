use simulator::{ Document, Kind, Operation, Outline, Point, Segment, Shape };


fn point( x : i32, y : i32 ) -> Point
{
	Point { x, y }
}

fn layer( kinds : &[ Kind ], drawing : Vec<Shape>, layout : Vec<Point> ) -> Document
{
	let mut document = Document::default();

	for ( index, &kind ) in kinds.iter().enumerate() { document.apply( Operation::Add { kind, segment : Segment { from : point( 0, 2 * index as i32 ), to : point( 1, 2 * index as i32 ) }, mirrored : false } ).unwrap(); }

	document.apply( Operation::Describe { name : String::new(), drawing, layout } ).unwrap();

	document
}


#[test]
fn it_puts_the_inputs_in_a_left_column_and_the_outputs_in_a_right_column_in_their_order()
{
	let gate = layer( &[ Kind::Input { high : false }, Kind::Output, Kind::Wire, Kind::Input { high : true } ], vec![], vec![] );

	assert_eq!( Outline::of( &gate ), Outline { low : point( -2, -2 ), high : point( 2, 2 ), pins : vec![ point( -2, -1 ), point( 2, 0 ), point( -2, 1 ) ], lights : vec![] } );
}

#[test]
fn it_places_the_pins_where_the_layout_puts_them_and_the_others_in_their_column()
{
	let gate = layer( &[ Kind::Input { high : false }, Kind::Output, Kind::Output ], vec![], vec![ point( -4, 3 ) ] );

	assert_eq!( Outline::of( &gate ), Outline { low : point( -4, -2 ), high : point( 2, 3 ), pins : vec![ point( -4, 3 ), point( 2, -1 ), point( 2, 1 ) ], lights : vec![] } );
}

#[test]
fn it_puts_the_leds_in_a_middle_column_and_takes_their_places_from_the_layout_after_the_pins()
{
	let display = layer( &[ Kind::Led { color : 0 }, Kind::Input { high : false }, Kind::Led { color : 2 }, Kind::Led { color : 1 } ], vec![], vec![ point( -3, 0 ), point( 4, 5 ) ] );

	assert_eq!( Outline::of( &display ), Outline { low : point( -3, -2 ), high : point( 4, 5 ), pins : vec![ point( -3, 0 ) ], lights : vec![ point( 4, 5 ), point( 0, 0 ), point( 0, 2 ) ] } );
}

#[test]
fn it_bounds_the_pins_and_the_drawing_in_whole_grid_steps_without_a_box_around_the_center()
{
	let drawing = vec![ Shape::Circle { center : point( 10, 0 ), radius : 3 }, Shape::Line { from : point( -9, 1 ), to : point( 0, -13 ) } ];

	assert_eq!( Outline::of( &layer( &[], drawing, vec![] ) ), Outline { low : point( -3, -4 ), high : point( 4, 1 ), pins : vec![], lights : vec![] } );
}

#[test]
fn it_turns_the_pins_around_the_center_by_quarter_turns()
{
	let outline = Outline { low : point( -2, -2 ), high : point( 2, 2 ), pins : vec![ point( -2, 0 ), point( 2, 1 ) ], lights : vec![] };
	let center = point( 10, 10 );

	assert_eq!( outline.terminals( center, 0 ), vec![ point( 8, 10 ), point( 12, 11 ) ] );
	assert_eq!( outline.terminals( center, 1 ), vec![ point( 10, 8 ), point( 9, 12 ) ] );
	assert_eq!( outline.terminals( center, 2 ), vec![ point( 12, 10 ), point( 8, 9 ) ] );
	assert_eq!( outline.terminals( center, 3 ), vec![ point( 10, 12 ), point( 11, 8 ) ] );
}

#[test]
fn it_gives_the_inside_of_its_bounds_one_step_from_each_side_after_a_turn()
{
	let outline = Outline { low : point( -2, -4 ), high : point( 3, 4 ), pins : vec![], lights : vec![] };

	assert_eq!( outline.zone( point( 10, 10 ), 0 ), Segment { from : point( 9, 7 ), to : point( 12, 13 ) } );
	assert_eq!( outline.zone( point( 10, 10 ), 3 ), Segment { from : point( 7, 8 ), to : point( 13, 11 ) } );
}

#[test]
fn it_gives_the_inside_of_its_bounds_without_its_center_when_the_bounds_are_away_from_it()
{
	let outline = Outline { low : point( -8, -7 ), high : point( -3, -2 ), pins : vec![], lights : vec![] };

	assert_eq!( outline.zone( point( 10, 10 ), 0 ), Segment { from : point( 3, 4 ), to : point( 6, 7 ) } );
}

#[test]
fn it_gives_the_middle_of_its_bounds_rounded_down()
{
	let outline = Outline { low : point( -8, -7 ), high : point( -3, -2 ), pins : vec![], lights : vec![] };

	assert_eq!( outline.middle(), point( -6, -5 ) );
}
