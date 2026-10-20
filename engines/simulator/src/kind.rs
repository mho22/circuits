use crate::Error;


#[derive( Clone, Copy, Debug, PartialEq )]
pub enum Kind
{
	Wire,
	Resistor { ohms : f64 },
	Battery { volts : f64 },
	Ground,
}

impl Kind
{
	pub fn check( &self ) -> Result<(), Error>
	{
		match *self
		{
			Kind::Resistor { ohms } if ! ( ohms.is_finite() && ohms > 0.0 ) => Err( Error::Resistance ),
			Kind::Battery { volts } if ! volts.is_finite() => Err( Error::Voltage ),
			_ => Ok( () ),
		}
	}

	pub fn with_value( &self, value : f64 ) -> Result<Kind, Error>
	{
		let kind = match self
		{
			Kind::Resistor { .. } => Kind::Resistor { ohms : value },
			Kind::Battery { .. } => Kind::Battery { volts : value },
			_ => return Err( Error::Valueless ),
		};

		kind.check()?;

		Ok( kind )
	}
}
