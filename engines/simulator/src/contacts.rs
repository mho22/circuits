use std::collections::HashMap;
use crate::{ Document, Element, Kind, Outline, Point };


pub fn terminals( layers : &[ Document ], element : &Element ) -> Vec<Point>
{
	let Kind::Symbol { layer, rotation } = element.kind else { return element.terminals(); };

	layers.get( layer as usize ).map_or( Vec::new(), | document | Outline::of( document ).terminals( element.segment.from, rotation ) )
}


pub struct Contacts
{
	ids : HashMap<Point, Vec<u32>>,
}

impl Contacts
{
	pub fn of( layers : &[ Document ], layer : usize ) -> Contacts
	{
		let mut ids : HashMap<Point, Vec<u32>> = HashMap::new();

		for element in layers[ layer ].elements()
		{
			for point in terminals( layers, element ) { ids.entry( point ).or_default().push( element.id ); }
		}

		Contacts { ids }
	}

	pub fn driven( &self, id : u32, point : Point ) -> bool
	{
		self.ids.get( &point ).is_some_and( | ids | ids.iter().any( | &other | other != id ) )
	}
}
