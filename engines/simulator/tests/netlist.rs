use simulator::{ Element, Kind, Netlist, Point, Segment };


fn element( kind : Kind, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { id : 0, kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } } }
}

fn point( x : i32, y : i32 ) -> Point
{
	Point { x, y }
}


#[test]
fn it_joins_wires_that_share_an_end()
{
	let netlist = Netlist::build( &[ element( Kind::Wire, ( 0, 0 ), ( 2, 0 ) ), element( Kind::Wire, ( 2, 0 ), ( 2, 2 ) ) ] );

	assert_eq!( netlist.count, 1 );
	assert_eq!( netlist.edges.len(), 2 );
}

#[test]
fn it_joins_an_end_that_touches_the_middle_of_a_wire()
{
	let netlist = Netlist::build( &[ element( Kind::Wire, ( 0, 0 ), ( 4, 0 ) ), element( Kind::Wire, ( 2, 0 ), ( 2, 3 ) ) ] );

	assert_eq!( netlist.count, 1 );
	assert_eq!( netlist.edges, vec![ ( 0, 2 ), ( 2, 1 ), ( 2, 3 ) ] );
}

#[test]
fn it_keeps_crossing_wires_apart()
{
	let netlist = Netlist::build( &[ element( Kind::Wire, ( 0, 2 ), ( 4, 2 ) ), element( Kind::Wire, ( 2, 0 ), ( 2, 4 ) ) ] );

	assert_eq!( netlist.count, 2 );
	assert_ne!( netlist.net( point( 0, 2 ) ), netlist.net( point( 2, 0 ) ) );
}

#[test]
fn it_keeps_the_two_ends_of_a_part_apart()
{
	let netlist = Netlist::build( &[ element( Kind::Resistor { ohms : 1.0 }, ( 0, 0 ), ( 2, 0 ) ) ] );

	assert_eq!( netlist.count, 2 );
	assert!( netlist.edges.is_empty() );
}

#[test]
fn it_joins_a_part_end_on_the_middle_of_a_wire()
{
	let netlist = Netlist::build( &[ element( Kind::Wire, ( 0, 0 ), ( 4, 0 ) ), element( Kind::Battery { volts : 1.0 }, ( 2, 0 ), ( 2, 2 ) ) ] );

	assert_eq!( netlist.net( point( 0, 0 ) ), netlist.net( point( 2, 0 ) ) );
	assert_ne!( netlist.net( point( 2, 0 ) ), netlist.net( point( 2, 2 ) ) );
}

#[test]
fn it_joins_every_ground_into_one_earth()
{
	let netlist = Netlist::build( &[ element( Kind::Ground, ( 0, 0 ), ( 0, 0 ) ), element( Kind::Ground, ( 5, 5 ), ( 5, 5 ) ), element( Kind::Wire, ( 9, 9 ), ( 9, 8 ) ) ] );

	assert_eq!( netlist.count, 2 );
	assert_eq!( netlist.earth, Some( netlist.net( point( 5, 5 ) ) ) );
	assert_eq!( netlist.net( point( 0, 0 ) ), netlist.net( point( 5, 5 ) ) );
}

#[test]
fn it_has_no_earth_without_a_ground()
{
	let netlist = Netlist::build( &[ element( Kind::Wire, ( 0, 0 ), ( 1, 0 ) ) ] );

	assert_eq!( netlist.earth, None );
	assert_eq!( netlist.index( point( 1, 0 ) ), 1 );
	assert_eq!( netlist.points, vec![ point( 0, 0 ), point( 1, 0 ) ] );
	assert_eq!( netlist.nets, vec![ 0, 0 ] );
}
