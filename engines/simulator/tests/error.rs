use simulator::Error;


#[test]
fn it_describes_each_error()
{
	assert_eq!( Error::Capacitance.to_string(), "A capacitance must be greater than 0 F." );
	assert_eq!( Error::Crooked.to_string(), "A part must run at 0°, 45° or 90°." );
	assert_eq!( Error::Empty.to_string(), "A part must have a length." );
	assert_eq!( Error::Inductance.to_string(), "An inductance must be greater than 0 H." );
	assert_eq!( Error::Misplaced.to_string(), "A ground sits on one point." );
	assert_eq!( Error::Missing.to_string(), "No part has this id." );
	assert_eq!( Error::Resistance.to_string(), "A resistance must be greater than 0 Ω." );
	assert_eq!( Error::Singular.to_string(), "The circuit has no single solution: a battery is shorted, or batteries form a loop." );
	assert_eq!( Error::Switchless.to_string(), "This part is not a switch." );
	assert_eq!( Error::Taken.to_string(), "A part already has this id." );
	assert_eq!( Error::Valueless.to_string(), "This part has no value." );
	assert_eq!( Error::Voltage.to_string(), "A voltage must be a finite number." );
}
