use std::f64::consts::SQRT_2;


const LEAKAGE : f64 = 1e-12;


#[derive( Clone, Copy, Debug, PartialEq )]
pub struct Junction
{
	pub saturation : f64,
	pub thermal : f64,
}

pub const SILICON : Junction = Junction { saturation : 1e-14, thermal : 0.025852 };
pub const LIGHTS : [ Junction; 3 ] = [
	Junction { saturation : 1e-18, thermal : 0.051704 },
	Junction { saturation : 1e-19, thermal : 0.051704 },
	Junction { saturation : 1e-27, thermal : 0.051704 },
];

impl Junction
{
	pub fn linearize( &self, voltage : f64 ) -> ( f64, f64 )
	{
		let exponential = ( voltage / self.thermal ).exp();
		let current = self.saturation * ( exponential - 1.0 );
		let slope = self.saturation / self.thermal * exponential;

		( slope + LEAKAGE, slope * voltage - current )
	}

	pub fn limit( &self, voltage : f64, previous : f64 ) -> f64
	{
		let thermal = self.thermal;
		let critical = thermal * ( thermal / ( SQRT_2 * self.saturation ) ).ln();

		if voltage <= critical || ( voltage - previous ).abs() <= 2.0 * thermal { return voltage; }

		if previous <= 0.0 { return thermal * ( voltage / thermal ).ln(); }

		let argument = 1.0 + ( voltage - previous ) / thermal;

		if argument <= 0.0 { return critical; }

		previous + thermal * argument.ln()
	}
}
