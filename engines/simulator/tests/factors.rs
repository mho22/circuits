use simulator::{ arrange, Error, Factors };


fn solve( matrix : Vec<f64>, rhs : Vec<f64> ) -> Result<Vec<f64>, Error>
{
	let size = rhs.len();
	let entries : Vec<( usize, usize, f64 )> = matrix.iter().enumerate().map( | ( index, &value ) | ( index / size, index % size, value ) ).collect();

	Factors::new( size, &entries, &arrange( size, &entries ) ).map( | factors | factors.solve( rhs ) )
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

	assert_eq!( Factors::new( 2, &entries, &[ 0, 1 ] ).map( | factors | factors.solve( vec![ 4.0, 8.0 ] ) ), Ok( vec![ 2.0, 2.0 ] ) );
}

#[test]
fn it_pivots_on_a_small_value_beside_a_large_one_in_a_same_row()
{
	let entries = [ ( 0, 0, 1e-12 ), ( 0, 1, -1.0 ), ( 1, 1, 1.0 ) ];

	assert_eq!( Factors::new( 2, &entries, &[ 0, 1 ] ).map( | factors | factors.solve( vec![ 0.0, 1e-12 ] ) ), Ok( vec![ 1.0, 1e-12 ] ) );
}

#[test]
fn it_rejects_a_pivot_that_is_only_a_rounding_error()
{
	assert_eq!( solve( vec![ 0.1, 0.3, 0.3, 0.9 ], vec![ 1.0, 3.0 ] ), Err( Error::Singular ) );
}

#[test]
fn it_solves_a_grid_where_each_row_line_meets_each_column_line()
{
	let ( rows, columns ) = ( 34, 40 );
	let size = rows + columns;
	let mut matrix = vec![ 0.0; size * size ];

	for row in 0 .. rows
	{
		for column in rows .. size
		{
			let conductance = 1.0 + ( row * column % 7 ) as f64;

			matrix[ row * size + row ] += conductance;
			matrix[ column * size + column ] += conductance;
			matrix[ row * size + column ] -= conductance;
			matrix[ column * size + row ] -= conductance;
		}
	}

	for node in 0 .. size { matrix[ node * size + node ] += 1.0; }

	let expected : Vec<f64> = ( 0 .. size ).map( | node | node as f64 ).collect();
	let rhs = ( 0 .. size ).map( | row | ( 0 .. size ).map( | column | matrix[ row * size + column ] * expected[ column ] ).sum() ).collect();
	let solution = solve( matrix, rhs ).unwrap();

	assert!( solution.iter().zip( &expected ).all( | ( value, expected ) | ( value - expected ).abs() < 1e-9 ) );
}

#[test]
fn it_refines_a_solution_with_the_factors_of_a_close_matrix()
{
	let factors = Factors::new( 2, &[ ( 0, 0, 2.0 ), ( 0, 1, 1.0 ), ( 1, 0, 1.0 ), ( 1, 1, 3.0 ) ], &[ 0, 1 ] ).unwrap();
	let solution = factors.refine( &[ ( 0, 0, 2.000001 ), ( 0, 1, 1.0 ), ( 1, 0, 1.0 ), ( 1, 1, 3.0 ) ], &[ 3.000001, 4.0 ] ).unwrap();

	assert!( ( solution[ 0 ] - 1.0 ).abs() < 1e-15 && ( solution[ 1 ] - 1.0 ).abs() < 1e-15 );
}

#[test]
fn it_gives_up_refining_with_the_factors_of_a_distant_matrix()
{
	let factors = Factors::new( 1, &[ ( 0, 0, 1.0 ) ], &[ 0 ] ).unwrap();

	assert_eq!( factors.refine( &[ ( 0, 0, 3.0 ) ], &[ 3.0 ] ), None );
}

#[test]
fn it_is_near_a_matrix_whose_entries_differ_by_a_millionth()
{
	let factors = Factors::new( 1, &[ ( 0, 0, 1.0 ) ], &[ 0 ] ).unwrap();

	assert!( factors.near( &[ ( 0, 0, 1.000001 ) ] ) );
	assert!( ! factors.near( &[ ( 0, 0, 1.00001 ) ] ) );
	assert!( ! factors.near( &[ ( 0, 0, 1.0 ), ( 0, 0, 0.0 ) ] ) );
}

#[test]
fn it_replays_the_factors_of_a_matrix_of_a_same_shape()
{
	let factors = Factors::new( 3, &[ ( 0, 0, 4.0 ), ( 0, 1, 1.0 ), ( 1, 0, 1.0 ), ( 1, 1, 3.0 ), ( 1, 2, 1.0 ), ( 2, 1, 1.0 ), ( 2, 2, 2.0 ) ], &[ 0, 1, 2 ] ).unwrap();
	let entries = [ ( 0, 0, 7.0 ), ( 0, 1, 2.0 ), ( 1, 0, 1.5 ), ( 1, 1, 5.0 ), ( 1, 2, 0.5 ), ( 2, 1, 3.0 ), ( 2, 2, 9.0 ) ];
	let rhs = vec![ 0.1, 0.2, 0.3 ];

	assert_eq!( factors.replay( &entries ).map( | factors | factors.solve( rhs.clone() ) ), Some( Factors::new( 3, &entries, &[ 0, 1, 2 ] ).unwrap().solve( rhs ) ) );
}

#[test]
fn it_does_not_replay_the_factors_of_a_matrix_of_another_shape()
{
	let factors = Factors::new( 2, &[ ( 0, 0, 2.0 ), ( 1, 1, 3.0 ) ], &[ 0, 1 ] ).unwrap();

	assert!( factors.replay( &[ ( 0, 0, 2.0 ), ( 0, 1, 3.0 ) ] ).is_none() );
	assert!( factors.replay( &[ ( 0, 0, 2.0 ), ( 1, 1, 3.0 ), ( 1, 0, 1.0 ) ] ).is_none() );
}

#[test]
fn it_does_not_replay_the_factors_when_the_pivot_changes()
{
	let factors = Factors::new( 2, &[ ( 0, 0, 2.0 ), ( 0, 1, 1.0 ), ( 1, 0, 1.0 ), ( 1, 1, 3.0 ) ], &[ 0, 1 ] ).unwrap();

	assert!( factors.replay( &[ ( 0, 0, 0.01 ), ( 0, 1, 1.0 ), ( 1, 0, 1.0 ), ( 1, 1, 3.0 ) ] ).is_none() );
}

#[test]
fn it_does_not_replay_the_factors_when_a_cell_stops_being_a_rounding_error()
{
	let factors = Factors::new( 2, &[ ( 0, 0, 1.0 ), ( 0, 0, -1.0 ), ( 0, 1, 1.0 ), ( 1, 0, 1.0 ) ], &[ 0, 1 ] ).unwrap();

	assert!( factors.replay( &[ ( 0, 0, 1.0 ), ( 0, 0, -0.5 ), ( 0, 1, 1.0 ), ( 1, 0, 1.0 ) ] ).is_none() );
}
