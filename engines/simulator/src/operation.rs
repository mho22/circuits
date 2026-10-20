use crate::Point;


#[derive( Clone, Copy, Debug, PartialEq, Eq )]
pub enum Operation
{
	AddWire { from : Point, to : Point },
	DeleteWire { id : u32 },
}
