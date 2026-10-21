use simulator::{ enclose, Element, Enclosure, Error, Kind, Operation, Point, Segment };


const RESISTOR : Kind = Kind::Resistor { ohms : 1000.0 };
const LOW : Point = Point { x : 2, y : -2 };
const HIGH : Point = Point { x : 8, y : 2 };


fn point( x : i32, y : i32 ) -> Point
{
	Point { x, y }
}

fn element( id : u32, kind : Kind, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { id, kind, segment : Segment { from : point( from.0, from.1 ), to : point( to.0, to.1 ) }, mirrored : false }
}


#[test]
fn it_moves_the_parts_inside_and_gives_the_end_of_the_lead_of_each_input_and_output()
{
	let elements = [ element( 0, Kind::Input { high : true }, ( 3, 0 ), ( 4, 0 ) ), element( 1, RESISTOR, ( 4, 0 ), ( 6, 0 ) ), element( 2, Kind::Output, ( 7, 0 ), ( 6, 0 ) ), element( 3, Kind::Wire, ( 6, 0 ), ( 12, 0 ) ) ];

	assert_eq!( enclose( &elements, LOW, HIGH, Element::terminals ), Ok( Enclosure {
		inner : elements[ .. 3 ].to_vec(),
		outside : vec![ Operation::Delete { id : 0 }, Operation::Delete { id : 1 }, Operation::Delete { id : 2 } ],
		pins : vec![ point( 4, 0 ), point( 6, 0 ) ],
	} ) );
}

#[test]
fn it_leaves_a_wire_across_the_edge_whole_outside()
{
	let elements = [ element( 0, Kind::Wire, ( 0, 0 ), ( 10, 0 ) ), element( 1, RESISTOR, ( 4, 1 ), ( 6, 1 ) ) ];
	let enclosure = enclose( &elements, LOW, HIGH, Element::terminals ).unwrap();

	assert_eq!( ( enclosure.inner, enclosure.outside ), ( vec![ elements[ 1 ] ], vec![ Operation::Delete { id : 1 } ] ) );
}

#[test]
fn it_keeps_a_vcc_and_a_ground_inside_without_a_pin()
{
	let elements = [ element( 0, Kind::Supply { volts : 5.0 }, ( 4, -1 ), ( 4, -1 ) ), element( 1, RESISTOR, ( 4, -1 ), ( 4, 1 ) ), element( 2, Kind::Ground, ( 4, 1 ), ( 4, 1 ) ) ];
	let enclosure = enclose( &elements, LOW, HIGH, Element::terminals ).unwrap();

	assert_eq!( ( enclosure.inner.len(), enclosure.pins ), ( 3, vec![] ) );
}

#[test]
fn it_rejects_a_part_across_the_edge()
{
	let elements = [ element( 0, RESISTOR, ( 0, 0 ), ( 4, 0 ) ) ];

	assert_eq!( enclose( &elements, LOW, HIGH, Element::terminals ), Err( Error::Straddled ) );
}

#[test]
fn it_rejects_a_selection_without_a_part()
{
	let elements = [ element( 0, Kind::Wire, ( 0, 5 ), ( 10, 5 ) ), element( 1, Kind::Wire, ( 0, 2 ), ( 2, 2 ) ) ];

	assert_eq!( enclose( &elements, LOW, HIGH, Element::terminals ), Err( Error::Unselected ) );
}
