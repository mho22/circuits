use crate::{ Element, Kind, Solution };


#[derive( Debug, Default )]
pub struct History
{
	values : Vec<[ f64; 2 ]>,
}

impl History
{
	pub fn record( elements : &[ Element ], solution : &Solution ) -> History
	{
		let size = elements.iter().map( | element | element.id as usize + 1 ).max().unwrap_or( 0 );
		let mut values = vec![ [ 0.0; 2 ]; size ];

		for ( element, reading ) in elements.iter().zip( &solution.readings )
		{
			let across = | ( a, b ) : ( usize, usize ) | reading.potentials[ a ] - reading.potentials[ b ];

			let value = match element.kind
			{
				Kind::Capacitor { .. } => [ across( ( 0, 1 ) ), 0.0 ],
				Kind::Inductor { .. } => [ reading.currents[ 0 ], 0.0 ],
				kind if ! kind.controls().is_empty() =>
				{
					let mut voltages = [ 0.0; 2 ];

					for ( index, &pair ) in kind.controls().iter().enumerate() { voltages[ index ] = across( pair ); }

					voltages
				},
				_ => continue,
			};

			values[ element.id as usize ] = value;
		}

		History { values }
	}

	pub fn get( &self, id : u32 ) -> [ f64; 2 ]
	{
		self.values.get( id as usize ).copied().unwrap_or( [ 0.0; 2 ] )
	}
}
