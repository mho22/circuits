use std::cmp::Reverse;
use std::collections::{ BinaryHeap, HashMap, VecDeque };

use crate::{ Action, Event, Exposure, Solution };


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
	pub since : Vec<Option<f64>>,
	pub queued : Vec<VecDeque<( u64, usize )>>,
	pub memory : HashMap<u32, ( usize, usize )>,
	pub spare : Solution,
	pub drawn : Vec<f64>,
	pub peaks : Vec<f64>,
	pub kept : bool,
	pub exposure : Exposure,
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

	pub fn firing( &self ) -> bool
	{
		self.queue.iter().any( | Reverse( event ) | matches!( event.action, Action::Fire { .. } ) )
	}

	pub fn clear( &mut self )
	{
		self.queue.clear();
	}
}
