use simulator::{ Point, Wire };


const ORIGIN : Point = Point { x : 0, y : 0 };


#[test]
fn it_accepts_horizontal_vertical_and_diagonal_wires()
{
	assert!( Wire::is_straight( ORIGIN, Point { x : 5, y : 0 } ) );
	assert!( Wire::is_straight( ORIGIN, Point { x : 0, y : -5 } ) );
	assert!( Wire::is_straight( ORIGIN, Point { x : -3, y : 3 } ) );
}

#[test]
fn it_rejects_other_angles()
{
	assert!( ! Wire::is_straight( ORIGIN, Point { x : 2, y : 1 } ) );
}

#[test]
fn it_snaps_to_the_nearest_axis()
{
	assert_eq!( Wire::snap( ORIGIN, Point { x : 6, y : 1 } ), Point { x : 6, y : 0 } );
	assert_eq!( Wire::snap( ORIGIN, Point { x : -1, y : -6 } ), Point { x : 0, y : -6 } );
}

#[test]
fn it_snaps_to_the_nearest_diagonal()
{
	assert_eq!( Wire::snap( ORIGIN, Point { x : 4, y : 5 } ), Point { x : 5, y : 5 } );
	assert_eq!( Wire::snap( ORIGIN, Point { x : -4, y : 3 } ), Point { x : -4, y : 4 } );
}

#[test]
fn it_prefers_the_axis_when_a_diagonal_is_as_near()
{
	assert_eq!( Wire::snap( ORIGIN, Point { x : 2, y : 1 } ), Point { x : 2, y : 0 } );
	assert_eq!( Wire::snap( ORIGIN, Point { x : 1, y : 2 } ), Point { x : 0, y : 2 } );
}

#[test]
fn it_keeps_a_cursor_that_is_already_straight()
{
	assert_eq!( Wire::snap( ORIGIN, Point { x : 3, y : -3 } ), Point { x : 3, y : -3 } );
	assert_eq!( Wire::snap( ORIGIN, ORIGIN ), ORIGIN );
}

#[test]
fn it_contains_the_points_on_its_segment()
{
	let wire = Wire { id : 0, from : ORIGIN, to : Point { x : 4, y : 4 } };

	assert!( wire.contains( ORIGIN ) );
	assert!( wire.contains( Point { x : 2, y : 2 } ) );
	assert!( wire.contains( Point { x : 4, y : 4 } ) );
}

#[test]
fn it_does_not_contain_points_off_its_segment()
{
	let wire = Wire { id : 0, from : ORIGIN, to : Point { x : 4, y : 4 } };

	assert!( ! wire.contains( Point { x : 2, y : 3 } ) );
	assert!( ! wire.contains( Point { x : 5, y : 5 } ) );
	assert!( ! wire.contains( Point { x : -1, y : -1 } ) );
}
