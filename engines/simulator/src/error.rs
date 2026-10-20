use std::fmt;


#[derive( Clone, Copy, Debug, PartialEq, Eq )]
pub enum Error
{
	Capacitance,
	Crooked,
	Empty,
	Inductance,
	Layerless,
	Misplaced,
	Missing,
	Recursive,
	Resistance,
	Rotation,
	Singular,
	Switchless,
	Taken,
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
			Error::Crooked => "A part must run at 0° or 90°.",
			Error::Empty => "A part must have a length.",
			Error::Inductance => "An inductance must be greater than 0 H.",
			Error::Layerless => "No layer has this number.",
			Error::Misplaced => "A ground, a VCC, a pin or a symbol sits on one point.",
			Error::Missing => "No part has this id.",
			Error::Recursive => "A symbol cannot hold itself.",
			Error::Resistance => "A resistance must be greater than 0 Ω.",
			Error::Rotation => "A rotation is 0, 1, 2 or 3 quarter turns.",
			Error::Singular => "The circuit has no single solution: a battery is shorted, or batteries form a loop.",
			Error::Switchless => "This part is not a switch.",
			Error::Taken => "A part already has this id.",
			Error::Unsettled => "The diodes do not settle: no stable voltage found after 100 tries.",
			Error::Valueless => "This part has no value.",
			Error::Voltage => "A voltage must be a finite number.",
		};

		formatter.write_str( message )
	}
}
