use crate::{ Element, Kind, Operation, Point, Segment };


fn direction( segment : &Segment ) -> ( i32, i32 )
{
	( ( segment.to.x - segment.from.x ).signum(), ( segment.to.y - segment.from.y ).signum() )
}

fn position( segment : &Segment, point : Point ) -> i32
{
	let ( dx, dy ) = direction( segment );

	if dx != 0 { return ( point.x - segment.from.x ) * dx; }

	( point.y - segment.from.y ) * dy
}

fn point( segment : &Segment, position : i32 ) -> Point
{
	let ( dx, dy ) = direction( segment );

	Point { x : segment.from.x + dx * position, y : segment.from.y + dy * position }
}

fn aligned( segment : &Segment, point : Point ) -> bool
{
	( segment.to.x - segment.from.x ) * ( point.y - segment.from.y ) == ( segment.to.y - segment.from.y ) * ( point.x - segment.from.x )
}


pub fn place( elements : &[ Element ], kind : Kind, segment : Segment ) -> Vec<Operation>
{
	let part = Operation::Add { kind, segment, mirrored : false };

	if matches!( kind, Kind::Wire | Kind::Ground ) { return vec![ part ]; }

	let mut operations = Vec::new();

	for wire in elements.iter().filter( | element | element.kind == Kind::Wire )
	{
		if ! aligned( &wire.segment, segment.from ) || ! aligned( &wire.segment, segment.to ) { continue; }

		let length = wire.segment.length();
		let ( a, b ) = ( position( &wire.segment, segment.from ), position( &wire.segment, segment.to ) );
		let low = a.min( b ).max( 0 );
		let high = a.max( b ).min( length );

		if low >= high { continue; }

		operations.push( Operation::Delete { id : wire.id } );

		if low > 0 { operations.push( Operation::Add { kind : Kind::Wire, segment : Segment { from : wire.segment.from, to : point( &wire.segment, low ) }, mirrored : false } ); }

		if high < length { operations.push( Operation::Add { kind : Kind::Wire, segment : Segment { from : point( &wire.segment, high ), to : wire.segment.to }, mirrored : false } ); }
	}

	operations.push( part );

	operations
}
