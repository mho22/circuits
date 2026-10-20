use std::collections::HashMap;

use crate::{ Element, Kind, Solution };


#[derive( Debug, Default )]
pub struct History
{
	values : HashMap<u32, f64>,
}

impl History
{
	pub fn record( elements : &[ Element ], solution : &Solution ) -> History
	{
		let values = elements.iter().zip( &solution.readings ).filter_map( | ( element, reading ) | match element.kind
		{
			Kind::Capacitor { .. } => Some( ( element.id, reading.from - reading.to ) ),
			Kind::Inductor { .. } => Some( ( element.id, reading.current ) ),
			_ => None,
		} ).collect();

		History { values }
	}

	pub fn get( &self, id : u32 ) -> f64
	{
		self.values.get( &id ).copied().unwrap_or( 0.0 )
	}
}
