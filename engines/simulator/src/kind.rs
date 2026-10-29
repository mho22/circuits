use crate::flatten::HIGH;
use crate::{ Error, Junction, LIGHTS, SILICON };


const COLORS : u8 = LIGHTS.len() as u8;
const FASTEST : f64 = 1e10;


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
	Led { color : u8 },
	Clock { hertz : f64, on : bool },
	Slider { volts : f64 },
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
			Kind::Clock { hertz, .. } if ! positive( hertz ) || hertz > FASTEST => Err( Error::Frequency ),
			Kind::Led { color } if color >= COLORS => Err( Error::Color ),
			Kind::Slider { volts } if ! ( 0.0 ..= HIGH ).contains( &volts ) => Err( Error::Range ),
			_ => Ok( () ),
		}
	}

	pub fn controls( &self ) -> &'static [ ( usize, usize ) ]
	{
		match self
		{
			Kind::Diode | Kind::Led { .. } => &[ ( 0, 1 ) ],
			Kind::Npn => &[ ( 0, 2 ), ( 0, 1 ) ],
			Kind::Pnp => &[ ( 2, 0 ), ( 1, 0 ) ],
			Kind::Nmos => &[ ( 0, 2 ), ( 1, 2 ) ],
			Kind::Pmos => &[ ( 2, 0 ), ( 2, 1 ) ],
			_ => &[],
		}
	}

	pub fn junction( &self ) -> Junction
	{
		if let Kind::Led { color } = *self { return LIGHTS[ color as usize ]; }

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
			Kind::Clock { hertz, .. } => Some( hertz ),
			Kind::Slider { volts } => Some( volts ),
			Kind::Led { color } => Some( color as f64 ),
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
			Kind::Clock { on, .. } => Kind::Clock { hertz : value, on : *on },
			Kind::Slider { .. } => Kind::Slider { volts : value },
			Kind::Led { .. } if value.fract() == 0.0 && ( 0.0 .. COLORS as f64 ).contains( &value ) => Kind::Led { color : value as u8 },
			Kind::Led { .. } => return Err( Error::Color ),
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
			Kind::Clock { hertz, .. } => Ok( Kind::Clock { hertz : *hertz, on : high } ),
			_ => Err( Error::Inputless ),
		}
	}

	pub fn pin( &self ) -> bool
	{
		matches!( self, Kind::Input { .. } | Kind::Output )
	}

	pub fn labeled( &self ) -> bool
	{
		self.pin() || matches!( self, Kind::Symbol { .. } )
	}
}
