use crate::{ Error, Junction, LIGHT, SILICON };


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
	Input { high : bool },
	Output,
	Symbol { layer : u32, rotation : u8 },
	Toggle { high : bool },
	Led,
	Clock { hertz : f64 },
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
			Kind::Symbol { rotation, .. } if rotation > 3 => Err( Error::Rotation ),
			Kind::Clock { hertz } if ! positive( hertz ) => Err( Error::Frequency ),
			_ => Ok( () ),
		}
	}

	pub fn controls( &self ) -> &'static [ ( usize, usize ) ]
	{
		match self
		{
			Kind::Diode | Kind::Led => &[ ( 0, 1 ) ],
			Kind::Npn => &[ ( 0, 2 ), ( 0, 1 ) ],
			Kind::Pnp => &[ ( 2, 0 ), ( 1, 0 ) ],
			Kind::Nmos => &[ ( 0, 2 ), ( 1, 2 ) ],
			Kind::Pmos => &[ ( 2, 0 ), ( 2, 1 ) ],
			_ => &[],
		}
	}

	pub fn junction( &self ) -> Junction
	{
		if *self == Kind::Led { return LIGHT; }

		SILICON
	}

	pub fn value( &self ) -> Option<f64>
	{
		match *self
		{
			Kind::Resistor { ohms } => Some( ohms ),
			Kind::Battery { volts } | Kind::Supply { volts } => Some( volts ),
			Kind::Capacitor { farads } => Some( farads ),
			Kind::Inductor { henries } => Some( henries ),
			Kind::Clock { hertz } => Some( hertz ),
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
			Kind::Clock { .. } => Kind::Clock { hertz : value },
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

	pub fn with_high( &self, high : bool ) -> Result<Kind, Error>
	{
		match self
		{
			Kind::Input { .. } => Ok( Kind::Input { high } ),
			Kind::Toggle { .. } => Ok( Kind::Toggle { high } ),
			_ => Err( Error::Inputless ),
		}
	}

	pub fn pin( &self ) -> bool
	{
		matches!( self, Kind::Input { .. } | Kind::Output )
	}
}
