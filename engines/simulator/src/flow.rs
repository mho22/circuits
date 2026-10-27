use std::collections::VecDeque;


#[derive( Clone, Debug, Default, PartialEq )]
pub struct Flow
{
	edges : Vec<( usize, usize )>,
	order : Vec<usize>,
	parents : Vec<Option<( usize, usize )>>,
}

impl Flow
{
	pub fn build( vertices : usize, edges : &[ ( usize, usize ) ] ) -> Flow
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

		Flow { edges : edges.to_vec(), order, parents }
	}

	pub fn distribute( &self, injections : &[ f64 ] ) -> Vec<f64>
	{
		let mut totals = injections.to_vec();
		let mut currents = vec![ 0.0; self.edges.len() ];

		for &vertex in self.order.iter().rev()
		{
			let Some( ( edge, parent ) ) = self.parents[ vertex ] else { continue; };

			currents[ edge ] = if self.edges[ edge ].0 == vertex { totals[ vertex ] } else { - totals[ vertex ] };

			totals[ parent ] += totals[ vertex ];
		}

		currents
	}
}
