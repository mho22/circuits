use std::collections::HashMap;

use crate::union_find::UnionFind;
use crate::{ Element, Kind, Point, Segment };


pub struct Netlist
{
	pub points : Vec<Point>,
	pub nets : Vec<usize>,
	pub count : usize,
	pub earth : Option<usize>,
	pub edges : Vec<( usize, usize )>,
	pub links : Vec<( usize, usize )>,
	indices : HashMap<Point, usize>,
}

impl Netlist
{
	pub fn build( elements : &[ Element ] ) -> Netlist
	{
		let mut points = Vec::new();
		let mut indices = HashMap::new();

		for element in elements
		{
			for point in element.terminals()
			{
				indices.entry( point ).or_insert_with( || { points.push( point ); points.len() - 1 } );
			}
		}

		let mut edges = Vec::new();

		for element in elements.iter().filter( | element | element.kind == Kind::Wire )
		{
			let mut on : Vec<usize> = ( 0 .. points.len() ).filter( | &index | element.segment.contains( points[ index ] ) ).collect();

			on.sort_by_key( | &index | Segment { from : element.segment.from, to : points[ index ] }.length() );

			edges.extend( on.windows( 2 ).map( | pair | ( pair[ 0 ], pair[ 1 ] ) ) );
		}

		let links : Vec<( usize, usize )> = elements.iter().filter( | element | element.kind == Kind::Switch { closed : true } ).map( | element | ( indices[ &element.segment.from ], indices[ &element.segment.to ] ) ).collect();

		let mut forest = UnionFind::new( points.len() );

		for &( a, b ) in edges.iter().chain( &links ) { forest.union( a, b ); }

		let grounds : Vec<usize> = elements.iter().filter( | element | element.kind == Kind::Ground ).map( | element | indices[ &element.segment.from ] ).collect();

		for &ground in grounds.iter().skip( 1 ) { forest.union( grounds[ 0 ], ground ); }

		let mut roots = HashMap::new();

		let nets : Vec<usize> = ( 0 .. points.len() ).map( | index | { let count = roots.len(); *roots.entry( forest.find( index ) ).or_insert( count ) } ).collect();

		let earth = grounds.first().map( | &ground | nets[ ground ] );

		Netlist { points, nets, count : roots.len(), earth, edges, links, indices }
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
