use crate::{ Branch, Point, Reading };


const SETTLED : f64 = 1e-15;


pub fn mix( from : f64, to : f64, weight : f64 ) -> f64
{
	if ! from.is_finite() { return to; }

	let next = from + ( to - from ) * weight;

	if ( to - next ).abs() < SETTLED { return to; }

	next
}

fn largest( peak : f64, value : f64 ) -> f64
{
	peak.max( value.abs() )
}


#[derive( Clone, Debug, Default, PartialEq )]
pub struct Solution
{
	pub readings : Vec<Reading>,
	pub branches : Vec<Branch>,
}

impl Solution
{
	pub fn branch_at( &self, point : Point ) -> Option<usize>
	{
		self.branches.iter().position( | branch | branch.segment.contains( point ) )
	}

	pub fn fits( &self, other : &Solution ) -> bool
	{
		let readings = self.readings.len() == other.readings.len() && self.readings.iter().zip( &other.readings ).all( | ( a, b ) | a.potentials.len() == b.potentials.len() && a.currents.len() == b.currents.len() );
		let branches = self.branches.len() == other.branches.len() && self.branches.iter().zip( &other.branches ).all( | ( a, b ) | a.segment == b.segment );

		readings && branches
	}

	pub fn peaks( &self ) -> [ f64; 2 ]
	{
		let potential = self.readings.iter().flat_map( | reading | &reading.potentials ).fold( 0.0, | peak, value | largest( peak, *value ) );
		let current = self.readings.iter().flat_map( | reading | &reading.currents ).chain( self.branches.iter().map( | branch | &branch.current ) ).fold( 0.0, | peak, value | largest( peak, *value ) );

		[ potential, current ]
	}

	pub fn blend( &mut self, next : &Solution, weight : f64 )
	{
		if ! self.fits( next ) { self.clone_from( next ); return; }

		let values = | a : &mut [ f64 ], b : &[ f64 ] | for ( a, &b ) in a.iter_mut().zip( b ) { *a = mix( *a, b, weight ); };

		for ( a, b ) in self.readings.iter_mut().zip( &next.readings )
		{
			values( &mut a.potentials, &b.potentials );
			values( &mut a.currents, &b.currents );
		}

		for ( a, b ) in self.branches.iter_mut().zip( &next.branches )
		{
			a.voltage = mix( a.voltage, b.voltage, weight );
			a.current = mix( a.current, b.current, weight );
		}
	}
}
