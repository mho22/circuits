use std::collections::HashMap;


#[derive( Debug, Default )]
pub struct Registry
{
	ids : HashMap<Vec<u32>, u32>,
}

impl Registry
{
	pub fn id( &mut self, path : &[ u32 ] ) -> u32
	{
		let next = self.ids.len() as u32;

		*self.ids.entry( path.to_vec() ).or_insert( next )
	}
}
