use crate::{ Kind, Segment };


#[derive( Clone, Copy, Debug, PartialEq )]
pub struct Element
{
	pub id : u32,
	pub kind : Kind,
	pub segment : Segment,
}
