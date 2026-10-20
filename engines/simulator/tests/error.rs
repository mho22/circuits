use simulator::Error;


#[test]
fn it_describes_each_error()
{
	assert_eq!( Error::Capacitance.to_string(), "A capacitance must be greater than 0 F." );
	assert_eq!( Error::Crooked.to_string(), "A part must run at 0° or 90°." );
	assert_eq!( Error::Empty.to_string(), "A part must have a length." );
	assert_eq!( Error::Inductance.to_string(), "An inductance must be greater than 0 H." );
	assert_eq!( Error::Layerless.to_string(), "No layer has this number." );
	assert_eq!( Error::Misplaced.to_string(), "A ground, a VCC, a pin or a symbol sits on one point." );
	assert_eq!( Error::Missing.to_string(), "No part has this id." );
	assert_eq!( Error::Recursive.to_string(), "A symbol cannot hold itself." );
	assert_eq!( Error::Resistance.to_string(), "A resistance must be greater than 0 Ω." );
	assert_eq!( Error::Rotation.to_string(), "A rotation is 0, 1, 2 or 3 quarter turns." );
	assert_eq!( Error::Singular.to_string(), "The circuit has no single solution: a battery is shorted, or batteries form a loop." );
	assert_eq!( Error::Switchless.to_string(), "This part is not a switch." );
	assert_eq!( Error::Taken.to_string(), "A part already has this id." );
	assert_eq!( Error::Unsettled.to_string(), "The diodes do not settle: no stable voltage found after 100 tries." );
	assert_eq!( Error::Valueless.to_string(), "This part has no value." );
	assert_eq!( Error::Voltage.to_string(), "A voltage must be a finite number." );
}
