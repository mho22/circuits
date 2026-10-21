use std::cmp::Ordering;

use crate::Action;


#[derive( Clone, Copy, Debug )]
pub struct Event
{
	pub time : f64,
	pub sequence : u64,
	pub action : Action,
}

impl PartialEq for Event
{
	fn eq( &self, other : &Event ) -> bool
	{
		self.cmp( other ) == Ordering::Equal
	}
}

impl Eq for Event {}

impl PartialOrd for Event
{
	fn partial_cmp( &self, other : &Event ) -> Option<Ordering>
	{
		Some( self.cmp( other ) )
	}
}

impl Ord for Event
{
	fn cmp( &self, other : &Event ) -> Ordering
	{
		self.time.total_cmp( &other.time ).then( self.sequence.cmp( &other.sequence ) )
	}
}
