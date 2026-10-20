use simulator::Stamp;


#[test]
fn it_gives_the_current_of_its_conductance_and_source()
{
	let stamp = Stamp { across : ( 2, 0 ), conductance : 0.5, source : 0.25, weights : [ 1.0, -1.0, 0.0 ] };

	assert_eq!( stamp.current( &[ 1.0, 9.0, 3.0 ] ), 0.75 );
}
