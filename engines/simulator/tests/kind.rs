use simulator::{ Error, Kind };


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
fn it_reads_the_value_of_a_part()
{
	assert_eq!( Kind::Resistor { ohms : 220.0 }.value(), Some( 220.0 ) );
	assert_eq!( Kind::Battery { volts : 9.0 }.value(), Some( 9.0 ) );
	assert_eq!( Kind::Supply { volts : 3.3 }.value(), Some( 3.3 ) );
	assert_eq!( Kind::Capacitor { farads : 1e-6 }.value(), Some( 1e-6 ) );
	assert_eq!( Kind::Inductor { henries : 1e-3 }.value(), Some( 1e-3 ) );
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
}

#[test]
fn it_rejects_an_invalid_new_value()
{
	assert_eq!( Kind::Resistor { ohms : 1.0 }.with_value( 0.0 ), Err( Error::Resistance ) );
	assert_eq!( Kind::Battery { volts : 1.0 }.with_value( f64::NAN ), Err( Error::Voltage ) );
	assert_eq!( Kind::Supply { volts : 5.0 }.with_value( f64::NAN ), Err( Error::Voltage ) );
	assert_eq!( Kind::Capacitor { farads : 1.0 }.with_value( 0.0 ), Err( Error::Capacitance ) );
	assert_eq!( Kind::Inductor { henries : 1.0 }.with_value( -1.0 ), Err( Error::Inductance ) );
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
fn it_lists_the_voltages_that_control_each_part()
{
	assert_eq!( Kind::Diode.controls(), &[ ( 0, 1 ) ] );
	assert_eq!( Kind::Npn.controls(), &[ ( 0, 2 ), ( 0, 1 ) ] );
	assert_eq!( Kind::Pnp.controls(), &[ ( 2, 0 ), ( 1, 0 ) ] );
	assert_eq!( Kind::Nmos.controls(), &[ ( 0, 2 ), ( 1, 2 ) ] );
	assert_eq!( Kind::Pmos.controls(), &[ ( 2, 0 ), ( 2, 1 ) ] );
	assert!( Kind::Resistor { ohms : 1.0 }.controls().is_empty() );
	assert!( Kind::Wire.controls().is_empty() );
}
