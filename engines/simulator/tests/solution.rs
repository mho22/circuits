use simulator::{ mix, Branch, Point, Reading, Segment, Solution };


#[test]
fn it_finds_the_branch_at_a_point()
{
	let branch = | x : i32 | Branch { segment : Segment { from : Point { x, y : 0 }, to : Point { x : x + 2, y : 0 } }, voltage : 0.0, current : 0.0 };

	let solution = Solution { readings : vec![], branches : vec![ branch( 0 ), branch( 2 ) ] };

	assert_eq!( solution.branch_at( Point { x : 1, y : 0 } ), Some( 0 ) );
	assert_eq!( solution.branch_at( Point { x : 3, y : 0 } ), Some( 1 ) );
	assert_eq!( solution.branch_at( Point { x : 1, y : 1 } ), None );
}

#[test]
fn it_mixes_a_value_toward_the_next_by_a_weight_and_replaces_a_value_that_is_not_finite()
{
	assert_eq!( [ mix( 1.0, 3.0, 0.25 ), mix( 1.0, 3.0, 0.0 ), mix( 1.0, 3.0, 1.0 ), mix( f64::NAN, 3.0, 0.25 ) ], [ 1.5, 1.0, 3.0, 3.0 ] );
}

#[test]
fn it_reaches_the_next_value_once_the_gap_is_below_a_femto_unit()
{
	assert_eq!( [ mix( 2e-12, 0.0, 0.5 ), mix( 1.5e-15, 0.0, 0.5 ), mix( 1.0, 1.0 + 1e-15, 0.5 ) ], [ 1e-12, 0.0, 1.0 + 1e-15 ] );
}

#[test]
fn it_blends_each_reading_and_each_branch_toward_the_next_solution()
{
	let segment = Segment { from : Point { x : 0, y : 0 }, to : Point { x : 2, y : 0 } };
	let solution = | value : f64 | Solution { readings : vec![ Reading { potentials : vec![ value, 0.0 ], currents : vec![ value, - value ] } ], branches : vec![ Branch { segment, voltage : value, current : value } ] };

	let mut seen = solution( 0.0 );

	seen.blend( &solution( 4.0 ), 0.5 );

	assert_eq!( seen, solution( 2.0 ) );
}

#[test]
fn it_takes_the_next_solution_when_its_readings_or_its_branches_differ()
{
	let branch = | x : i32 | Branch { segment : Segment { from : Point { x, y : 0 }, to : Point { x : x + 2, y : 0 } }, voltage : 1.0, current : 1.0 };
	let reading = | count : usize | Reading { potentials : vec![ 1.0; count ], currents : vec![ 1.0; count ] };
	let solution = | readings : Vec<Reading>, branches : Vec<Branch> | Solution { readings, branches };
	let next = solution( vec![ reading( 2 ) ], vec![ branch( 0 ) ] );

	let blended = | mut seen : Solution | { seen.blend( &next, 0.5 ); seen };

	assert_eq!( blended( solution( vec![ reading( 3 ) ], vec![ branch( 0 ) ] ) ), next );
	assert_eq!( blended( solution( vec![], vec![ branch( 0 ) ] ) ), next );
	assert_eq!( blended( solution( vec![ reading( 2 ) ], vec![ branch( 2 ) ] ) ), next );
	assert_eq!( blended( solution( vec![ reading( 2 ) ], vec![] ) ), next );
}

#[test]
fn it_gives_the_largest_potential_and_the_largest_current_in_size_and_ignores_an_undefined_current()
{
	let segment = Segment { from : Point { x : 0, y : 0 }, to : Point { x : 2, y : 0 } };
	let solution = Solution { readings : vec![ Reading { potentials : vec![ -9.0, 3.0 ], currents : vec![ -0.02, f64::NAN ] } ], branches : vec![ Branch { segment, voltage : 12.0, current : 0.01 } ] };

	assert_eq!( solution.peaks(), [ 9.0, 0.02 ] );
	assert_eq!( Solution::default().peaks(), [ 0.0, 0.0 ] );
}
