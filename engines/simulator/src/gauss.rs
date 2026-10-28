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
