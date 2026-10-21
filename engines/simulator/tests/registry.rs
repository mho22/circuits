use simulator::Registry;


#[test]
fn it_gives_each_path_its_own_id()
{
	let mut registry = Registry::default();

	assert_eq!( registry.id( &[ 4 ] ), 0 );
	assert_eq!( registry.id( &[ 4, 2 ] ), 1 );
	assert_eq!( registry.id( &[ 2, 4 ] ), 2 );
}

#[test]
fn it_gives_a_path_the_same_id_each_time()
{
	let mut registry = Registry::default();

	registry.id( &[ 4 ] );
	registry.id( &[ 4, 2 ] );

	assert_eq!( registry.id( &[ 4 ] ), 0 );
	assert_eq!( registry.id( &[ 4, 2 ] ), 1 );
}

#[test]
fn it_finds_only_a_path_it_gave_an_id()
{
	let mut registry = Registry::default();

	registry.id( &[ 4 ] );
	registry.id( &[ 4, 2 ] );

	assert_eq!( registry.find( &[ 4, 2 ] ), Some( 1 ) );
	assert_eq!( registry.find( &[ 2 ] ), None );
}
