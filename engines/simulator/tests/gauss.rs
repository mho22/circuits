use simulator::arrange;


#[test]
fn it_starts_with_an_end_of_a_star_rather_than_its_center()
{
	let entries = [ ( 0, 1, 1.0 ), ( 0, 2, 1.0 ), ( 0, 3, 1.0 ) ];

	assert_eq!( arrange( 4, &entries )[ 0 ], 1 );
}
