use simulator::{ Point, Segment };


const ORIGIN : Point = Point { x : 0, y : 0 };


fn segment( x : i32, y : i32 ) -> Segment
{
	Segment { from : ORIGIN, to : Point { x, y } }
}


#[test]
fn it_accepts_horizontal_and_vertical_segments()
{
	assert!( segment( 5, 0 ).is_straight() );
	assert!( segment( 0, -5 ).is_straight() );
	assert!( segment( 0, 0 ).is_straight() );
}

#[test]
fn it_rejects_other_angles()
{
	assert!( ! segment( -3, 3 ).is_straight() );
	assert!( ! segment( 2, 1 ).is_straight() );
}

#[test]
fn it_is_empty_when_both_ends_meet()
{
	assert!( segment( 0, 0 ).is_empty() );
	assert!( ! segment( 1, 0 ).is_empty() );
}

#[test]
fn it_measures_its_length_in_grid_steps()
{
	assert_eq!( segment( -4, 0 ).length(), 4 );
	assert_eq!( segment( 0, 3 ).length(), 3 );
	assert_eq!( segment( 0, 0 ).length(), 0 );
}

#[test]
fn it_snaps_to_the_nearest_axis()
{
	assert_eq!( Segment::snap( ORIGIN, Point { x : 6, y : 1 } ), Point { x : 6, y : 0 } );
	assert_eq!( Segment::snap( ORIGIN, Point { x : -1, y : -6 } ), Point { x : 0, y : -6 } );
}

#[test]
fn it_snaps_a_diagonal_cursor_to_the_longer_axis()
{
	assert_eq!( Segment::snap( ORIGIN, Point { x : 4, y : 5 } ), Point { x : 0, y : 5 } );
	assert_eq!( Segment::snap( ORIGIN, Point { x : -4, y : 3 } ), Point { x : -4, y : 0 } );
}

#[test]
fn it_prefers_the_horizontal_axis_when_both_are_as_long()
{
	assert_eq!( Segment::snap( ORIGIN, Point { x : 3, y : -3 } ), Point { x : 3, y : 0 } );
}

#[test]
fn it_keeps_a_cursor_that_is_already_straight()
{
	assert_eq!( Segment::snap( ORIGIN, Point { x : 0, y : -3 } ), Point { x : 0, y : -3 } );
	assert_eq!( Segment::snap( ORIGIN, ORIGIN ), ORIGIN );
}

#[test]
fn it_contains_the_points_on_it()
{
	assert!( segment( 4, 4 ).contains( ORIGIN ) );
	assert!( segment( 4, 4 ).contains( Point { x : 2, y : 2 } ) );
	assert!( segment( 4, 4 ).contains( Point { x : 4, y : 4 } ) );
	assert!( segment( 0, 0 ).contains( ORIGIN ) );
}

#[test]
fn it_does_not_contain_points_off_it()
{
	assert!( ! segment( 4, 4 ).contains( Point { x : 2, y : 3 } ) );
	assert!( ! segment( 4, 4 ).contains( Point { x : 5, y : 5 } ) );
	assert!( ! segment( 4, 4 ).contains( Point { x : -1, y : -1 } ) );
	assert!( ! segment( 0, 0 ).contains( Point { x : 1, y : 0 } ) );
}

#[test]
fn it_turns_a_quarter_clockwise_around_the_point_at_half_its_length()
{
	assert_eq!( segment( 4, 0 ).turned(), Segment { from : Point { x : 2, y : -2 }, to : Point { x : 2, y : 2 } } );
	assert_eq!( segment( 3, 0 ).turned(), Segment { from : Point { x : 1, y : -1 }, to : Point { x : 1, y : 2 } } );
	assert_eq!( segment( 0, 0 ).turned(), segment( 0, 0 ) );
}

#[test]
fn it_comes_back_after_four_turns()
{
	for part in [ segment( 4, 0 ), segment( 3, 0 ), segment( 0, -5 ) ]
	{
		assert_eq!( part.turned().turned().turned().turned(), part );
	}
}

#[test]
fn it_mirrors_across_its_middle()
{
	assert_eq!( segment( 4, 0 ).mirrored( false ), Segment { from : Point { x : 4, y : 0 }, to : ORIGIN } );
	assert_eq!( segment( 4, 0 ).mirrored( true ), segment( 4, 0 ) );
	assert_eq!( segment( 0, 3 ).mirrored( true ), Segment { from : Point { x : 0, y : 3 }, to : ORIGIN } );
	assert_eq!( segment( 0, 3 ).mirrored( false ), segment( 0, 3 ) );
}

#[test]
fn it_shifts_both_ends()
{
	assert_eq!( segment( 4, 0 ).shifted( 2, -1 ), Segment { from : Point { x : 2, y : -1 }, to : Point { x : 6, y : -1 } } );
}
