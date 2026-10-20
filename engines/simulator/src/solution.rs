use crate::{ Branch, Point, Reading };


#[derive( Clone, Debug, Default, PartialEq )]
pub struct Solution
{
	pub readings : Vec<Reading>,
	pub branches : Vec<Branch>,
}

impl Solution
{
	pub fn branch_at( &self, point : Point ) -> Option<usize>
	{
		self.branches.iter().position( | branch | branch.segment.contains( point ) )
	}
}
