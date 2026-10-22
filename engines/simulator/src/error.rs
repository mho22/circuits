use std::fmt;


#[derive( Clone, Copy, Debug, PartialEq, Eq )]
pub enum Error
{
	Capacitance,
	Color,
	Crooked,
	Empty,
	Flat,
	Frequency,
	Inductance,
	Inputless,
	Layerless,
	Misplaced,
	Missing,
	Oscillating,
	Pinless,
	Recursive,
	Resistance,
	Rotation,
	Singular,
	Straddled,
	Switchless,
	Taken,
	Unselected,
	Unsettled,
	Valueless,
	Voltage,
}

impl fmt::Display for Error
{
	fn fmt( &self, formatter : &mut fmt::Formatter ) -> fmt::Result
	{
		let message = match self
		{
			Error::Capacitance => "A capacitance must be greater than 0 F.",
			Error::Color => "An LED is red, green or blue.",
			Error::Crooked => "A part must run at 0° or 90°.",
			Error::Empty => "A part must have a length.",
			Error::Flat => "A shape must have a size.",
			Error::Frequency => "A frequency must be greater than 0 Hz.",
			Error::Inductance => "An inductance must be greater than 0 H.",
			Error::Inputless => "This part is not an input or a toggle.",
			Error::Layerless => "No layer has this number.",
			Error::Misplaced => "A ground, a VCC or a symbol sits on one point.",
			Error::Missing => "No part has this id.",
			Error::Oscillating => "A loop of gates switches too fast for the time step.",
			Error::Pinless => "This part is not an input or an output.",
			Error::Recursive => "A symbol cannot hold itself.",
			Error::Resistance => "A resistance must be greater than 0 Ω.",
			Error::Rotation => "A rotation is 0, 1, 2 or 3 quarter turns.",
			Error::Singular => "The circuit has no single solution: a battery is shorted, or batteries form a loop.",
			Error::Straddled => "A part crosses the edge of the selection.",
			Error::Switchless => "This part is not a switch.",
			Error::Taken => "A part already has this id.",
			Error::Unselected => "The selection holds no part.",
			Error::Unsettled => "The diodes do not settle: no stable voltage found after 100 tries.",
			Error::Valueless => "This part has no value.",
			Error::Voltage => "A voltage must be a finite number.",
		};

		formatter.write_str( message )
	}
}
