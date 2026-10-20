use crate::{ Kind, Segment };


#[derive( Clone, Copy, Debug, PartialEq )]
pub enum Operation
{
	Add { kind : Kind, segment : Segment },
	Delete { id : u32 },
	SetValue { id : u32, value : f64 },
	SetClosed { id : u32, closed : bool },
}
