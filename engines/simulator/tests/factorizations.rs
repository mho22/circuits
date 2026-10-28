use simulator::{ Error, Factorizations, Factors };


const FOO : [ ( usize, usize, f64 ); 4 ] = [ ( 0, 0, 1.0 / 7.0 ), ( 0, 1, 1.0 ), ( 1, 0, 1.0 ), ( 1, 1, 3.0 ) ];
const BAR : [ ( usize, usize, f64 ); 4 ] = [ ( 0, 0, 1.0 / 7.0 + 1e-7 ), ( 0, 1, 1.0 ), ( 1, 0, 1.0 ), ( 1, 1, 3.0 ) ];
const BAZ : [ ( usize, usize, f64 ); 4 ] = [ ( 0, 0, 5.0 ), ( 0, 1, 1.0 ), ( 1, 0, 1.0 ), ( 1, 1, 3.0 ) ];


#[test]
fn it_refines_with_the_factors_of_a_close_matrix_solved_before()
{
	let factorizations = Factorizations::default();
	let rhs = vec![ 0.3, 0.7 ];

	factorizations.solve( 2, &FOO, rhs.clone(), &[ 0, 1 ] ).unwrap();

	assert_eq!( factorizations.solve( 2, &BAR, rhs.clone(), &[ 0, 1 ] ), Ok( Factors::new( 2, &FOO, &[ 0, 1 ] ).unwrap().refine( &BAR, &rhs ).unwrap() ) );
}

#[test]
fn it_factors_again_a_distant_matrix()
{
	let factorizations = Factorizations::default();
	let rhs = vec![ 0.3, 0.7 ];

	factorizations.solve( 2, &FOO, rhs.clone(), &[ 0, 1 ] ).unwrap();

	assert_eq!( factorizations.solve( 2, &BAZ, rhs.clone(), &[ 0, 1 ] ), Ok( Factors::new( 2, &BAZ, &[ 0, 1 ] ).unwrap().solve( rhs ) ) );
}

#[test]
fn it_rejects_a_singular_matrix_close_to_one_solved_before()
{
	let factorizations = Factorizations::default();

	factorizations.solve( 1, &[ ( 0, 0, 1e-300 ) ], vec![ 1.0 ], &[ 0 ] ).unwrap();

	assert_eq!( factorizations.solve( 1, &[ ( 0, 0, 0.0 ) ], vec![ 1.0 ], &[ 0 ] ), Err( Error::Singular ) );
}

#[test]
fn it_clones_without_the_factors_and_equals_any_other()
{
	let factorizations = Factorizations::default();
	let rhs = vec![ 1.0, 2.0 ];

	factorizations.solve( 2, &FOO, rhs.clone(), &[ 0, 1 ] ).unwrap();

	assert_eq!( factorizations.clone().solve( 2, &BAR, rhs.clone(), &[ 0, 1 ] ), Ok( Factors::new( 2, &BAR, &[ 0, 1 ] ).unwrap().solve( rhs.clone() ) ) );
	assert_ne!( Factors::new( 2, &BAR, &[ 0, 1 ] ).unwrap().solve( rhs.clone() ), Factors::new( 2, &FOO, &[ 0, 1 ] ).unwrap().refine( &BAR, &rhs ).unwrap() );
	assert_eq!( factorizations, Factorizations::default() );
}
