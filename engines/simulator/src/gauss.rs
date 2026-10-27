use std::cmp::Ordering;

use crate::Error;


const TOLERANCE : f64 = 1e-12;
const RATIO : f64 = 0.1;
const BITS : usize = 64;


pub fn arrange( size : usize, entries : &[ ( usize, usize, f64 ) ] ) -> Vec<usize>
{
	let words = size.div_ceil( BITS );
	let mut links = vec![ 0_u64; size * words ];

	for &( row, column, _ ) in entries
	{
		if row == column { continue; }

		links[ row * words + column / BITS ] |= 1 << ( column % BITS );
		links[ column * words + row / BITS ] |= 1 << ( row % BITS );
	}

	let count = | links : &[ u64 ], node : usize | links[ node * words .. ( node + 1 ) * words ].iter().map( | word | word.count_ones() ).sum::<u32>();
	let mut degrees : Vec<u32> = ( 0 .. size ).map( | node | count( &links, node ) ).collect();
	let mut alive = vec![ true; size ];
	let mut order = Vec::with_capacity( size );

	for _ in 0 .. size
	{
		let next = ( 0 .. size ).filter( | &node | alive[ node ] ).min_by_key( | &node | degrees[ node ] ).unwrap();
		let neighbors = links[ next * words .. ( next + 1 ) * words ].to_vec();

		alive[ next ] = false;
		order.push( next );

		for ( word, &bits ) in neighbors.iter().enumerate()
		{
			let mut rest = bits;

			while rest != 0
			{
				let neighbor = word * BITS + rest.trailing_zeros() as usize;

				rest &= rest - 1;

				for ( index, &bits ) in neighbors.iter().enumerate() { links[ neighbor * words + index ] |= bits; }

				links[ neighbor * words + neighbor / BITS ] &= ! ( 1 << ( neighbor % BITS ) );
				links[ neighbor * words + next / BITS ] &= ! ( 1 << ( next % BITS ) );
				degrees[ neighbor ] = count( &links, neighbor );
			}
		}
	}

	order
}

pub fn eliminate( size : usize, entries : &[ ( usize, usize, f64 ) ], mut rhs : Vec<f64>, order : &[ usize ] ) -> Result<Vec<f64>, Error>
{
	let mut matrix = vec![ 0.0; size * size ];
	let mut magnitudes = vec![ 0.0; size * size ];
	let mut present = vec![ 0_u64; ( size * size ).div_ceil( BITS ) ];
	let mut rows : Vec<Vec<usize>> = vec![ Vec::new(); size ];
	let mut columns : Vec<Vec<usize>> = vec![ Vec::new(); size ];

	let mark = | present : &mut [ u64 ], cell : usize | -> bool
	{
		let fresh = present[ cell / BITS ] >> ( cell % BITS ) & 1 == 0;

		present[ cell / BITS ] |= 1 << ( cell % BITS );

		fresh
	};

	for &( row, column, value ) in entries
	{
		matrix[ row * size + column ] += value;
		magnitudes[ row * size + column ] += value.abs();

		if ! mark( &mut present, row * size + column ) { continue; }

		rows[ row ].push( column );
		columns[ column ].push( row );
	}

	let mut ranks = vec![ 0; size ];
	let mut done = vec![ false; size ];
	let mut pivots = Vec::with_capacity( size );

	for ( rank, &column ) in order.iter().enumerate() { ranks[ column ] = rank; }

	for ( rank, &column ) in order.iter().enumerate()
	{
		let candidates = std::mem::take( &mut columns[ column ] );
		let ( noise, active ) : ( Vec<usize>, Vec<usize> ) = candidates.into_iter().filter( | &row | ! done[ row ] ).partition( | &row | matrix[ row * size + column ].abs() <= magnitudes[ row * size + column ] * TOLERANCE );
		let largest = active.iter().map( | &row | matrix[ row * size + column ].abs() ).max_by( f64::total_cmp ).unwrap_or( 0.0 );
		let pivot = active.iter().copied().filter( | &row | matrix[ row * size + column ].abs().partial_cmp( &( RATIO * largest ) ) != Some( Ordering::Less ) ).min_by_key( | &row | rows[ row ].len() ).ok_or( Error::Singular )?;
		let head = matrix[ pivot * size + column ];

		for &row in &noise { matrix[ row * size + column ] = 0.0; }

		done[ pivot ] = true;
		pivots.push( pivot );

		let tail : Vec<usize> = rows[ pivot ].iter().copied().filter( | &other | ranks[ other ] > rank ).collect();

		for &row in active.iter().filter( | &&row | row != pivot )
		{
			let factor = matrix[ row * size + column ] / head;

			matrix[ row * size + column ] = 0.0;

			for &other in &tail
			{
				if mark( &mut present, row * size + other )
				{
					rows[ row ].push( other );
					columns[ other ].push( row );
				}

				let change = factor * matrix[ pivot * size + other ];

				matrix[ row * size + other ] -= change;
				magnitudes[ row * size + other ] = magnitudes[ row * size + other ].max( change.abs() );
			}

			rhs[ row ] -= factor * rhs[ pivot ];
		}
	}

	let mut solution = vec![ 0.0; size ];

	for ( rank, &column ) in order.iter().enumerate().rev()
	{
		let pivot = pivots[ rank ];
		let known : f64 = rows[ pivot ].iter().filter( | &&other | ranks[ other ] > rank ).map( | &other | matrix[ pivot * size + other ] * solution[ other ] ).sum();

		solution[ column ] = ( rhs[ pivot ] - known ) / matrix[ pivot * size + column ];
	}

	Ok( solution )
}
