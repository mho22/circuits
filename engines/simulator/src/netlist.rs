use std::collections::HashMap;

use crate::union_find::UnionFind;
use crate::{ Element, Hashing, Kind, Point };


#[derive( Clone, Debug, Default, PartialEq )]
pub struct Netlist
{
	pub points : Vec<Point>,
	pub nets : Vec<usize>,
	pub count : usize,
	pub earth : Option<usize>,
	pub edges : Vec<( usize, usize )>,
	pub links : Vec<( usize, usize )>,
	indices : HashMap<Point, usize, Hashing>,
}

impl Netlist
{
	pub fn build( elements : &[ Element ] ) -> Netlist
	{
		let mut points = Vec::new();
		let mut indices = HashMap::with_capacity_and_hasher( elements.len() * 3, Hashing::default() );

		for element in elements
		{
			for point in element.terminals()
			{
				indices.entry( point ).or_insert_with( || { points.push( point ); points.len() - 1 } );
			}
		}

		let edges : Vec<( usize, usize )> = elements.iter().filter( | element | element.kind == Kind::Wire ).map( | element | ( indices[ &element.segment.from ], indices[ &element.segment.to ] ) ).collect();

		let links : Vec<( usize, usize )> = elements.iter().filter( | element | element.kind == Kind::Switch { closed : true } ).map( | element | ( indices[ &element.segment.from ], indices[ &element.segment.to ] ) ).collect();

		let mut forest = UnionFind::new( points.len() );

		for &( a, b ) in edges.iter().chain( &links ) { forest.union( a, b ); }

		let grounds : Vec<usize> = elements.iter().filter( | element | element.kind == Kind::Ground ).map( | element | indices[ &element.segment.from ] ).collect();

		for &ground in grounds.iter().skip( 1 ) { forest.union( grounds[ 0 ], ground ); }

		let mut roots = vec![ usize::MAX; points.len() ];
		let mut count = 0;

		let nets : Vec<usize> = ( 0 .. points.len() ).map( | index |
		{
			let root = forest.find( index );

			if roots[ root ] == usize::MAX { roots[ root ] = count; count += 1; }

			roots[ root ]
		} ).collect();

		let supplied = elements.iter().any( | element | matches!( element.kind, Kind::Supply { .. } ) );
		let earth = grounds.first().map( | &ground | nets[ ground ] ).or_else( || supplied.then( || { count += 1; count - 1 } ) );

		Netlist { points, nets, count, earth, edges, links, indices }
	}

	pub fn index( &self, point : Point ) -> usize
	{
		self.indices[ &point ]
	}

	pub fn net( &self, point : Point ) -> usize
	{
		self.nets[ self.index( point ) ]
	}
}
