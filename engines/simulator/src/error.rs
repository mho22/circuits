use std::fmt;


#[derive( Clone, Copy, Debug, PartialEq, Eq )]
pub enum Error
{
	Crooked,
	Empty,
	Missing,
}

impl fmt::Display for Error
{
	fn fmt( &self, formatter : &mut fmt::Formatter ) -> fmt::Result
	{
		let message = match self
		{
			Error::Crooked => "A wire must run at 0°, 45° or 90°.",
			Error::Empty => "A wire must have a length.",
			Error::Missing => "No wire has this id.",
		};

		formatter.write_str( message )
	}
}
