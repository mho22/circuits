use std::collections::{ HashMap, HashSet };
use std::hash::{ BuildHasher, Hash };

use simulator::{ Hasher, Hashing, Point };


fn hash( value : impl Hash ) -> u64
{
	Hashing::default().hash_one( value )
}


#[test]
fn it_gives_the_same_hash_to_the_same_value()
{
	assert_eq!( hash( Point { x : 3, y : -4 } ), hash( Point { x : 3, y : -4 } ) );
	assert_eq!( hash( [ 1u32, 2, 3 ].as_slice() ), hash( vec![ 1u32, 2, 3 ] ) );
	assert_eq!( std::hash::Hasher::finish( &Hasher::default() ), 0 );
}

#[test]
fn it_gives_distinct_hashes_to_the_points_of_a_grid()
{
	let hashes : HashSet<u64> = ( -100 .. 100 ).flat_map( | x | ( -100 .. 100 ).map( move | y | hash( Point { x, y } ) ) ).collect();

	assert_eq!( hashes.len(), 40_000 );
}

#[test]
fn it_spreads_points_that_differ_only_in_their_high_bits_over_the_low_bits()
{
	let buckets : HashSet<u64> = ( 0 .. 4096 ).map( | scope | hash( Point { x : scope * 65_536, y : 0 } ) & 4095 ).collect();

	assert!( buckets.len() > 2000 );
}

#[test]
fn it_tells_apart_paths_that_share_their_words()
{
	assert_ne!( hash( vec![ 1u32, 2 ] ), hash( vec![ 2u32, 1 ] ) );
	assert_ne!( hash( vec![ 1u32 ] ), hash( vec![ 1u32, 0 ] ) );
}

#[test]
fn it_finds_each_key_of_a_map()
{
	let map : HashMap<Point, usize, Hashing> = ( 0 .. 1000 ).map( | index | ( Point { x : index as i32 * 65_536, y : 0 }, index ) ).collect();

	assert!( ( 0 .. 1000 ).all( | index | map[ &Point { x : index as i32 * 65_536, y : 0 } ] == index ) );
}
