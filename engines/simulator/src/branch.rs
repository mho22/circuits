use crate::Segment;


#[derive( Clone, Copy, Debug, PartialEq )]
pub struct Branch
{
	pub segment : Segment,
	pub voltage : f64,
	pub current : f64,
}
