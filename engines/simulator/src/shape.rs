use crate::{ Error, Point };


pub const FINE : i32 = 4;


#[derive( Clone, Debug, PartialEq )]
pub enum Shape
{
	Circle { center : Point, radius : i32 },
	Line { from : Point, to : Point },
}

impl Shape
{
	pub fn check( &self ) -> Result<(), Error>
	{
		let sized = match self
		{
			Shape::Circle { radius, .. } => *radius > 0,
			Shape::Line { from, to } => from != to,
		};

		if sized { Ok( () ) } else { Err( Error::Flat ) }
	}

	pub fn bounds( &self ) -> ( Point, Point )
	{
		let points = match self
		{
			Shape::Circle { center, radius } => vec![ Point { x : center.x - radius, y : center.y - radius }, Point { x : center.x + radius, y : center.y + radius } ],
			Shape::Line { from, to } => vec![ *from, *to ],
		};

		let low = Point { x : points.iter().map( | point | point.x ).min().unwrap_or( 0 ), y : points.iter().map( | point | point.y ).min().unwrap_or( 0 ) };
		let high = Point { x : points.iter().map( | point | point.x ).max().unwrap_or( 0 ), y : points.iter().map( | point | point.y ).max().unwrap_or( 0 ) };

		( low, high )
	}
}
