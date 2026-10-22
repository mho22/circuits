use crate::{ Element, Error, Kind, Operation, Point };


fn within( point : Point, low : Point, high : Point ) -> bool
{
	point.x >= low.x && point.x <= high.x && point.y >= low.y && point.y <= high.y
}

fn interior( point : Point, low : Point, high : Point ) -> bool
{
	point.x > low.x && point.x < high.x && point.y > low.y && point.y < high.y
}


#[derive( Debug, PartialEq )]
pub struct Enclosure
{
	pub inner : Vec<Element>,
	pub outside : Vec<Operation>,
	pub pins : Vec<Point>,
}


pub fn enclose( elements : &[ Element ], low : Point, high : Point, points : impl Fn( &Element ) -> Vec<Point> ) -> Result<Enclosure, Error>
{
	let mut inner = Vec::new();

	for element in elements
	{
		let points = points( element );

		if points.iter().all( | &point | within( point, low, high ) ) { inner.push( *element ); continue; }

		if element.kind != Kind::Wire && points.iter().any( | &point | interior( point, low, high ) ) { return Err( Error::Straddled ); }
	}

	if inner.is_empty() { return Err( Error::Unselected ); }

	let outside = inner.iter().map( | element | Operation::Delete { id : element.id } ).collect();
	let pins = inner.iter().filter( | element | element.kind.pin() ).map( | element | element.segment.to ).collect();

	Ok( Enclosure { inner, outside, pins } )
}
