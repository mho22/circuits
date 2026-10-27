use simulator::{ arrange, eliminate, Error };


fn solve( matrix : Vec<f64>, rhs : Vec<f64> ) -> Result<Vec<f64>, Error>
{
	let size = rhs.len();
	let entries : Vec<( usize, usize, f64 )> = matrix.iter().enumerate().map( | ( index, &value ) | ( index / size, index % size, value ) ).collect();

	eliminate( size, &entries, rhs, &arrange( size, &entries ) )
}


#[test]
fn it_solves_a_linear_system()
{
	let solution = solve( vec![ 2.0, 1.0, 1.0, 3.0 ], vec![ 3.0, 5.0 ] ).unwrap();

	assert!( ( solution[ 0 ] - 0.8 ).abs() < 1e-12 );
	assert!( ( solution[ 1 ] - 1.4 ).abs() < 1e-12 );
}

#[test]
fn it_swaps_rows_when_a_pivot_is_zero()
{
	assert_eq!( solve( vec![ 0.0, 1.0, 1.0, 0.0 ], vec![ 2.0, 3.0 ] ), Ok( vec![ 3.0, 2.0 ] ) );
}

#[test]
fn it_solves_rows_of_very_different_sizes()
{
	assert_eq!( solve( vec![ 1.0, 0.0, 0.0, 1e-13 ], vec![ 1.0, 1e-13 ] ), Ok( vec![ 1.0, 1.0 ] ) );
}

#[test]
fn it_solves_an_empty_system()
{
	assert_eq!( solve( vec![], vec![] ), Ok( vec![] ) );
}

#[test]
fn it_rejects_a_singular_system()
{
	assert_eq!( solve( vec![ 1.0, 2.0, 2.0, 4.0 ], vec![ 1.0, 2.0 ] ), Err( Error::Singular ) );
	assert_eq!( solve( vec![ 0.0 ], vec![ 1.0 ] ), Err( Error::Singular ) );
}

#[test]
fn it_sums_the_entries_of_a_same_cell()
{
	let entries = [ ( 0, 0, 1.0 ), ( 0, 0, 1.0 ), ( 1, 1, 4.0 ), ( 1, 0, 0.0 ) ];

	assert_eq!( eliminate( 2, &entries, vec![ 4.0, 8.0 ], &[ 0, 1 ] ), Ok( vec![ 2.0, 2.0 ] ) );
}

#[test]
fn it_pivots_on_a_small_value_beside_a_large_one_in_a_same_row()
{
	let entries = [ ( 0, 0, 1e-12 ), ( 0, 1, -1.0 ), ( 1, 1, 1.0 ) ];

	assert_eq!( eliminate( 2, &entries, vec![ 0.0, 1e-12 ], &[ 0, 1 ] ), Ok( vec![ 1.0, 1e-12 ] ) );
}

#[test]
fn it_rejects_a_pivot_that_is_only_a_rounding_error()
{
	assert_eq!( solve( vec![ 0.1, 0.3, 0.3, 0.9 ], vec![ 1.0, 3.0 ] ), Err( Error::Singular ) );
}
