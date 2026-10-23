use simulator::conduct;


const THRESHOLD : f64 = 1.5;
const GAIN : f64 = 0.02;
const MODULATION : f64 = 0.01;


fn current( gate : f64, drain : f64 ) -> f64
{
	conduct( gate, drain ).0
}


#[test]
fn it_conducts_nothing_below_the_threshold()
{
	assert_eq!( conduct( 1.0, 5.0 ), ( 0.0, 0.0, 0.0 ) );
	assert_eq!( conduct( THRESHOLD, 5.0 ), ( 0.0, 0.0, 0.0 ) );
}

#[test]
fn it_follows_the_square_law_in_saturation()
{
	for ( gate, drain ) in [ ( 2.0, 3.0 ), ( 3.5, 2.0 ), ( 5.0, 5.0 ) ]
	{
		assert!( ( current( gate, drain ) - GAIN / 2.0 * ( gate - THRESHOLD ).powi( 2 ) * ( 1.0 + MODULATION * drain ) ).abs() < 1e-15 );
		assert!( ( conduct( gate, drain ).2 - GAIN / 2.0 * ( gate - THRESHOLD ).powi( 2 ) * MODULATION ).abs() < 1e-15 );
	}
}

#[test]
fn it_acts_as_a_resistor_in_the_linear_region()
{
	assert!( ( current( 5.0, 0.5 ) - GAIN * ( 3.5 * 0.5 - 0.125 ) * ( 1.0 + MODULATION * 0.5 ) ).abs() < 1e-15 );
}

#[test]
fn it_meets_saturation_without_a_jump()
{
	assert!( ( current( 3.5, 2.0 - 1e-9 ) - current( 3.5, 2.0 ) ).abs() < 1e-9 );
}

#[test]
fn it_reverses_the_current_when_the_drain_is_below_the_source()
{
	assert!( ( current( 3.0, -0.5 ) + current( 3.5, 0.5 ) ).abs() < 1e-15 );
}

#[test]
fn it_follows_the_slopes_of_the_current()
{
	for ( gate, drain ) in [ ( 3.5, 3.0 ), ( 5.0, 0.5 ), ( 3.0, -0.5 ) ]
	{
		let ( _, transconductance, output ) = conduct( gate, drain );
		let along_gate = ( current( gate + 1e-6, drain ) - current( gate - 1e-6, drain ) ) / 2e-6;
		let along_drain = ( current( gate, drain + 1e-6 ) - current( gate, drain - 1e-6 ) ) / 2e-6;

		assert!( ( transconductance - along_gate ).abs() < 1e-8 );
		assert!( ( output - along_drain ).abs() < 1e-8 );
	}
}
