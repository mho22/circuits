use crate::{ Element, Kind, Segment };


#[derive( Clone, Copy, Debug, PartialEq )]
pub enum Operation
{
	Add { kind : Kind, segment : Segment, mirrored : bool },
	Insert { element : Element, index : usize },
	Delete { id : u32 },
	SetValue { id : u32, value : f64 },
	SetClosed { id : u32, closed : bool },
	Move { id : u32, kind : Kind, segment : Segment, mirrored : bool },
}
