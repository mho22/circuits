use crate::{ Branch, Point, Reading };


pub fn mix( from : f64, to : f64, weight : f64 ) -> f64
{
	if ! from.is_finite() { return to; }

	from + ( to - from ) * weight
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

	pub fn blend( &self, next : &Solution, weight : f64 ) -> Solution
	{
		if ! self.fits( next ) { return next.clone(); }

		let values = | a : &[ f64 ], b : &[ f64 ] | a.iter().zip( b ).map( | ( &a, &b ) | mix( a, b, weight ) ).collect();
		let readings = self.readings.iter().zip( &next.readings ).map( | ( a, b ) | Reading { potentials : values( &a.potentials, &b.potentials ), currents : values( &a.currents, &b.currents ) } ).collect();
		let branches = self.branches.iter().zip( &next.branches ).map( | ( a, b ) | Branch { segment : b.segment, voltage : mix( a.voltage, b.voltage, weight ), current : mix( a.current, b.current, weight ) } ).collect();

		Solution { readings, branches }
	}
}
