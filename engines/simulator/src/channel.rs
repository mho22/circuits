const THRESHOLD : f64 = 1.5;
const GAIN : f64 = 0.02;


pub fn conduct( gate : f64, drain : f64 ) -> ( f64, f64, f64 )
{
	if drain < 0.0
	{
		let ( current, transconductance, output ) = conduct( gate - drain, - drain );

		return ( - current, - transconductance, transconductance + output );
	}

	let overdrive = gate - THRESHOLD;

	if overdrive <= 0.0 { return ( 0.0, 0.0, 0.0 ); }

	if drain < overdrive { return ( GAIN * ( overdrive - drain / 2.0 ) * drain, GAIN * drain, GAIN * ( overdrive - drain ) ); }

	( GAIN / 2.0 * overdrive * overdrive, GAIN * overdrive, 0.0 )
}
