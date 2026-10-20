use simulator::UnionFind;


#[test]
fn it_starts_with_each_item_alone()
{
	let mut forest = UnionFind::new( 3 );

	assert_eq!( [ forest.find( 0 ), forest.find( 1 ), forest.find( 2 ) ], [ 0, 1, 2 ] );
}

#[test]
fn it_joins_items_transitively()
{
	let mut forest = UnionFind::new( 4 );

	forest.union( 0, 1 );
	forest.union( 2, 1 );

	assert_eq!( forest.find( 0 ), forest.find( 2 ) );
	assert_ne!( forest.find( 0 ), forest.find( 3 ) );
}
