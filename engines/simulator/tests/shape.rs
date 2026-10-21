use simulator::{ Error, Point, Shape };


const FOO : Point = Point { x : 0, y : 0 };
const BAR : Point = Point { x : 2, y : 3 };


#[test]
fn it_accepts_shapes_with_a_size()
{
	assert_eq!( Shape::Circle { center : FOO, radius : 1 }.check(), Ok( () ) );
	assert_eq!( Shape::Line { from : FOO, to : BAR }.check(), Ok( () ) );
	assert_eq!( Shape::Curve { from : FOO, through : BAR, to : Point { x : 4, y : 0 } }.check(), Ok( () ) );
}

#[test]
fn it_rejects_shapes_without_a_size()
{
	assert_eq!( Shape::Circle { center : FOO, radius : 0 }.check(), Err( Error::Flat ) );
	assert_eq!( Shape::Circle { center : FOO, radius : -1 }.check(), Err( Error::Flat ) );
	assert_eq!( Shape::Line { from : BAR, to : BAR }.check(), Err( Error::Flat ) );
	assert_eq!( Shape::Curve { from : FOO, through : BAR, to : Point { x : 4, y : 6 } }.check(), Err( Error::Flat ) );
	assert_eq!( Shape::Curve { from : FOO, through : BAR, to : FOO }.check(), Err( Error::Flat ) );
}

#[test]
fn it_bounds_each_shape_by_its_lowest_and_highest_corner()
{
	let point = | x, y | Point { x, y };

	assert_eq!( Shape::Circle { center : point( 1, 1 ), radius : 2 }.bounds(), ( point( -1, -1 ), point( 3, 3 ) ) );
	assert_eq!( Shape::Line { from : point( 4, 0 ), to : point( -3, 5 ) }.bounds(), ( point( -3, 0 ), point( 4, 5 ) ) );
}

#[test]
fn it_bounds_a_curve_by_its_ends_and_by_the_sides_of_its_circle_that_it_reaches()
{
	let point = | x, y | Point { x, y };

	assert_eq!( Shape::Curve { from : point( 0, -4 ), through : point( 4, 0 ), to : point( 0, 4 ) }.bounds(), ( point( 0, -4 ), point( 4, 4 ) ) );
	assert_eq!( Shape::Curve { from : point( 0, -4 ), through : point( -4, 0 ), to : point( 0, 4 ) }.bounds(), ( point( -4, -4 ), point( 0, 4 ) ) );
	assert_eq!( Shape::Curve { from : point( 0, -3 ), through : point( -9, 0 ), to : point( 0, 3 ) }.bounds(), ( point( -9, -5 ), point( 0, 5 ) ) );
	assert_eq!( Shape::Curve { from : point( 0, 0 ), through : point( 1, 1 ), to : point( 2, 0 ) }.bounds(), ( point( 0, 0 ), point( 2, 1 ) ) );
}
