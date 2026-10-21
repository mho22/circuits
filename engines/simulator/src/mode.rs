use std::fmt;

use crate::{ Device, Plan };


#[derive( Clone, Copy, Debug, PartialEq, Eq )]
pub enum Mode
{
	Analog,
	Mixed,
	Digital,
}

impl Mode
{
	pub fn of( devices : &[ Device ], plan : &Plan ) -> Mode
	{
		if devices.is_empty() { return Mode::Analog; }

		if ! plan.analog.is_empty() { return Mode::Mixed; }

		Mode::Digital
	}
}

impl fmt::Display for Mode
{
	fn fmt( &self, formatter : &mut fmt::Formatter ) -> fmt::Result
	{
		let name = match self
		{
			Mode::Analog => "Analog",
			Mode::Mixed => "Mixed",
			Mode::Digital => "Digital",
		};

		formatter.write_str( name )
	}
}
