use simulator::{ Element, Kind, Point, Segment };


fn element( kind : Kind, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { id : 0, kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } } }
}

fn points( list : &[ ( i32, i32 ) ] ) -> Vec<Point>
{
	list.iter().map( | &( x, y ) | Point { x, y } ).collect()
}


#[test]
fn it_gives_a_two_ended_part_its_two_ends()
{
	assert_eq!( element( Kind::Resistor { ohms : 1.0 }, ( 0, 0 ), ( 4, 0 ) ).terminals(), points( &[ ( 0, 0 ), ( 4, 0 ) ] ) );
	assert_eq!( element( Kind::Diode, ( 0, 0 ), ( 2, 2 ) ).terminals(), points( &[ ( 0, 0 ), ( 2, 2 ) ] ) );
}

#[test]
fn it_puts_the_collector_left_and_the_emitter_right_one_step_past_the_bar()
{
	assert_eq!( element( Kind::Npn, ( 0, 0 ), ( 2, 0 ) ).terminals(), points( &[ ( 0, 0 ), ( 3, -1 ), ( 3, 1 ) ] ) );
	assert_eq!( element( Kind::Pnp, ( 0, 0 ), ( 2, 0 ) ).terminals(), points( &[ ( 0, 0 ), ( 3, -1 ), ( 3, 1 ) ] ) );
	assert_eq!( element( Kind::Npn, ( 0, 0 ), ( 0, 2 ) ).terminals(), points( &[ ( 0, 0 ), ( 1, 3 ), ( -1, 3 ) ] ) );
	assert_eq!( element( Kind::Npn, ( 2, 0 ), ( 0, 0 ) ).terminals(), points( &[ ( 2, 0 ), ( -1, 1 ), ( -1, -1 ) ] ) );
	assert_eq!( element( Kind::Npn, ( 0, 0 ), ( 2, 2 ) ).terminals(), points( &[ ( 0, 0 ), ( 4, 2 ), ( 2, 4 ) ] ) );
}

#[test]
fn it_puts_the_drain_left_and_the_source_right_one_step_past_the_bar()
{
	assert_eq!( element( Kind::Nmos, ( 0, 0 ), ( 2, 0 ) ).terminals(), points( &[ ( 0, 0 ), ( 3, -1 ), ( 3, 1 ) ] ) );
	assert_eq!( element( Kind::Pmos, ( 0, 0 ), ( 0, 2 ) ).terminals(), points( &[ ( 0, 0 ), ( 1, 3 ), ( -1, 3 ) ] ) );
}
