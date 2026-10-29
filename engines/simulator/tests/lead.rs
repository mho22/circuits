use simulator::{ leads, Element, Kind, Point, Segment };


fn element( kind : Kind, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { id : 0, kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }, mirrored : false }
}


#[test]
fn it_finds_the_pins_on_the_anode_and_the_cathode_of_each_led_through_wires_and_resistors()
{
	let elements = [
		element( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		element( Kind::Output, ( 9, 0 ), ( 8, 0 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) ),
		element( Kind::Led { color : 0 }, ( 4, 0 ), ( 6, 0 ) ),
		element( Kind::Wire, ( 6, 0 ), ( 8, 0 ) ),
		element( Kind::Led { color : 1 }, ( 8, 0 ), ( 0, 0 ) ),
	];

	assert_eq!( leads( &elements ), vec![ Some( ( 0, 1 ) ), Some( ( 1, 0 ) ) ] );
}

#[test]
fn it_finds_no_pins_for_an_led_with_an_end_on_ground_or_on_an_inner_point()
{
	let elements = [
		element( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		element( Kind::Led { color : 0 }, ( 0, 0 ), ( 0, 4 ) ),
		element( Kind::Ground, ( 0, 4 ), ( 0, 4 ) ),
		element( Kind::Led { color : 0 }, ( 2, 2 ), ( 0, 0 ) ),
	];

	assert_eq!( leads( &elements ), vec![ None, None ] );
}

#[test]
fn it_finds_nothing_without_an_led()
{
	assert_eq!( leads( &[ element( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ) ] ), Vec::new() );
}
