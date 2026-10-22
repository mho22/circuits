use crate::shape::FINE;
use crate::{ Document, Kind, Point, Segment };


const COLUMN : i32 = 2;
const MARGIN : i32 = 2;


fn rotate( point : Point, rotation : u8 ) -> Point
{
	( 0 .. rotation ).fold( point, | point, _ | Point { x : - point.y, y : point.x } )
}

fn ceil( value : i32 ) -> i32
{
	- ( - value ).div_euclid( FINE )
}


#[derive( Clone, Debug, PartialEq )]
pub struct Outline
{
	pub low : Point,
	pub high : Point,
	pub pins : Vec<Point>,
}

impl Outline
{
	pub fn of( document : &Document ) -> Outline
	{
		let outputs : Vec<bool> = document.elements().iter().filter( | element | element.kind.pin() ).map( | element | element.kind == Kind::Output ).collect();
		let mut pins = Vec::new();

		for ( index, &output ) in outputs.iter().enumerate()
		{
			let count = outputs.iter().filter( | &&other | other == output ).count() as i32;
			let rank = outputs[ .. index ].iter().filter( | &&other | other == output ).count() as i32;
			let column = Point { x : if output { COLUMN } else { - COLUMN }, y : 2 * rank - ( count - 1 ) };

			pins.push( document.layout().get( index ).copied().unwrap_or( column ) );
		}

		let shapes = document.drawing().iter().flat_map( | shape |
		{
			let ( low, high ) = shape.bounds();

			[ Point { x : low.x.div_euclid( FINE ), y : low.y.div_euclid( FINE ) }, Point { x : ceil( high.x ), y : ceil( high.y ) } ]
		} );
		let margin = if document.drawing().is_empty() { vec![ Point { x : - MARGIN, y : - MARGIN }, Point { x : MARGIN, y : MARGIN } ] } else { Vec::new() };
		let points : Vec<Point> = pins.iter().copied().chain( shapes ).chain( margin ).collect();
		let low = Point { x : points.iter().map( | point | point.x ).min().unwrap(), y : points.iter().map( | point | point.y ).min().unwrap() };
		let high = Point { x : points.iter().map( | point | point.x ).max().unwrap(), y : points.iter().map( | point | point.y ).max().unwrap() };

		Outline { low, high, pins }
	}

	pub fn zone( &self, center : Point, rotation : u8 ) -> Segment
	{
		let first = rotate( Point { x : self.low.x + 1, y : self.low.y + 1 }, rotation );
		let second = rotate( Point { x : self.high.x - 1, y : self.high.y - 1 }, rotation );

		Segment { from : Point { x : center.x + first.x.min( second.x ), y : center.y + first.y.min( second.y ) }, to : Point { x : center.x + first.x.max( second.x ), y : center.y + first.y.max( second.y ) } }
	}

	pub fn terminals( &self, center : Point, rotation : u8 ) -> Vec<Point>
	{
		self.pins.iter().map( | &pin | { let turned = rotate( pin, rotation ); Point { x : center.x + turned.x, y : center.y + turned.y } } ).collect()
	}
}
