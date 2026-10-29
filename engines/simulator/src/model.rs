pub const THRESHOLD : f64 = 2.5;
pub const FLOOR : f64 = 1e-3;
pub const DELAY : f64 = 1e-9;


#[derive( Clone, Copy, Debug, PartialEq )]
pub struct Drive
{
	pub volts : f64,
	pub ohms : f64,
}

impl Drive
{
	pub fn high( &self ) -> bool
	{
		self.volts > THRESHOLD
	}
}


#[derive( Clone, Debug, PartialEq )]
pub struct Model
{
	pub rows : Vec<Vec<Drive>>,
	pub lights : Vec<Vec<f64>>,
	pub next : Vec<usize>,
	pub delay : f64,
}
