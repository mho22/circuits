use std::f64::consts::SQRT_2;


const SATURATION : f64 = 1e-14;
const THERMAL : f64 = 0.025852;
const LEAKAGE : f64 = 1e-12;


pub fn linearize( voltage : f64 ) -> ( f64, f64 )
{
	let exponential = ( voltage / THERMAL ).exp();
	let current = SATURATION * ( exponential - 1.0 );
	let slope = SATURATION / THERMAL * exponential;

	( slope + LEAKAGE, slope * voltage - current )
}

pub fn limit( voltage : f64, previous : f64 ) -> f64
{
	let critical = THERMAL * ( THERMAL / ( SQRT_2 * SATURATION ) ).ln();

	if voltage <= critical || ( voltage - previous ).abs() <= 2.0 * THERMAL { return voltage; }

	if previous <= 0.0 { return THERMAL * ( voltage / THERMAL ).ln(); }

	let argument = 1.0 + ( voltage - previous ) / THERMAL;

	if argument <= 0.0 { return critical; }

	previous + THERMAL * argument.ln()
}
