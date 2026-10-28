use std::cmp::Ordering;
use std::sync::Arc;

use crate::Error;


const TOLERANCE : f64 = 1e-12;
const RATIO : f64 = 0.1;
const NONE : usize = usize::MAX;
const LONG : usize = 32;
const NEAR : f64 = 1e-6;
const ACCURATE : f64 = 1e-14;
const REFINEMENTS : usize = 3;


#[derive( Clone )]
struct Cell
{
	column : usize,
	value : f64,
	magnitude : f64,
}


#[derive( Debug )]
struct Step
{
	column : usize,
	pivot : usize,
	candidates : Vec<( usize, usize, usize, bool )>,
	tail : Vec<( usize, usize )>,
	targets : Vec<( usize, usize, Vec<usize> )>,
}


#[derive( Debug )]
struct Elimination
{
	cells : Vec<usize>,
	count : usize,
	steps : Vec<Step>,
}


#[derive( Debug )]
pub struct Factors
{
	size : usize,
	entries : Vec<( usize, usize, f64 )>,
	lower : Vec<( usize, usize, f64 )>,
	pivots : Vec<( usize, usize, f64, usize )>,
	upper : Vec<( usize, f64 )>,
	elimination : Arc<Elimination>,
}

impl Factors
{
	pub fn new( size : usize, entries : &[ ( usize, usize, f64 ) ], order : &[ usize ] ) -> Result<Factors, Error>
	{
		let mut rows : Vec<Vec<Cell>> = vec![ Vec::new(); size ];
		let mut columns : Vec<Vec<( usize, usize )>> = vec![ Vec::new(); size ];
		let mut places = vec![ NONE; size ];
		let mut indexes : Vec<Vec<usize>> = vec![ Vec::new(); size ];
		let mut starts = vec![ 0; size + 1 ];

		for &( row, _, _ ) in entries { starts[ row + 1 ] += 1; }

		for row in 0 .. size { starts[ row + 1 ] += starts[ row ]; }

		let mut sorted = vec![ 0; entries.len() ];
		let mut slots = vec![ NONE; entries.len() ];
		let mut owners = vec![ ( 0, 0 ); entries.len() ];
		let mut next = starts.clone();

		for ( index, &( row, _, _ ) ) in entries.iter().enumerate()
		{
			sorted[ next[ row ] ] = index;
			next[ row ] += 1;
		}

		for ( row, cells ) in rows.iter_mut().enumerate()
		{
			for &index in &sorted[ starts[ row ] .. starts[ row + 1 ] ]
			{
				let ( _, column, value ) = entries[ index ];

				if places[ column ] == NONE
				{
					places[ column ] = cells.len();
					slots[ index ] = cells.len();
					cells.push( Cell { column, value : 0.0, magnitude : 0.0 } );
				}

				cells[ places[ column ] ].value += value;
				cells[ places[ column ] ].magnitude += value.abs();
				owners[ index ] = ( row, places[ column ] );
			}

			for cell in cells.iter() { places[ cell.column ] = NONE; }
		}

		for ( &( row, column, _ ), &slot ) in entries.iter().zip( &slots )
		{
			if slot != NONE { columns[ column ].push( ( row, slot ) ); }
		}

		let mut ranks = vec![ 0; size ];
		let mut done = vec![ false; size ];
		let mut lower = Vec::new();
		let mut pivots = Vec::with_capacity( size );
		let mut upper = Vec::new();
		let mut steps = Vec::with_capacity( size );

		for ( rank, &column ) in order.iter().enumerate() { ranks[ column ] = rank; }

		for ( rank, &column ) in order.iter().enumerate()
		{
			let live : Vec<( usize, usize )> = std::mem::take( &mut columns[ column ] ).into_iter().filter( | &( row, _ ) | ! done[ row ] ).collect();
			let quiet = | &( row, slot ) : &( usize, usize ) | rows[ row ][ slot ].value.abs() <= rows[ row ][ slot ].magnitude * TOLERANCE;
			let candidates = live.iter().map( | &( row, slot ) | ( row, slot, rows[ row ].len(), ! quiet( &( row, slot ) ) ) ).collect();
			let ( noise, active ) : ( Vec<_>, Vec<_> ) = live.into_iter().partition( quiet );
			let largest = active.iter().map( | &( row, slot ) | rows[ row ][ slot ].value.abs() ).max_by( f64::total_cmp ).unwrap_or( 0.0 );
			let ( pivot, head ) = active.iter().copied().filter( | &( row, slot ) | rows[ row ][ slot ].value.abs().partial_cmp( &( RATIO * largest ) ) != Some( Ordering::Less ) ).min_by_key( | &( row, _ ) | rows[ row ].len() ).ok_or( Error::Singular )?;
			let divisor = rows[ pivot ][ head ].value;

			for &( row, slot ) in &noise { rows[ row ][ slot ].value = 0.0; }

			done[ pivot ] = true;
			pivots.push( ( pivot, column, divisor, upper.len() ) );

			let tail : Vec<( usize, f64 )> = rows[ pivot ].iter().filter( | cell | ranks[ cell.column ] > rank ).map( | cell | ( cell.column, cell.value ) ).collect();
			let places_of_tail = rows[ pivot ].iter().enumerate().filter( | ( _, cell ) | ranks[ cell.column ] > rank ).map( | ( place, cell ) | ( cell.column, place ) ).collect();
			let mut targets = Vec::with_capacity( active.len() );

			upper.extend_from_slice( &tail );

			for &( row, slot ) in active.iter().filter( | &&( row, _ ) | row != pivot )
			{
				let cells = &mut rows[ row ];
				let index = &mut indexes[ row ];
				let factor = cells[ slot ].value / divisor;

				cells[ slot ].value = 0.0;

				if index.is_empty() && cells.len() > LONG { index.resize( size, NONE ); for ( place, cell ) in cells.iter().enumerate() { index[ cell.column ] = place; } }

				let scattered = index.is_empty();
				let lookup = if scattered { for ( place, cell ) in cells.iter().enumerate() { places[ cell.column ] = place; } &mut places } else { index };
				let mut reached = Vec::with_capacity( tail.len() );

				for &( other, value ) in &tail
				{
					if lookup[ other ] == NONE
					{
						lookup[ other ] = cells.len();
						columns[ other ].push( ( row, cells.len() ) );
						cells.push( Cell { column : other, value : 0.0, magnitude : 0.0 } );
					}

					let change = factor * value;
					let cell = &mut cells[ lookup[ other ] ];

					cell.value -= change;
					cell.magnitude = cell.magnitude.max( change.abs() );
					reached.push( lookup[ other ] );
				}

				if scattered { for cell in cells.iter() { lookup[ cell.column ] = NONE; } }

				lower.push( ( row, pivot, factor ) );
				targets.push( ( row, slot, reached ) );
			}

			steps.push( Step { column, pivot, candidates, tail : places_of_tail, targets } );
		}

		let mut offsets = vec![ 0; size + 1 ];

		for ( row, cells ) in rows.iter().enumerate() { offsets[ row + 1 ] = offsets[ row ] + cells.len(); }

		for step in &mut steps
		{
			for candidate in &mut step.candidates { candidate.1 += offsets[ candidate.0 ]; }

			for place in &mut step.tail { place.1 += offsets[ step.pivot ]; }

			for ( row, slot, reached ) in &mut step.targets
			{
				*slot += offsets[ *row ];

				for place in reached.iter_mut() { *place += offsets[ *row ]; }
			}
		}

		let cells = owners.iter().map( | &( row, place ) | offsets[ row ] + place ).collect();
		let elimination = Arc::new( Elimination { cells, count : offsets[ size ], steps } );

		Ok( Factors { size, entries : entries.to_vec(), lower, pivots, upper, elimination } )
	}

	pub fn replay( &self, entries : &[ ( usize, usize, f64 ) ] ) -> Option<Factors>
	{
		let same = | ( &( row, column, _ ), &( other_row, other_column, _ ) ) : ( &( usize, usize, f64 ), &( usize, usize, f64 ) ) | row == other_row && column == other_column;

		if self.entries.len() != entries.len() || ! self.entries.iter().zip( entries ).all( same ) { return None; }

		let elimination = &self.elimination;
		let mut values = vec![ 0.0; elimination.count ];
		let mut magnitudes = vec![ 0.0; elimination.count ];
		let mut lower = Vec::with_capacity( self.lower.len() );
		let mut pivots = Vec::with_capacity( self.pivots.len() );
		let mut upper = Vec::with_capacity( self.upper.len() );

		for ( &( _, _, value ), &cell ) in entries.iter().zip( &elimination.cells )
		{
			values[ cell ] += value;
			magnitudes[ cell ] += value.abs();
		}

		for step in &elimination.steps
		{
			if step.candidates.iter().any( | &( _, slot, _, active ) | ( values[ slot ].abs() <= magnitudes[ slot ] * TOLERANCE ) == active ) { return None; }

			let largest = step.candidates.iter().filter( | candidate | candidate.3 ).map( | candidate | values[ candidate.1 ].abs() ).max_by( f64::total_cmp ).unwrap_or( 0.0 );
			let chosen = step.candidates.iter().filter( | candidate | candidate.3 && values[ candidate.1 ].abs().partial_cmp( &( RATIO * largest ) ) != Some( Ordering::Less ) ).min_by_key( | candidate | candidate.2 )?;

			if chosen.0 != step.pivot { return None; }

			let divisor = values[ chosen.1 ];
			let start = upper.len();

			for candidate in step.candidates.iter().filter( | candidate | ! candidate.3 ) { values[ candidate.1 ] = 0.0; }

			pivots.push( ( step.pivot, step.column, divisor, start ) );
			upper.extend( step.tail.iter().map( | &( column, slot ) | ( column, values[ slot ] ) ) );

			for ( row, slot, reached ) in &step.targets
			{
				let factor = values[ *slot ] / divisor;

				values[ *slot ] = 0.0;

				for ( &( _, value ), &target ) in upper[ start .. ].iter().zip( reached )
				{
					let change = factor * value;

					values[ target ] -= change;
					magnitudes[ target ] = magnitudes[ target ].max( change.abs() );
				}

				lower.push( ( *row, step.pivot, factor ) );
			}
		}

		Some( Factors { size : self.size, entries : entries.to_vec(), lower, pivots, upper, elimination : Arc::clone( elimination ) } )
	}

	pub fn solve( &self, mut rhs : Vec<f64> ) -> Vec<f64>
	{
		for &( row, pivot, factor ) in &self.lower { rhs[ row ] -= factor * rhs[ pivot ]; }

		let mut solution = vec![ 0.0; self.size ];

		for ( index, &( pivot, column, divisor, start ) ) in self.pivots.iter().enumerate().rev()
		{
			let end = self.pivots.get( index + 1 ).map_or( self.upper.len(), | next | next.3 );
			let known : f64 = self.upper[ start .. end ].iter().map( | &( other, value ) | value * solution[ other ] ).sum();

			solution[ column ] = ( rhs[ pivot ] - known ) / divisor;
		}

		solution
	}

	pub fn near( &self, entries : &[ ( usize, usize, f64 ) ] ) -> bool
	{
		let close = | ( &( row, column, value ), &( other_row, other_column, other ) ) : ( &( usize, usize, f64 ), &( usize, usize, f64 ) ) | row == other_row && column == other_column && ( value - other ).abs() <= NEAR * value.abs().max( other.abs() );

		self.entries.len() == entries.len() && self.entries.iter().zip( entries ).all( close )
	}

	pub fn refine( &self, entries : &[ ( usize, usize, f64 ) ], rhs : &[ f64 ] ) -> Option<Vec<f64>>
	{
		let mut solution = self.solve( rhs.to_vec() );

		for _ in 0 ..= REFINEMENTS
		{
			let mut residual = rhs.to_vec();
			let mut scale : Vec<f64> = rhs.iter().map( | value | value.abs() ).collect();

			for &( row, column, value ) in entries
			{
				let product = value * solution[ column ];

				residual[ row ] -= product;
				scale[ row ] += product.abs();
			}

			if residual.iter().zip( &scale ).all( | ( residual, scale ) | residual.abs() <= ACCURATE * scale ) { return Some( solution ); }

			for ( value, correction ) in solution.iter_mut().zip( self.solve( residual ) ) { *value += correction; }
		}

		None
	}
}
