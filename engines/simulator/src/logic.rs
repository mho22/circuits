use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::{ Action, Event };


#[derive( Clone, Debug, Default )]
pub struct Logic
{
	pub time : f64,
	pub keys : Vec<u32>,
	pub levels : Vec<f64>,
	pub rows : Vec<usize>,
	pub applied : Vec<usize>,
	pub states : Vec<usize>,
	pub sensed : Vec<Vec<bool>>,
	pub stamps : Vec<u64>,
	queue : BinaryHeap<Reverse<Event>>,
	sequence : u64,
}

impl Logic
{
	pub fn schedule( &mut self, time : f64, action : Action )
	{
		self.sequence += 1;
		self.queue.push( Reverse( Event { time, sequence : self.sequence, action } ) );
	}

	pub fn next( &mut self, until : f64 ) -> Option<Event>
	{
		let Reverse( event ) = *self.queue.peek()?;

		if event.time > until { return None; }

		self.queue.pop();

		Some( event )
	}

	pub fn pending( &self, until : f64 ) -> bool
	{
		self.queue.peek().is_some_and( | Reverse( event ) | event.time <= until )
	}

	pub fn clear( &mut self )
	{
		self.queue.clear();
	}
}
