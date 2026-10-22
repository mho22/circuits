use crate::{ Document, Element, Kind, Outline, Point };


fn terminals( layers : &[ Document ], element : &Element ) -> Vec<Point>
{
	let Kind::Symbol { layer, rotation } = element.kind else { return element.terminals(); };

	layers.get( layer as usize ).map_or( Vec::new(), | document | Outline::of( document ).terminals( element.segment.from, rotation ) )
}


pub fn driven( layers : &[ Document ], layer : usize, id : u32, point : Point ) -> bool
{
	layers[ layer ].elements().iter().filter( | element | element.id != id ).any( | element | terminals( layers, element ).contains( &point ) )
}
