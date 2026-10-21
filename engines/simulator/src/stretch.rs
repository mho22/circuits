use crate::{ Element, Kind, Operation, Point, Segment };


fn covers( zone : Segment, point : Point ) -> bool
{
	point.x >= zone.from.x.min( zone.to.x ) && point.x <= zone.from.x.max( zone.to.x ) && point.y >= zone.from.y.min( zone.to.y ) && point.y <= zone.from.y.max( zone.to.y )
}

fn overlaps( piece : Segment, zone : Segment ) -> bool
{
	let step = Point { x : ( piece.to.x - piece.from.x ).signum(), y : ( piece.to.y - piece.from.y ).signum() };
	let at = | index : i32 | Point { x : piece.from.x + step.x * index, y : piece.from.y + step.y * index };

	( 0 .. piece.length() ).any( | index | covers( zone, at( index ) ) && covers( zone, at( index + 1 ) ) )
}

fn collinear( piece : Segment, other : Segment ) -> bool
{
	let span = | a : i32, b : i32 | ( a.min( b ), a.max( b ) );
	let shared = | ( low, high ) : ( i32, i32 ), ( from, to ) : ( i32, i32 ) | low.max( from ) < high.min( to );

	if piece.from.y == piece.to.y && other.from.y == other.to.y && piece.from.y == other.from.y { return shared( span( piece.from.x, piece.to.x ), span( other.from.x, other.to.x ) ); }

	if piece.from.x == piece.to.x && other.from.x == other.to.x && piece.from.x == other.from.x { return shared( span( piece.from.y, piece.to.y ), span( other.from.y, other.to.y ) ); }

	false
}

fn detour( start : Point, end : Point, zone : Segment, laid : &[ Segment ] ) -> Option<Vec<Segment>>
{
	let low = Point { x : zone.from.x.min( zone.to.x ), y : zone.from.y.min( zone.to.y ) };
	let high = Point { x : zone.from.x.max( zone.to.x ), y : zone.from.y.max( zone.to.y ) };

	let approach = match end
	{
		Point { x, y } if x > high.x => Point { x : x + 1, y },
		Point { x, y } if x < low.x => Point { x : x - 1, y },
		Point { x, y } if y < low.y => Point { x, y : y - 1 },
		Point { x, y } if y > high.y => Point { x, y : y + 1 },
		_ => return None,
	};

	let lanes = [
		[ Point { x : start.x, y : low.y - 2 }, Point { x : approach.x, y : low.y - 2 } ],
		[ Point { x : start.x, y : high.y + 2 }, Point { x : approach.x, y : high.y + 2 } ],
		[ Point { x : low.x - 2, y : start.y }, Point { x : low.x - 2, y : approach.y } ],
		[ Point { x : high.x + 2, y : start.y }, Point { x : high.x + 2, y : approach.y } ],
	];

	lanes.into_iter().map( | [ first, second ] |
	{
		let points = [ start, first, second, approach, end ];

		points.windows( 2 ).map( | pair | Segment { from : pair[ 0 ], to : pair[ 1 ] } ).filter( | piece | ! piece.is_empty() ).collect::<Vec<_>>()
	} ).filter( | pieces | pieces.iter().all( | &piece | ! overlaps( piece, zone ) && ! laid.iter().any( | &other | collinear( piece, other ) ) ) ).min_by_key( | pieces | pieces.iter().map( | piece | piece.length() ).sum::<i32>() )
}

fn bend( start : Point, end : Point, horizontal : bool, zone : Segment, laid : &[ Segment ] ) -> Vec<Segment>
{
	let across = Point { x : end.x, y : start.y };
	let along = Point { x : start.x, y : end.y };
	let ( preferred, other ) = if horizontal { ( across, along ) } else { ( along, across ) };
	let pieces = | corner : Point | [ Segment { from : start, to : corner }, Segment { from : corner, to : end } ].into_iter().filter( | piece | ! piece.is_empty() ).collect::<Vec<_>>();
	let crossing = | corner : Point | pieces( corner ).into_iter().any( | piece | overlaps( piece, zone ) );

	if ! crossing( preferred ) { return pieces( preferred ); }

	if ! crossing( other ) { return pieces( other ); }

	detour( start, end, zone, laid ).unwrap_or_else( || pieces( preferred ) )
}

fn tapped( elements : &[ Element ], point : Point, terminals : &impl Fn( &Element ) -> Vec<Point> ) -> bool
{
	elements.iter().any( | element |
	{
		if element.kind != Kind::Wire { return terminals( element ).contains( &point ); }

		element.segment.contains( point ) && point != element.segment.from && point != element.segment.to
	} )
}


pub fn stretch( elements : &[ Element ], old : &[ Point ], new : &[ Point ], zone : Segment, terminals : impl Fn( &Element ) -> Vec<Point> ) -> Vec<Operation>
{
	let follow = | point : Point | old.iter().position( | &terminal | terminal == point ).map_or( point, | index | new[ index ] );
	let mut operations = Vec::new();
	let mut pieces = Vec::new();

	for wire in elements.iter().filter( | element | element.kind == Kind::Wire )
	{
		let Segment { from, to } = wire.segment;
		let ( start, end ) = ( follow( from ), follow( to ) );

		if start == from && end == to { continue; }

		operations.push( Operation::Delete { id : wire.id } );

		let bent = bend( start, end, ( from.y == to.y ) == ( start == from ), zone, &pieces );

		pieces.extend( bent );
	}

	let mut pairs : Vec<( Point, Point )> = old.iter().copied().zip( new.iter().copied() ).collect();

	pairs.dedup();

	for ( terminal, moved ) in pairs
	{
		if terminal == moved || ! tapped( elements, terminal, &terminals ) { continue; }

		for piece in bend( terminal, moved, true, zone, &pieces )
		{
			if pieces.iter().any( | &laid : &Segment | laid == piece || laid == Segment { from : piece.to, to : piece.from } ) { continue; }

			pieces.push( piece );
		}
	}

	operations.extend( pieces.into_iter().map( | segment | Operation::Add { kind : Kind::Wire, segment, mirrored : false } ) );

	operations
}
