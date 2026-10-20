use simulator::distribute;


#[test]
fn it_carries_an_injection_along_a_chain()
{
	let currents = distribute( 3, &[ ( 0, 1 ), ( 1, 2 ) ], &[ 2.0, 0.0, -2.0 ] );

	assert_eq!( currents, vec![ 2.0, 2.0 ] );
}

#[test]
fn it_signs_a_current_against_the_edge_direction()
{
	let currents = distribute( 2, &[ ( 1, 0 ) ], &[ 2.0, -2.0 ] );

	assert_eq!( currents, vec![ -2.0 ] );
}

#[test]
fn it_splits_injections_at_a_junction()
{
	let currents = distribute( 4, &[ ( 0, 1 ), ( 1, 2 ), ( 1, 3 ) ], &[ 3.0, 0.0, -1.0, -2.0 ] );

	assert_eq!( currents, vec![ 3.0, 1.0, 2.0 ] );
}

#[test]
fn it_leaves_no_current_on_a_closing_edge()
{
	let currents = distribute( 3, &[ ( 0, 1 ), ( 1, 2 ), ( 2, 0 ) ], &[ 1.0, -1.0, 0.0 ] );

	assert_eq!( currents.iter().sum::<f64>(), 1.0 );
	assert!( currents.contains( &0.0 ) );
}

#[test]
fn it_handles_separate_groups()
{
	let currents = distribute( 4, &[ ( 0, 1 ), ( 2, 3 ) ], &[ 1.0, -1.0, -4.0, 4.0 ] );

	assert_eq!( currents, vec![ 1.0, -4.0 ] );
}
