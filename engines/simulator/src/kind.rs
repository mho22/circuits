use crate::Error;


#[derive( Clone, Copy, Debug, PartialEq )]
pub enum Kind
{
	Wire,
	Resistor { ohms : f64 },
	Battery { volts : f64 },
	Ground,
	Capacitor { farads : f64 },
	Inductor { henries : f64 },
	Switch { closed : bool },
	Diode,
	Npn,
	Pnp,
	Nmos,
	Pmos,
	Supply { volts : f64 },
}

impl Kind
{
	pub fn check( &self ) -> Result<(), Error>
	{
		let positive = | value : f64 | value.is_finite() && value > 0.0;

		match *self
		{
			Kind::Resistor { ohms } if ! positive( ohms ) => Err( Error::Resistance ),
			Kind::Battery { volts } | Kind::Supply { volts } if ! volts.is_finite() => Err( Error::Voltage ),
			Kind::Capacitor { farads } if ! positive( farads ) => Err( Error::Capacitance ),
			Kind::Inductor { henries } if ! positive( henries ) => Err( Error::Inductance ),
			_ => Ok( () ),
		}
	}

	pub fn controls( &self ) -> &'static [ ( usize, usize ) ]
	{
		match self
		{
			Kind::Diode => &[ ( 0, 1 ) ],
			Kind::Npn => &[ ( 0, 2 ), ( 0, 1 ) ],
			Kind::Pnp => &[ ( 2, 0 ), ( 1, 0 ) ],
			Kind::Nmos => &[ ( 0, 2 ), ( 1, 2 ) ],
			Kind::Pmos => &[ ( 2, 0 ), ( 2, 1 ) ],
			_ => &[],
		}
	}

	pub fn value( &self ) -> Option<f64>
	{
		match *self
		{
			Kind::Resistor { ohms } => Some( ohms ),
			Kind::Battery { volts } | Kind::Supply { volts } => Some( volts ),
			Kind::Capacitor { farads } => Some( farads ),
			Kind::Inductor { henries } => Some( henries ),
			_ => None,
		}
	}

	pub fn with_value( &self, value : f64 ) -> Result<Kind, Error>
	{
		let kind = match self
		{
			Kind::Resistor { .. } => Kind::Resistor { ohms : value },
			Kind::Battery { .. } => Kind::Battery { volts : value },
			Kind::Supply { .. } => Kind::Supply { volts : value },
			Kind::Capacitor { .. } => Kind::Capacitor { farads : value },
			Kind::Inductor { .. } => Kind::Inductor { henries : value },
			_ => return Err( Error::Valueless ),
		};

		kind.check()?;

		Ok( kind )
	}

	pub fn with_closed( &self, closed : bool ) -> Result<Kind, Error>
	{
		let Kind::Switch { .. } = self else { return Err( Error::Switchless ); };

		Ok( Kind::Switch { closed } )
	}
}
