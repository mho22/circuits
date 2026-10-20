use std::collections::HashMap;

use crate::{ Element, Kind, Solution };


#[derive( Debug, Default )]
pub struct History
{
	values : HashMap<u32, [ f64; 2 ]>,
}

impl History
{
	pub fn record( elements : &[ Element ], solution : &Solution ) -> History
	{
		let values = elements.iter().zip( &solution.readings ).filter_map( | ( element, reading ) |
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
				_ => return None,
			};

			Some( ( element.id, value ) )
		} ).collect();

		History { values }
	}

	pub fn get( &self, id : u32 ) -> [ f64; 2 ]
	{
		self.values.get( &id ).copied().unwrap_or( [ 0.0; 2 ] )
	}
}
