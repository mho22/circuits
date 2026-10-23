use simulator::{ Element, Kind, Point, Segment, Topology };


fn element( kind : Kind, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { id : 0, kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }, mirrored : false }
}

fn point( x : i32, y : i32 ) -> Point
{
	Point { x, y }
}


#[test]
fn it_numbers_each_net_but_the_grounded_one_as_an_unknown()
{
	let topology = Topology::build( &[
		element( Kind::Battery { volts : 5.0 }, ( 0, 4 ), ( 0, 0 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 0, 4 ) ),
		element( Kind::Ground, ( 0, 4 ), ( 0, 4 ) ),
	] );
	let net = | x, y | topology.netlist.net( point( x, y ) );

	assert_eq!( topology.unknowns[ net( 0, 4 ) ], None );
	assert!( topology.unknowns[ net( 0, 0 ) ].is_some() && topology.unknowns[ net( 4, 0 ) ].is_some() );
	assert_eq!( ( topology.nodes, topology.batteries.clone() ), ( 2, vec![ 0 ] ) );
	assert_eq!( topology.terminals[ 0 ], vec![ net( 0, 4 ), net( 0, 0 ) ] );
}

#[test]
fn it_ties_a_supply_to_the_ground_or_to_an_earth_of_its_own()
{
	let grounded = Topology::build( &[ element( Kind::Supply { volts : 5.0 }, ( 2, 0 ), ( 2, 0 ) ), element( Kind::Resistor { ohms : 1000.0 }, ( 2, 0 ), ( 2, 4 ) ), element( Kind::Ground, ( 2, 4 ), ( 2, 4 ) ) ] );
	let floating = Topology::build( &[ element( Kind::Supply { volts : 5.0 }, ( 2, 0 ), ( 2, 0 ) ), element( Kind::Resistor { ohms : 1000.0 }, ( 2, 0 ), ( 2, 4 ) ) ] );

	assert_eq!( grounded.terminals[ 0 ], vec![ grounded.netlist.net( point( 2, 4 ) ), grounded.netlist.net( point( 2, 0 ) ) ] );
	assert_eq!( floating.terminals[ 0 ], vec![ 2, floating.netlist.net( point( 2, 0 ) ) ] );
	assert_eq!( floating.unknowns[ 2 ], None );
}

#[test]
fn it_references_a_floating_part_to_its_first_net()
{
	let topology = Topology::build( &[ element( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 2, 0 ) ) ] );

	assert_eq!( topology.unknowns, vec![ None, Some( 0 ) ] );
	assert_eq!( topology.nodes, 1 );
}
