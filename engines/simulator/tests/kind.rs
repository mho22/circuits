use simulator::{ Error, Kind };


#[test]
fn it_accepts_valid_values()
{
	assert_eq!( Kind::Wire.check(), Ok( () ) );
	assert_eq!( Kind::Ground.check(), Ok( () ) );
	assert_eq!( Kind::Resistor { ohms : 1000.0 }.check(), Ok( () ) );
	assert_eq!( Kind::Battery { volts : -9.0 }.check(), Ok( () ) );
	assert_eq!( Kind::Battery { volts : 0.0 }.check(), Ok( () ) );
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
fn it_rejects_a_voltage_that_is_not_finite()
{
	assert_eq!( Kind::Battery { volts : f64::INFINITY }.check(), Err( Error::Voltage ) );
	assert_eq!( Kind::Battery { volts : f64::NAN }.check(), Err( Error::Voltage ) );
}

#[test]
fn it_replaces_the_value_of_a_part()
{
	assert_eq!( Kind::Resistor { ohms : 1.0 }.with_value( 220.0 ), Ok( Kind::Resistor { ohms : 220.0 } ) );
	assert_eq!( Kind::Battery { volts : 1.0 }.with_value( 9.0 ), Ok( Kind::Battery { volts : 9.0 } ) );
}

#[test]
fn it_rejects_an_invalid_new_value()
{
	assert_eq!( Kind::Resistor { ohms : 1.0 }.with_value( 0.0 ), Err( Error::Resistance ) );
	assert_eq!( Kind::Battery { volts : 1.0 }.with_value( f64::NAN ), Err( Error::Voltage ) );
}

#[test]
fn it_rejects_a_value_for_a_part_without_one()
{
	assert_eq!( Kind::Wire.with_value( 1.0 ), Err( Error::Valueless ) );
	assert_eq!( Kind::Ground.with_value( 1.0 ), Err( Error::Valueless ) );
}
