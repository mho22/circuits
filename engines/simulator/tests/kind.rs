use simulator::{ Error, Kind, LIGHTS, SILICON };


#[test]
fn it_accepts_valid_values()
{
	assert_eq!( Kind::Wire.check(), Ok( () ) );
	assert_eq!( Kind::Ground.check(), Ok( () ) );
	assert_eq!( Kind::Resistor { ohms : 1000.0 }.check(), Ok( () ) );
	assert_eq!( Kind::Battery { volts : -9.0 }.check(), Ok( () ) );
	assert_eq!( Kind::Battery { volts : 0.0 }.check(), Ok( () ) );
	assert_eq!( Kind::Capacitor { farads : 1e-6 }.check(), Ok( () ) );
	assert_eq!( Kind::Inductor { henries : 1e-3 }.check(), Ok( () ) );
	assert_eq!( Kind::Switch { closed : true }.check(), Ok( () ) );
	assert_eq!( Kind::Diode.check(), Ok( () ) );
	assert_eq!( Kind::Npn.check(), Ok( () ) );
	assert_eq!( Kind::Pnp.check(), Ok( () ) );
	assert_eq!( Kind::Nmos.check(), Ok( () ) );
	assert_eq!( Kind::Pmos.check(), Ok( () ) );
	assert_eq!( Kind::Supply { volts : 5.0 }.check(), Ok( () ) );
	assert_eq!( Kind::Input { high : false }.check(), Ok( () ) );
	assert_eq!( Kind::Output.check(), Ok( () ) );
	assert_eq!( Kind::Symbol { layer : 1, rotation : 3 }.check(), Ok( () ) );
	assert_eq!( Kind::Clock { hertz : 1e7 }.check(), Ok( () ) );
	assert_eq!( Kind::Led { color : 2 }.check(), Ok( () ) );
}

#[test]
fn it_rejects_an_led_color_past_blue()
{
	assert_eq!( Kind::Led { color : 3 }.check(), Err( Error::Color ) );
}

#[test]
fn it_rejects_a_frequency_that_is_not_positive_and_finite()
{
	assert_eq!( Kind::Clock { hertz : 0.0 }.check(), Err( Error::Frequency ) );
	assert_eq!( Kind::Clock { hertz : f64::INFINITY }.check(), Err( Error::Frequency ) );
	assert_eq!( Kind::Clock { hertz : f64::NAN }.check(), Err( Error::Frequency ) );
}

#[test]
fn it_rejects_a_rotation_past_three_quarter_turns()
{
	assert_eq!( Kind::Symbol { layer : 1, rotation : 4 }.check(), Err( Error::Rotation ) );
}

#[test]
fn it_rejects_a_resistance_that_is_not_positive_and_finite()
{
	assert_eq!( Kind::Resistor { ohms : 0.0 }.check(), Err( Error::Resistance ) );
	assert_eq!( Kind::Resistor { ohms : -1.0 }.check(), Err( Error::Resistance ) );
	assert_eq!( Kind::Resistor { ohms : f64::INFINITY }.check(), Err( Error::Resistance ) );
	assert_eq!( Kind::Resistor { ohms : f64::NAN }.check(), Err( Error::Resistance ) );
}

#[test]
fn it_rejects_a_capacitance_that_is_not_positive_and_finite()
{
	assert_eq!( Kind::Capacitor { farads : 0.0 }.check(), Err( Error::Capacitance ) );
	assert_eq!( Kind::Capacitor { farads : -1e-6 }.check(), Err( Error::Capacitance ) );
	assert_eq!( Kind::Capacitor { farads : f64::NAN }.check(), Err( Error::Capacitance ) );
}

#[test]
fn it_rejects_an_inductance_that_is_not_positive_and_finite()
{
	assert_eq!( Kind::Inductor { henries : 0.0 }.check(), Err( Error::Inductance ) );
	assert_eq!( Kind::Inductor { henries : f64::INFINITY }.check(), Err( Error::Inductance ) );
	assert_eq!( Kind::Inductor { henries : f64::NAN }.check(), Err( Error::Inductance ) );
}

#[test]
fn it_rejects_a_voltage_that_is_not_finite()
{
	assert_eq!( Kind::Battery { volts : f64::INFINITY }.check(), Err( Error::Voltage ) );
	assert_eq!( Kind::Battery { volts : f64::NAN }.check(), Err( Error::Voltage ) );
	assert_eq!( Kind::Supply { volts : f64::INFINITY }.check(), Err( Error::Voltage ) );
}

#[test]
fn it_accepts_a_slider_voltage_from_0_v_to_5_v_only()
{
	for volts in [ 0.0, 2.5, 5.0 ] { assert_eq!( Kind::Slider { volts }.check(), Ok( () ) ); }

	for volts in [ -0.1, 5.1, f64::NAN, f64::INFINITY ] { assert_eq!( Kind::Slider { volts }.check(), Err( Error::Range ) ); }
}

#[test]
fn it_reads_the_value_of_a_part()
{
	assert_eq!( Kind::Resistor { ohms : 220.0 }.value(), Some( 220.0 ) );
	assert_eq!( Kind::Battery { volts : 9.0 }.value(), Some( 9.0 ) );
	assert_eq!( Kind::Supply { volts : 3.3 }.value(), Some( 3.3 ) );
	assert_eq!( Kind::Capacitor { farads : 1e-6 }.value(), Some( 1e-6 ) );
	assert_eq!( Kind::Inductor { henries : 1e-3 }.value(), Some( 1e-3 ) );
	assert_eq!( Kind::Clock { hertz : 1e7 }.value(), Some( 1e7 ) );
	assert_eq!( Kind::Slider { volts : 2.4 }.value(), Some( 2.4 ) );
	assert_eq!( Kind::Led { color : 1 }.value(), Some( 1.0 ) );
	assert_eq!( Kind::Wire.value(), None );
	assert_eq!( Kind::Ground.value(), None );
	assert_eq!( Kind::Switch { closed : true }.value(), None );
	assert_eq!( Kind::Diode.value(), None );
	assert_eq!( Kind::Npn.value(), None );
	assert_eq!( Kind::Nmos.value(), None );
}

#[test]
fn it_replaces_the_value_of_a_part()
{
	assert_eq!( Kind::Resistor { ohms : 1.0 }.with_value( 220.0 ), Ok( Kind::Resistor { ohms : 220.0 } ) );
	assert_eq!( Kind::Battery { volts : 1.0 }.with_value( 9.0 ), Ok( Kind::Battery { volts : 9.0 } ) );
	assert_eq!( Kind::Supply { volts : 5.0 }.with_value( 3.3 ), Ok( Kind::Supply { volts : 3.3 } ) );
	assert_eq!( Kind::Capacitor { farads : 1.0 }.with_value( 1e-6 ), Ok( Kind::Capacitor { farads : 1e-6 } ) );
	assert_eq!( Kind::Inductor { henries : 1.0 }.with_value( 1e-3 ), Ok( Kind::Inductor { henries : 1e-3 } ) );
	assert_eq!( Kind::Clock { hertz : 1.0 }.with_value( 6e7 ), Ok( Kind::Clock { hertz : 6e7 } ) );
	assert_eq!( Kind::Slider { volts : 0.0 }.with_value( 3.7 ), Ok( Kind::Slider { volts : 3.7 } ) );
	assert_eq!( Kind::Led { color : 0 }.with_value( 2.0 ), Ok( Kind::Led { color : 2 } ) );
}

#[test]
fn it_rejects_an_invalid_new_value()
{
	assert_eq!( Kind::Resistor { ohms : 1.0 }.with_value( 0.0 ), Err( Error::Resistance ) );
	assert_eq!( Kind::Battery { volts : 1.0 }.with_value( f64::NAN ), Err( Error::Voltage ) );
	assert_eq!( Kind::Supply { volts : 5.0 }.with_value( f64::NAN ), Err( Error::Voltage ) );
	assert_eq!( Kind::Capacitor { farads : 1.0 }.with_value( 0.0 ), Err( Error::Capacitance ) );
	assert_eq!( Kind::Inductor { henries : 1.0 }.with_value( -1.0 ), Err( Error::Inductance ) );
	assert_eq!( Kind::Clock { hertz : 1.0 }.with_value( 0.0 ), Err( Error::Frequency ) );
	assert_eq!( Kind::Slider { volts : 0.0 }.with_value( 6.0 ), Err( Error::Range ) );
	assert_eq!( Kind::Led { color : 0 }.with_value( 3.0 ), Err( Error::Color ) );
	assert_eq!( Kind::Led { color : 0 }.with_value( 0.5 ), Err( Error::Color ) );
	assert_eq!( Kind::Led { color : 0 }.with_value( -1.0 ), Err( Error::Color ) );
	assert_eq!( Kind::Led { color : 0 }.with_value( f64::NAN ), Err( Error::Color ) );
}

#[test]
fn it_rejects_a_value_for_a_part_without_one()
{
	assert_eq!( Kind::Wire.with_value( 1.0 ), Err( Error::Valueless ) );
	assert_eq!( Kind::Ground.with_value( 1.0 ), Err( Error::Valueless ) );
	assert_eq!( Kind::Switch { closed : false }.with_value( 1.0 ), Err( Error::Valueless ) );
	assert_eq!( Kind::Diode.with_value( 1.0 ), Err( Error::Valueless ) );
	assert_eq!( Kind::Pnp.with_value( 1.0 ), Err( Error::Valueless ) );
	assert_eq!( Kind::Pmos.with_value( 1.0 ), Err( Error::Valueless ) );
}

#[test]
fn it_opens_and_closes_a_switch()
{
	assert_eq!( Kind::Switch { closed : false }.with_closed( true ), Ok( Kind::Switch { closed : true } ) );
	assert_eq!( Kind::Switch { closed : true }.with_closed( false ), Ok( Kind::Switch { closed : false } ) );
}

#[test]
fn it_rejects_a_closed_state_for_a_part_that_is_not_a_switch()
{
	assert_eq!( Kind::Wire.with_closed( true ), Err( Error::Switchless ) );
	assert_eq!( Kind::Resistor { ohms : 1.0 }.with_closed( true ), Err( Error::Switchless ) );
}

#[test]
fn it_sets_the_level_of_an_input()
{
	assert_eq!( Kind::Input { high : false }.with_high( true ), Ok( Kind::Input { high : true } ) );
	assert_eq!( Kind::Input { high : true }.with_high( false ), Ok( Kind::Input { high : false } ) );
}

#[test]
fn it_sets_the_level_of_a_toggle()
{
	assert_eq!( Kind::Toggle { high : false }.with_high( true ), Ok( Kind::Toggle { high : true } ) );
	assert_eq!( Kind::Toggle { high : true }.with_high( false ), Ok( Kind::Toggle { high : false } ) );
}

#[test]
fn it_rejects_a_level_for_a_part_that_is_not_an_input()
{
	assert_eq!( Kind::Wire.with_high( true ), Err( Error::Inputless ) );
	assert_eq!( Kind::Output.with_high( true ), Err( Error::Inputless ) );
}

#[test]
fn it_tells_an_input_and_an_output_from_the_other_parts()
{
	assert!( Kind::Input { high : true }.pin() && Kind::Output.pin() );
	assert!( ! Kind::Wire.pin() && ! Kind::Supply { volts : 5.0 }.pin() && ! Kind::Toggle { high : true }.pin() && ! Kind::Led { color : 0 }.pin() && ! Kind::Symbol { layer : 1, rotation : 0 }.pin() );
}

#[test]
fn it_tells_an_input_an_output_and_a_symbol_from_the_other_parts_by_their_label()
{
	assert!( Kind::Input { high : true }.labeled() && Kind::Output.labeled() && Kind::Symbol { layer : 1, rotation : 0 }.labeled() );
	assert!( ! Kind::Wire.labeled() && ! Kind::Toggle { high : true }.labeled() && ! Kind::Led { color : 0 }.labeled() );
}

#[test]
fn it_gives_an_led_the_light_junction_of_its_color_and_every_other_part_the_silicon_junction()
{
	assert_eq!( [ 0, 1, 2 ].map( | color | Kind::Led { color }.junction() ), LIGHTS );
	assert_eq!( Kind::Diode.junction(), SILICON );
	assert_eq!( Kind::Npn.junction(), SILICON );
}

#[test]
fn it_lists_the_voltages_that_control_each_part()
{
	assert_eq!( Kind::Diode.controls(), &[ ( 0, 1 ) ] );
	assert_eq!( Kind::Led { color : 0 }.controls(), &[ ( 0, 1 ) ] );
	assert_eq!( Kind::Npn.controls(), &[ ( 0, 2 ), ( 0, 1 ) ] );
	assert_eq!( Kind::Pnp.controls(), &[ ( 2, 0 ), ( 1, 0 ) ] );
	assert_eq!( Kind::Nmos.controls(), &[ ( 0, 2 ), ( 1, 2 ) ] );
	assert_eq!( Kind::Pmos.controls(), &[ ( 2, 0 ), ( 2, 1 ) ] );
	assert!( Kind::Resistor { ohms : 1.0 }.controls().is_empty() );
	assert!( Kind::Wire.controls().is_empty() );
}
