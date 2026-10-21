use crate::{ Kind, Point, Segment };


#[derive( Clone, Copy, Debug, PartialEq )]
pub struct Element
{
	pub id : u32,
	pub kind : Kind,
	pub segment : Segment,
	pub mirrored : bool,
}

impl Element
{
	pub fn terminals( &self ) -> Vec<Point>
	{
		let Segment { from, to } = self.segment;

		if self.kind.pin() || matches!( self.kind, Kind::Toggle { .. } ) { return vec![ to, to ]; }

		if ! matches!( self.kind, Kind::Npn | Kind::Pnp | Kind::Nmos | Kind::Pmos ) { return vec![ from, to ]; }

		let dx = ( to.x - from.x ).signum();
		let dy = ( to.y - from.y ).signum();

		let left = Point { x : to.x + dx + dy, y : to.y + dy - dx };
		let right = Point { x : to.x + dx - dy, y : to.y + dy + dx };

		if self.mirrored { return vec![ from, right, left ]; }

		vec![ from, left, right ]
	}
}
