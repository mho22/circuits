use simulator::{ place, Element, Kind, Operation, Point, Segment };


const RESISTOR : Kind = Kind::Resistor { ohms : 1000.0 };


fn segment( from : ( i32, i32 ), to : ( i32, i32 ) ) -> Segment
{
	Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }
}

fn wire( id : u32, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { id, kind : Kind::Wire, segment : segment( from, to ), mirrored : false }
}

fn piece( from : ( i32, i32 ), to : ( i32, i32 ) ) -> Operation
{
	Operation::Add { kind : Kind::Wire, segment : segment( from, to ), mirrored : false }
}


#[test]
fn it_splits_a_wire_under_a_part_in_its_middle()
{
	assert_eq!( place( &[ wire( 3, ( 0, 0 ), ( 8, 0 ) ) ], RESISTOR, segment( ( 2, 0 ), ( 5, 0 ) ) ), vec![
		Operation::Delete { id : 3 },
		piece( ( 0, 0 ), ( 2, 0 ) ),
		piece( ( 5, 0 ), ( 8, 0 ) ),
		Operation::Add { kind : RESISTOR, segment : segment( ( 2, 0 ), ( 5, 0 ) ), mirrored : false },
	] );
}

#[test]
fn it_splits_a_wire_under_a_part_drawn_the_other_way()
{
	assert_eq!( place( &[ wire( 3, ( 8, 0 ), ( 0, 0 ) ) ], RESISTOR, segment( ( 2, 0 ), ( 5, 0 ) ) )[ 1 .. 3 ], [ piece( ( 8, 0 ), ( 5, 0 ) ), piece( ( 2, 0 ), ( 0, 0 ) ) ] );
}

#[test]
fn it_keeps_one_piece_when_the_part_starts_at_the_end_of_the_wire()
{
	assert_eq!( place( &[ wire( 3, ( 0, 0 ), ( 8, 0 ) ) ], RESISTOR, segment( ( 0, 0 ), ( 3, 0 ) ) ).len(), 3 );
	assert_eq!( place( &[ wire( 3, ( 0, 0 ), ( 8, 0 ) ) ], RESISTOR, segment( ( -2, 0 ), ( 3, 0 ) ) )[ 1 ], piece( ( 3, 0 ), ( 8, 0 ) ) );
}

#[test]
fn it_deletes_a_wire_that_the_part_covers()
{
	assert_eq!( place( &[ wire( 3, ( 2, 0 ), ( 4, 0 ) ) ], RESISTOR, segment( ( 0, 0 ), ( 6, 0 ) ) ), vec![
		Operation::Delete { id : 3 },
		Operation::Add { kind : RESISTOR, segment : segment( ( 0, 0 ), ( 6, 0 ) ), mirrored : false },
	] );
}

#[test]
fn it_cuts_every_wire_under_the_part()
{
	let operations = place( &[ wire( 3, ( 0, 0 ), ( 4, 0 ) ), wire( 5, ( 4, 0 ), ( 8, 0 ) ) ], RESISTOR, segment( ( 2, 0 ), ( 6, 0 ) ) );

	assert_eq!( operations, vec![
		Operation::Delete { id : 3 },
		piece( ( 0, 0 ), ( 2, 0 ) ),
		Operation::Delete { id : 5 },
		piece( ( 6, 0 ), ( 8, 0 ) ),
		Operation::Add { kind : RESISTOR, segment : segment( ( 2, 0 ), ( 6, 0 ) ), mirrored : false },
	] );
}

#[test]
fn it_leaves_a_wire_that_the_part_does_not_run_along()
{
	let wires = [ wire( 3, ( 0, 0 ), ( 8, 0 ) ) ];
	let alone = | from, to | place( &wires, RESISTOR, segment( from, to ) ).len() == 1;

	assert!( alone( ( 2, 1 ), ( 5, 1 ) ) );
	assert!( alone( ( 4, -2 ), ( 4, 2 ) ) );
	assert!( alone( ( 8, 0 ), ( 10, 0 ) ) );
	assert!( alone( ( 9, 0 ), ( 12, 0 ) ) );
}

#[test]
fn it_never_splits_a_wire_for_a_wire_or_a_ground()
{
	let wires = [ wire( 3, ( 0, 0 ), ( 8, 0 ) ) ];

	assert_eq!( place( &wires, Kind::Wire, segment( ( 2, 0 ), ( 5, 0 ) ) ), vec![ piece( ( 2, 0 ), ( 5, 0 ) ) ] );
	assert_eq!( place( &wires, Kind::Ground, segment( ( 2, 0 ), ( 2, 0 ) ) ).len(), 1 );
}

#[test]
fn it_splits_a_wire_for_each_kind_of_two_ended_part()
{
	let wires = [ wire( 3, ( 0, 0 ), ( 8, 0 ) ) ];

	for kind in [ Kind::Battery { volts : 5.0 }, Kind::Capacitor { farads : 1e-6 }, Kind::Inductor { henries : 1.0 }, Kind::Switch { closed : false } ]
	{
		assert_eq!( place( &wires, kind, segment( ( 2, 0 ), ( 5, 0 ) ) ).len(), 4 );
	}
}
