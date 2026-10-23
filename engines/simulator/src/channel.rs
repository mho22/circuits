const THRESHOLD : f64 = 1.5;
const GAIN : f64 = 0.02;
const MODULATION : f64 = 0.01;


pub fn conduct( gate : f64, drain : f64 ) -> ( f64, f64, f64 )
{
	if drain < 0.0
	{
		let ( current, transconductance, output ) = conduct( gate - drain, - drain );

		return ( - current, - transconductance, transconductance + output );
	}

	let overdrive = gate - THRESHOLD;

	if overdrive <= 0.0 { return ( 0.0, 0.0, 0.0 ); }

	let modulation = 1.0 + MODULATION * drain;

	if drain < overdrive
	{
		let current = GAIN * ( overdrive - drain / 2.0 ) * drain;

		return ( current * modulation, GAIN * drain * modulation, GAIN * ( overdrive - drain ) * modulation + current * MODULATION );
	}

	let current = GAIN / 2.0 * overdrive * overdrive;

	( current * modulation, GAIN * overdrive * modulation, current * MODULATION )
}
