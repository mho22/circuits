use std::collections::HashMap;

use crate::contacts::terminals;
use crate::{ Document, Error, Hashing, Kind, Point, UnionFind };


#[derive( Clone, Debug, Default, PartialEq )]
pub struct Buses
{
	widths : HashMap<Point, usize, Hashing>,
}

impl Buses
{
	pub fn of( layers : &[ Document ], layer : usize ) -> Result<Buses, Error>
	{
		let elements = layers[ layer ].elements();
		let mut touches : HashMap<Point, Vec<usize>, Hashing> = HashMap::default();

		for element in elements.iter().filter( | element | element.kind != Kind::Wire )
		{
			let points = terminals( layers, element );
			let symbol = matches!( element.kind, Kind::Symbol { .. } );

			for ( index, &point ) in points.iter().enumerate()
			{
				if symbol && points[ .. index ].contains( &point ) { continue; }

				let width = if symbol { points.iter().filter( | &&other | other == point ).count() } else { 1 };

				touches.entry( point ).or_default().push( width );
			}
		}

		if touches.values().flatten().all( | &width | width == 1 ) { return Ok( Buses::default() ); }

		let mut points : Vec<Point> = touches.keys().copied().collect();
		let mut indices : HashMap<Point, usize, Hashing> = points.iter().enumerate().map( | ( index, &point ) | ( point, index ) ).collect();
		let mut edges = Vec::new();

		for element in elements.iter().filter( | element | element.kind == Kind::Wire )
		{
			let [ from, to ] = [ element.segment.from, element.segment.to ].map( | point | *indices.entry( point ).or_insert_with( || { points.push( point ); points.len() - 1 } ) );

			edges.push( ( from, to ) );
		}

		let mut forest = UnionFind::new( points.len() );

		for &( from, to ) in &edges { forest.union( from, to ); }

		let mut nets : HashMap<usize, Vec<usize>, Hashing> = HashMap::default();

		for ( point, widths ) in &touches { nets.entry( forest.find( indices[ point ] ) ).or_default().extend( widths ); }

		let mut widths : HashMap<usize, usize, Hashing> = HashMap::default();

		for ( &root, net ) in &nets
		{
			let width = net.iter().copied().max().unwrap_or( 1 );

			if width == 1 { continue; }

			if net.iter().any( | &other | other != width ) { return Err( Error::Bus ); }

			widths.insert( root, width );
		}

		Ok( Buses { widths : points.iter().enumerate().filter_map( | ( index, &point ) | widths.get( &forest.find( index ) ).map( | &width | ( point, width ) ) ).collect() } )
	}

	pub fn width( &self, point : Point ) -> usize
	{
		self.widths.get( &point ).copied().unwrap_or( 1 )
	}
}
