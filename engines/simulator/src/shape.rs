use crate::{ Error, Point };


pub const FINE : i32 = 4;


#[derive( Clone, Debug, PartialEq )]
pub enum Shape
{
	Circle { center : Point, radius : i32 },
	Line { from : Point, to : Point },
	Curve { from : Point, through : Point, to : Point },
	Text { at : Point, content : String },
}

fn side( from : Point, to : Point, x : f64, y : f64 ) -> f64
{
	( to.x - from.x ) as f64 * ( y - from.y as f64 ) - ( to.y - from.y ) as f64 * ( x - from.x as f64 )
}

fn arc( from : Point, through : Point, to : Point ) -> Vec<Point>
{
	let square = | point : Point | ( point.x * point.x + point.y * point.y ) as f64;
	let divisor = 2.0 * ( from.x * ( through.y - to.y ) + through.x * ( to.y - from.y ) + to.x * ( from.y - through.y ) ) as f64;
	let x = ( square( from ) * ( through.y - to.y ) as f64 + square( through ) * ( to.y - from.y ) as f64 + square( to ) * ( from.y - through.y ) as f64 ) / divisor;
	let y = ( square( from ) * ( to.x - through.x ) as f64 + square( through ) * ( from.x - to.x ) as f64 + square( to ) * ( through.x - from.x ) as f64 ) / divisor;
	let radius = ( from.x as f64 - x ).hypot( from.y as f64 - y );
	let bulge = side( from, to, through.x as f64, through.y as f64 );
	let extremes = [ ( x - radius, y ), ( x + radius, y ), ( x, y - radius ), ( x, y + radius ) ];

	extremes.into_iter().filter( | &( x, y ) | side( from, to, x, y ) * bulge > 0.0 ).flat_map( | ( x, y ) | [ Point { x : x.floor() as i32, y : y.floor() as i32 }, Point { x : x.ceil() as i32, y : y.ceil() as i32 } ] ).chain( [ from, through, to ] ).collect()
}

impl Shape
{
	pub fn check( &self ) -> Result<(), Error>
	{
		let sized = match self
		{
			Shape::Circle { radius, .. } => *radius > 0,
			Shape::Line { from, to } => from != to,
			Shape::Curve { from, through, to } => side( *from, *to, through.x as f64, through.y as f64 ) != 0.0,
			Shape::Text { content, .. } => ! content.trim().is_empty(),
		};

		if sized { Ok( () ) } else { Err( Error::Flat ) }
	}

	pub fn shifted( &self, dx : i32, dy : i32 ) -> Shape
	{
		let shift = | point : Point | Point { x : point.x + dx, y : point.y + dy };

		match self
		{
			Shape::Circle { center, radius } => Shape::Circle { center : shift( *center ), radius : *radius },
			Shape::Line { from, to } => Shape::Line { from : shift( *from ), to : shift( *to ) },
			Shape::Curve { from, through, to } => Shape::Curve { from : shift( *from ), through : shift( *through ), to : shift( *to ) },
			Shape::Text { at, content } => Shape::Text { at : shift( *at ), content : content.clone() },
		}
	}

	pub fn bounds( &self ) -> ( Point, Point )
	{
		let points = match self
		{
			Shape::Circle { center, radius } => vec![ Point { x : center.x - radius, y : center.y - radius }, Point { x : center.x + radius, y : center.y + radius } ],
			Shape::Line { from, to } => vec![ *from, *to ],
			Shape::Curve { from, through, to } => arc( *from, *through, *to ),
			Shape::Text { at, content } => { let width = ( 3 * content.chars().count() as i32 + 1 ) / 2; vec![ Point { x : at.x, y : at.y - 1 }, Point { x : at.x + width, y : at.y + 1 } ] },
		};

		let low = Point { x : points.iter().map( | point | point.x ).min().unwrap_or( 0 ), y : points.iter().map( | point | point.y ).min().unwrap_or( 0 ) };
		let high = Point { x : points.iter().map( | point | point.x ).max().unwrap_or( 0 ), y : points.iter().map( | point | point.y ).max().unwrap_or( 0 ) };

		( low, high )
	}
}
