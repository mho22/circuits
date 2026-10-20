#[derive( Clone, Copy, Debug, PartialEq )]
pub struct Stamp
{
	pub across : ( usize, usize ),
	pub conductance : f64,
	pub source : f64,
	pub weights : [ f64; 3 ],
}

impl Stamp
{
	pub fn current( &self, potentials : &[ f64 ] ) -> f64
	{
		self.conductance * ( potentials[ self.across.0 ] - potentials[ self.across.1 ] ) - self.source
	}
}
