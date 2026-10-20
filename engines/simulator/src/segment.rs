use crate::Point;


#[derive( Clone, Copy, Debug, PartialEq, Eq )]
pub struct Segment
{
	pub from : Point,
	pub to : Point,
}

impl Segment
{
	pub fn snap( from : Point, cursor : Point ) -> Point
	{
		if ( cursor.x - from.x ).abs() >= ( cursor.y - from.y ).abs() { return Point { x : cursor.x, y : from.y }; }

		Point { x : from.x, y : cursor.y }
	}

	pub fn turned( &self ) -> Segment
	{
		let step = Point { x : ( self.to.x - self.from.x ).signum(), y : ( self.to.y - self.from.y ).signum() };
		let half = self.length() / 2;
		let center = Point { x : self.from.x + step.x * half, y : self.from.y + step.y * half };
		let turn = | point : Point | Point { x : center.x - ( point.y - center.y ), y : center.y + point.x - center.x };

		Segment { from : turn( self.from ), to : turn( self.to ) }
	}

	pub fn mirrored( &self, vertical : bool ) -> Segment
	{
		let flip = | point : Point | if vertical { Point { x : point.x, y : self.from.y + self.to.y - point.y } } else { Point { x : self.from.x + self.to.x - point.x, y : point.y } };

		Segment { from : flip( self.from ), to : flip( self.to ) }
	}

	pub fn shifted( &self, dx : i32, dy : i32 ) -> Segment
	{
		let shift = | point : Point | Point { x : point.x + dx, y : point.y + dy };

		Segment { from : shift( self.from ), to : shift( self.to ) }
	}

	pub fn is_empty( &self ) -> bool
	{
		self.from == self.to
	}

	pub fn is_straight( &self ) -> bool
	{
		self.from.x == self.to.x || self.from.y == self.to.y
	}

	pub fn length( &self ) -> i32
	{
		( self.to.x - self.from.x ).abs().max( ( self.to.y - self.from.y ).abs() )
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
