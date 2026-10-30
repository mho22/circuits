use std::collections::HashMap;

use crate::Hashing;


#[derive( Debug, Default )]
pub struct Registry
{
	ids : HashMap<Vec<u32>, u32, Hashing>,
}

impl Registry
{
	pub fn id( &mut self, path : &[ u32 ] ) -> u32
	{
		let next = self.ids.len() as u32;

		*self.ids.entry( path.to_vec() ).or_insert( next )
	}

	pub fn find( &self, path : &[ u32 ] ) -> Option<u32>
	{
		self.ids.get( path ).copied()
	}
}
