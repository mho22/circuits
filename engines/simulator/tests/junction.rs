use simulator::{ LIGHT, SILICON };


const SATURATION : f64 = 1e-14;
const THERMAL : f64 = 0.025852;
const LEAKAGE : f64 = 1e-12;


fn current( voltage : f64 ) -> f64
{
	SATURATION * ( ( voltage / THERMAL ).exp() - 1.0 ) + LEAKAGE * voltage
}


#[test]
fn it_conducts_almost_nothing_at_0_v()
{
	let ( conductance, source ) = SILICON.linearize( 0.0 );

	assert!( ( conductance - ( SATURATION / THERMAL + LEAKAGE ) ).abs() < 1e-20 );
	assert_eq!( source, 0.0 );
}

#[test]
fn it_matches_the_shockley_current_at_the_point_of_the_line()
{
	for voltage in [ -5.0, 0.3, 0.65, 0.7 ]
	{
		let ( conductance, source ) = SILICON.linearize( voltage );

		assert!( ( conductance * voltage - source - current( voltage ) ).abs() <= current( voltage ).abs() * 1e-9 + 1e-18 );
	}
}

#[test]
fn it_follows_the_slope_of_the_shockley_curve()
{
	let ( conductance, _ ) = SILICON.linearize( 0.65 );
	let slope = ( current( 0.65 + 1e-7 ) - current( 0.65 - 1e-7 ) ) / 2e-7;

	assert!( ( conductance - slope ).abs() < slope * 1e-6 );
}

#[test]
fn it_keeps_a_voltage_below_the_critical_voltage()
{
	assert_eq!( SILICON.limit( 0.5, 0.0 ), 0.5 );
	assert_eq!( SILICON.limit( -10.0, 0.6 ), -10.0 );
}

#[test]
fn it_keeps_a_small_step_above_the_critical_voltage()
{
	assert_eq!( SILICON.limit( 0.78, 0.75 ), 0.78 );
}

#[test]
fn it_shortens_a_large_step_from_a_negative_voltage()
{
	assert!( ( SILICON.limit( 5.0, 0.0 ) - THERMAL * ( 5.0 / THERMAL ).ln() ).abs() < 1e-12 );
}

#[test]
fn it_shortens_a_large_step_from_a_positive_voltage()
{
	assert!( ( SILICON.limit( 5.0, 0.7 ) - ( 0.7 + THERMAL * ( 1.0 + 4.3 / THERMAL ).ln() ) ).abs() < 1e-12 );
}

#[test]
fn it_falls_back_to_the_critical_voltage_on_a_large_drop()
{
	let critical = THERMAL * ( THERMAL / ( 2.0_f64.sqrt() * SATURATION ) ).ln();

	assert!( ( SILICON.limit( 0.8, 2.0 ) - critical ).abs() < 1e-12 );
}

#[test]
fn it_drops_about_1_9_v_across_a_light_junction_at_10_ma()
{
	let current = | voltage : f64 | LIGHT.saturation * ( ( voltage / LIGHT.thermal ).exp() - 1.0 );

	assert!( current( 1.85 ) < 0.01 && current( 1.95 ) > 0.01 );
}
