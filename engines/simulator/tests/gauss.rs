use simulator::{ eliminate, Error };


#[test]
fn it_solves_a_linear_system()
{
	let solution = eliminate( vec![ 2.0, 1.0, 1.0, 3.0 ], vec![ 3.0, 5.0 ] ).unwrap();

	assert!( ( solution[ 0 ] - 0.8 ).abs() < 1e-12 );
	assert!( ( solution[ 1 ] - 1.4 ).abs() < 1e-12 );
}

#[test]
fn it_swaps_rows_when_a_pivot_is_zero()
{
	assert_eq!( eliminate( vec![ 0.0, 1.0, 1.0, 0.0 ], vec![ 2.0, 3.0 ] ), Ok( vec![ 3.0, 2.0 ] ) );
}

#[test]
fn it_solves_an_empty_system()
{
	assert_eq!( eliminate( vec![], vec![] ), Ok( vec![] ) );
}

#[test]
fn it_rejects_a_singular_system()
{
	assert_eq!( eliminate( vec![ 1.0, 2.0, 2.0, 4.0 ], vec![ 1.0, 2.0 ] ), Err( Error::Singular ) );
	assert_eq!( eliminate( vec![ 0.0 ], vec![ 1.0 ] ), Err( Error::Singular ) );
}
