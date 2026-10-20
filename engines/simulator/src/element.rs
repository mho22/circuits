use crate::{ Kind, Point, Segment };


#[derive( Clone, Copy, Debug, PartialEq )]
pub struct Element
{
	pub id : u32,
	pub kind : Kind,
	pub segment : Segment,
}

impl Element
{
	pub fn terminals( &self ) -> Vec<Point>
	{
		let Segment { from, to } = self.segment;

		if ! matches!( self.kind, Kind::Npn | Kind::Pnp ) { return vec![ from, to ]; }

		let dx = ( to.x - from.x ).signum();
		let dy = ( to.y - from.y ).signum();

		vec![ from, Point { x : to.x + dx + dy, y : to.y + dy - dx }, Point { x : to.x + dx - dy, y : to.y + dy + dx } ]
	}
}
