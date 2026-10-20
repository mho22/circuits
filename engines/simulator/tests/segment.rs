use simulator::{ Point, Segment };


const ORIGIN : Point = Point { x : 0, y : 0 };


fn segment( x : i32, y : i32 ) -> Segment
{
	Segment { from : ORIGIN, to : Point { x, y } }
}


#[test]
fn it_accepts_horizontal_vertical_and_diagonal_segments()
{
	assert!( segment( 5, 0 ).is_straight() );
	assert!( segment( 0, -5 ).is_straight() );
	assert!( segment( -3, 3 ).is_straight() );
	assert!( segment( 0, 0 ).is_straight() );
}

#[test]
fn it_rejects_other_angles()
{
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
	assert_eq!( segment( 3, 3 ).length(), 3 );
	assert_eq!( segment( 0, 0 ).length(), 0 );
}

#[test]
fn it_snaps_to_the_nearest_axis()
{
	assert_eq!( Segment::snap( ORIGIN, Point { x : 6, y : 1 } ), Point { x : 6, y : 0 } );
	assert_eq!( Segment::snap( ORIGIN, Point { x : -1, y : -6 } ), Point { x : 0, y : -6 } );
}

#[test]
fn it_snaps_to_the_nearest_diagonal()
{
	assert_eq!( Segment::snap( ORIGIN, Point { x : 4, y : 5 } ), Point { x : 5, y : 5 } );
	assert_eq!( Segment::snap( ORIGIN, Point { x : -4, y : 3 } ), Point { x : -4, y : 4 } );
}

#[test]
fn it_prefers_the_axis_when_a_diagonal_is_as_near()
{
	assert_eq!( Segment::snap( ORIGIN, Point { x : 2, y : 1 } ), Point { x : 2, y : 0 } );
	assert_eq!( Segment::snap( ORIGIN, Point { x : 1, y : 2 } ), Point { x : 0, y : 2 } );
}

#[test]
fn it_keeps_a_cursor_that_is_already_straight()
{
	assert_eq!( Segment::snap( ORIGIN, Point { x : 3, y : -3 } ), Point { x : 3, y : -3 } );
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
