use crate::Point;


#[derive( Clone, Copy, Debug, PartialEq, Eq )]
pub struct Wire
{
	pub id : u32,
	pub from : Point,
	pub to : Point,
}

impl Wire
{
	pub fn is_straight( from : Point, to : Point ) -> bool
	{
		let dx = ( to.x - from.x ).abs();
		let dy = ( to.y - from.y ).abs();

		dx == 0 || dy == 0 || dx == dy
	}

	pub fn snap( from : Point, cursor : Point ) -> Point
	{
		let dx = cursor.x - from.x;
		let dy = cursor.y - from.y;
		let diagonal = ( dx.abs() + dy.abs() + 1 ) / 2;

		let candidates = [
			Point { x : cursor.x, y : from.y },
			Point { x : from.x, y : cursor.y },
			Point { x : from.x + diagonal * dx.signum(), y : from.y + diagonal * dy.signum() },
		];

		let distance = | point : &Point | ( point.x - cursor.x ).pow( 2 ) + ( point.y - cursor.y ).pow( 2 );

		candidates.into_iter().min_by_key( distance ).unwrap()
	}

	pub fn contains( &self, point : Point ) -> bool
	{
		let cross = ( self.to.x - self.from.x ) * ( point.y - self.from.y ) - ( self.to.y - self.from.y ) * ( point.x - self.from.x );

		if cross != 0 { return false; }

		let within_x = point.x >= self.from.x.min( self.to.x ) && point.x <= self.from.x.max( self.to.x );
		let within_y = point.y >= self.from.y.min( self.to.y ) && point.y <= self.from.y.max( self.to.y );

		within_x && within_y
	}
}
