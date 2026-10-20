use simulator::{ Branch, Point, Segment, Solution };


#[test]
fn it_finds_the_branch_at_a_point()
{
	let branch = | x : i32 | Branch { segment : Segment { from : Point { x, y : 0 }, to : Point { x : x + 2, y : 0 } }, voltage : 0.0, current : 0.0 };

	let solution = Solution { readings : vec![], branches : vec![ branch( 0 ), branch( 2 ) ] };

	assert_eq!( solution.branch_at( Point { x : 1, y : 0 } ), Some( 0 ) );
	assert_eq!( solution.branch_at( Point { x : 3, y : 0 } ), Some( 1 ) );
	assert_eq!( solution.branch_at( Point { x : 1, y : 1 } ), None );
}
