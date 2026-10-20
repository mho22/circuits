use crate::{ Element, Kind, Point };


const LEFT : usize = 0;
const RIGHT : usize = 1;
const TOP : usize = 2;
const BOTTOM : usize = 3;


fn side( point : Point, low : Point, high : Point ) -> usize
{
	let distances = [ point.x - low.x, high.x - point.x, point.y - low.y, high.y - point.y ];

	( 0 .. 4 ).min_by_key( | &side | distances[ side ] ).unwrap()
}

fn rotate( point : Point, rotation : u8 ) -> Point
{
	( 0 .. rotation ).fold( point, | point, _ | Point { x : - point.y, y : point.x } )
}


#[derive( Clone, Debug, PartialEq )]
pub struct Outline
{
	pub width : i32,
	pub height : i32,
	pub pins : Vec<Point>,
}

impl Outline
{
	pub fn of( elements : &[ Element ] ) -> Outline
	{
		let points : Vec<Point> = elements.iter().filter( | element | element.kind == Kind::Pin ).map( | element | element.segment.from ).collect();
		let low = Point { x : points.iter().map( | point | point.x ).min().unwrap_or( 0 ), y : points.iter().map( | point | point.y ).min().unwrap_or( 0 ) };
		let high = Point { x : points.iter().map( | point | point.x ).max().unwrap_or( 0 ), y : points.iter().map( | point | point.y ).max().unwrap_or( 0 ) };
		let sides : Vec<usize> = points.iter().map( | &point | side( point, low, high ) ).collect();
		let count = | side : usize | sides.iter().filter( | &&other | other == side ).count() as i32;
		let width = ( 2 * ( count( TOP ).max( count( BOTTOM ) ) + 1 ) ).max( 4 );
		let height = ( 2 * ( count( LEFT ).max( count( RIGHT ) ) + 1 ) ).max( 4 );

		let pins = ( 0 .. points.len() ).map( | index |
		{
			let side = sides[ index ];
			let along = | other : usize | ( if side < TOP { points[ other ].y } else { points[ other ].x }, other );
			let rank = ( 0 .. points.len() ).filter( | &other | sides[ other ] == side && along( other ) < along( index ) ).count() as i32;
			let offset = 2 * rank - ( count( side ) - 1 );

			match side
			{
				LEFT => Point { x : - width / 2, y : offset },
				RIGHT => Point { x : width / 2, y : offset },
				TOP => Point { x : offset, y : - height / 2 },
				_ => Point { x : offset, y : height / 2 },
			}
		} ).collect();

		Outline { width, height, pins }
	}

	pub fn terminals( &self, center : Point, rotation : u8 ) -> Vec<Point>
	{
		self.pins.iter().map( | &pin | { let turned = rotate( pin, rotation ); Point { x : center.x + turned.x, y : center.y + turned.y } } ).collect()
	}
}
