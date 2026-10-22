use simulator::{ stretch, Element, Kind, Operation, Point, Segment };


const RESISTOR : Kind = Kind::Resistor { ohms : 1000.0 };


fn segment( from : ( i32, i32 ), to : ( i32, i32 ) ) -> Segment
{
	Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }
}

fn element( id : u32, kind : Kind, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { id, kind, segment : segment( from, to ), mirrored : false }
}

fn piece( from : ( i32, i32 ), to : ( i32, i32 ) ) -> Operation
{
	Operation::Add { kind : Kind::Wire, segment : segment( from, to ), mirrored : false }
}

fn moved( part : Element, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { segment : segment( from, to ), ..part }
}

fn stretched( elements : &[ Element ], before : &Element, after : &Element ) -> Vec<Operation>
{
	let others : Vec<Element> = elements.iter().filter( | element | element.id != before.id ).copied().collect();

	stretch( &others, &before.terminals(), &after.terminals(), after.segment, Element::terminals )
}


#[test]
fn it_leaves_the_wires_that_do_not_touch_the_part()
{
	let part = element( 0, RESISTOR, ( 0, 0 ), ( 2, 0 ) );
	let elements = [ part, element( 1, Kind::Wire, ( 5, 5 ), ( 7, 5 ) ) ];

	assert_eq!( stretched( &elements, &part, &moved( part, ( 0, 2 ), ( 2, 2 ) ) ), vec![] );
}

#[test]
fn it_stretches_a_wire_along_its_axis()
{
	let part = element( 0, RESISTOR, ( 2, 0 ), ( 4, 0 ) );
	let elements = [ part, element( 1, Kind::Wire, ( 0, 0 ), ( 2, 0 ) ) ];

	assert_eq!( stretched( &elements, &part, &moved( part, ( 3, 0 ), ( 5, 0 ) ) ), vec![ Operation::Delete { id : 1 }, piece( ( 0, 0 ), ( 3, 0 ) ) ] );
}

#[test]
fn it_bends_a_wire_and_keeps_its_axis_at_the_end_that_stays()
{
	let part = element( 0, RESISTOR, ( 2, 0 ), ( 4, 0 ) );
	let target = moved( part, ( 2, 2 ), ( 4, 2 ) );

	assert_eq!( stretched( &[ part, element( 1, Kind::Wire, ( 0, 0 ), ( 2, 0 ) ) ], &part, &target ), vec![ Operation::Delete { id : 1 }, piece( ( 0, 0 ), ( 2, 0 ) ), piece( ( 2, 0 ), ( 2, 2 ) ) ] );
	assert_eq!( stretched( &[ part, element( 1, Kind::Wire, ( 2, 0 ), ( 0, 0 ) ) ], &part, &target ), vec![ Operation::Delete { id : 1 }, piece( ( 2, 2 ), ( 2, 0 ) ), piece( ( 2, 0 ), ( 0, 0 ) ) ] );
}

#[test]
fn it_bends_a_wire_around_a_turned_part()
{
	let part = element( 0, RESISTOR, ( 0, 0 ), ( 4, 0 ) );
	let elements = [ part, element( 1, Kind::Wire, ( -3, 0 ), ( 0, 0 ) ), element( 2, Kind::Wire, ( 4, 0 ), ( 7, 0 ) ) ];

	assert_eq!( stretched( &elements, &part, &moved( part, ( 2, -2 ), ( 2, 2 ) ) ), vec![
		Operation::Delete { id : 1 },
		Operation::Delete { id : 2 },
		piece( ( -3, 0 ), ( -3, -2 ) ),
		piece( ( -3, -2 ), ( 2, -2 ) ),
		piece( ( 2, 2 ), ( 7, 2 ) ),
		piece( ( 7, 2 ), ( 7, 0 ) ),
	] );
}

#[test]
fn it_adds_no_wire_to_a_wire_that_the_part_touched_in_its_middle()
{
	let part = element( 0, RESISTOR, ( 0, 0 ), ( 0, 2 ) );
	let elements = [ part, element( 1, Kind::Wire, ( -2, 0 ), ( 2, 0 ) ) ];

	assert!( stretched( &elements, &part, &moved( part, ( 0, 4 ), ( 0, 6 ) ) ).is_empty() );
}

#[test]
fn it_adds_a_wire_to_a_part_that_the_part_touched()
{
	let part = element( 0, RESISTOR, ( 0, 0 ), ( 2, 0 ) );
	let elements = [ part, element( 1, Kind::Battery { volts : 5.0 }, ( 2, 0 ), ( 4, 0 ) ) ];

	assert_eq!( stretched( &elements, &part, &moved( part, ( 0, 2 ), ( 2, 2 ) ) ), vec![ piece( ( 2, 0 ), ( 2, 2 ) ) ] );
}

#[test]
fn it_follows_the_collector_of_a_mirrored_transistor()
{
	let part = element( 0, Kind::Npn, ( 0, 0 ), ( 2, 0 ) );
	let elements = [ part, element( 1, Kind::Wire, ( 3, -1 ), ( 3, -3 ) ) ];

	assert_eq!( stretched( &elements, &part, &Element { mirrored : true, ..part } ), vec![ Operation::Delete { id : 1 }, piece( ( 3, 1 ), ( 3, -3 ) ) ] );
}

#[test]
fn it_adds_no_wire_twice_for_a_moved_ground()
{
	let part = element( 0, Kind::Ground, ( 0, 0 ), ( 0, 0 ) );
	let elements = [ part, element( 1, Kind::Wire, ( 0, 0 ), ( 2, 0 ) ), element( 2, RESISTOR, ( 0, 0 ), ( 0, -2 ) ) ];

	assert_eq!( stretched( &elements, &part, &moved( part, ( 0, 3 ), ( 0, 3 ) ) ), vec![ Operation::Delete { id : 1 }, piece( ( 0, 3 ), ( 0, 0 ) ), piece( ( 0, 0 ), ( 2, 0 ) ) ] );
}

#[test]
fn it_leads_two_wires_around_a_box_on_two_lanes_to_the_pins_across_it()
{
	let elements = [ element( 0, Kind::Wire, ( 0, 0 ), ( 2, 0 ) ), element( 1, Kind::Wire, ( 10, 0 ), ( 8, 0 ) ) ];
	let old = [ Point { x : 2, y : 0 }, Point { x : 8, y : 0 } ];
	let new = [ Point { x : 7, y : 0 }, Point { x : 3, y : 0 } ];

	assert_eq!( stretch( &elements, &old, &new, segment( ( 4, -1 ), ( 6, 1 ) ), Element::terminals ), vec![
		Operation::Delete { id : 0 },
		Operation::Delete { id : 1 },
		piece( ( 0, 0 ), ( 0, -3 ) ),
		piece( ( 0, -3 ), ( 8, -3 ) ),
		piece( ( 8, -3 ), ( 8, 0 ) ),
		piece( ( 8, 0 ), ( 7, 0 ) ),
		piece( ( 10, 0 ), ( 10, 3 ) ),
		piece( ( 10, 3 ), ( 2, 3 ) ),
		piece( ( 2, 3 ), ( 2, 0 ) ),
		piece( ( 2, 0 ), ( 3, 0 ) ),
	] );
}

#[test]
fn it_keeps_a_straight_wire_to_a_terminal_inside_the_zone()
{
	let elements = [ element( 0, Kind::Wire, ( 0, 0 ), ( 2, 0 ) ) ];

	assert_eq!( stretch( &elements, &[ Point { x : 2, y : 0 } ], &[ Point { x : 5, y : 0 } ], segment( ( 3, -1 ), ( 6, 1 ) ), Element::terminals ), vec![ Operation::Delete { id : 0 }, piece( ( 0, 0 ), ( 5, 0 ) ) ] );
}
