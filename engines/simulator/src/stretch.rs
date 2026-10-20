use crate::{ Element, Kind, Operation, Point, Segment };


fn overlaps( piece : Segment, part : Segment ) -> bool
{
	let step = Point { x : ( piece.to.x - piece.from.x ).signum(), y : ( piece.to.y - piece.from.y ).signum() };
	let at = | index : i32 | Point { x : piece.from.x + step.x * index, y : piece.from.y + step.y * index };

	( 0 .. piece.length() ).any( | index | part.contains( at( index ) ) && part.contains( at( index + 1 ) ) )
}

fn bend( start : Point, end : Point, horizontal : bool, part : Segment ) -> Vec<Segment>
{
	let across = Point { x : end.x, y : start.y };
	let along = Point { x : start.x, y : end.y };
	let ( preferred, other ) = if horizontal { ( across, along ) } else { ( along, across ) };
	let pieces = | corner : Point | [ Segment { from : start, to : corner }, Segment { from : corner, to : end } ].into_iter().filter( | piece | ! piece.is_empty() ).collect::<Vec<_>>();
	let crossing = | corner : Point | pieces( corner ).into_iter().any( | piece | overlaps( piece, part ) );
	let corner = if crossing( preferred ) && ! crossing( other ) { other } else { preferred };

	pieces( corner )
}

fn tapped( elements : &[ Element ], part : &Element, point : Point ) -> bool
{
	elements.iter().filter( | element | element.id != part.id ).any( | element |
	{
		if element.kind != Kind::Wire { return element.terminals().contains( &point ); }

		element.segment.contains( point ) && point != element.segment.from && point != element.segment.to
	} )
}


pub fn stretch( elements : &[ Element ], before : &Element, after : &Element ) -> Vec<Operation>
{
	let ( old, new ) = ( before.terminals(), after.terminals() );
	let follow = | point : Point | old.iter().position( | &terminal | terminal == point ).map_or( point, | index | new[ index ] );
	let mut operations = Vec::new();
	let mut pieces = Vec::new();

	for wire in elements.iter().filter( | element | element.kind == Kind::Wire && element.id != before.id )
	{
		let Segment { from, to } = wire.segment;
		let ( start, end ) = ( follow( from ), follow( to ) );

		if start == from && end == to { continue; }

		operations.push( Operation::Delete { id : wire.id } );
		pieces.extend( bend( start, end, ( from.y == to.y ) == ( start == from ), after.segment ) );
	}

	let mut pairs : Vec<( Point, Point )> = old.iter().copied().zip( new.iter().copied() ).collect();

	pairs.dedup();

	for ( terminal, moved ) in pairs
	{
		if terminal == moved || ! tapped( elements, before, terminal ) { continue; }

		for piece in bend( terminal, moved, true, after.segment )
		{
			if pieces.iter().any( | &laid : &Segment | laid == piece || laid == Segment { from : piece.to, to : piece.from } ) { continue; }

			pieces.push( piece );
		}
	}

	operations.extend( pieces.into_iter().map( | segment | Operation::Add { kind : Kind::Wire, segment, mirrored : false } ) );

	operations
}
