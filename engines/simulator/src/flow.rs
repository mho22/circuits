use std::collections::VecDeque;


pub fn distribute( vertices : usize, edges : &[ ( usize, usize ) ], injections : &[ f64 ] ) -> Vec<f64>
{
	let mut neighbors = vec![ Vec::new(); vertices ];

	for ( index, &( a, b ) ) in edges.iter().enumerate()
	{
		neighbors[ a ].push( ( index, b ) );
		neighbors[ b ].push( ( index, a ) );
	}

	let mut parents : Vec<Option<( usize, usize )>> = vec![ None; vertices ];
	let mut visited = vec![ false; vertices ];
	let mut order = Vec::with_capacity( vertices );

	for root in 0 .. vertices
	{
		if visited[ root ] { continue; }

		visited[ root ] = true;

		let mut queue = VecDeque::from( [ root ] );

		while let Some( vertex ) = queue.pop_front()
		{
			order.push( vertex );

			for &( edge, neighbor ) in &neighbors[ vertex ]
			{
				if visited[ neighbor ] { continue; }

				visited[ neighbor ] = true;
				parents[ neighbor ] = Some( ( edge, vertex ) );

				queue.push_back( neighbor );
			}
		}
	}

	let mut totals = injections.to_vec();
	let mut currents = vec![ 0.0; edges.len() ];

	for &vertex in order.iter().rev()
	{
		let Some( ( edge, parent ) ) = parents[ vertex ] else { continue; };

		currents[ edge ] = if edges[ edge ].0 == vertex { totals[ vertex ] } else { - totals[ vertex ] };

		totals[ parent ] += totals[ vertex ];
	}

	currents
}
