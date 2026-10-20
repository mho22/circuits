use std::fmt;


#[derive( Clone, Copy, Debug, PartialEq, Eq )]
pub enum Error
{
	Crooked,
	Empty,
	Misplaced,
	Missing,
	Resistance,
	Singular,
	Valueless,
	Voltage,
}

impl fmt::Display for Error
{
	fn fmt( &self, formatter : &mut fmt::Formatter ) -> fmt::Result
	{
		let message = match self
		{
			Error::Crooked => "A part must run at 0°, 45° or 90°.",
			Error::Empty => "A part must have a length.",
			Error::Misplaced => "A ground sits on one point.",
			Error::Missing => "No part has this id.",
			Error::Resistance => "A resistance must be greater than 0 Ω.",
			Error::Singular => "The circuit has no single solution: a battery is shorted, or batteries form a loop.",
			Error::Valueless => "This part has no value.",
			Error::Voltage => "A voltage must be a finite number.",
		};

		formatter.write_str( message )
	}
}
