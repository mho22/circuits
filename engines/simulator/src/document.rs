use crate::{ Error, Operation, Point, Wire };


#[derive( Debug, Default )]
pub struct Document
{
	wires : Vec<Wire>,
	next : u32,
}

impl Document
{
	pub fn apply( &mut self, operation : Operation ) -> Result<u32, Error>
	{
		match operation
		{
			Operation::AddWire { from, to } => self.add_wire( from, to ),
			Operation::DeleteWire { id } => self.delete_wire( id ),
		}
	}

	pub fn wires( &self ) -> &[ Wire ]
	{
		&self.wires
	}

	pub fn wire_at( &self, point : Point ) -> Option<u32>
	{
		self.wires.iter().rev().find( | wire | wire.contains( point ) ).map( | wire | wire.id )
	}

	fn add_wire( &mut self, from : Point, to : Point ) -> Result<u32, Error>
	{
		if from == to { return Err( Error::Empty ); }

		if ! Wire::is_straight( from, to ) { return Err( Error::Crooked ); }

		let id = self.next;

		self.wires.push( Wire { id, from, to } );

		self.next += 1;

		Ok( id )
	}

	fn delete_wire( &mut self, id : u32 ) -> Result<u32, Error>
	{
		let index = self.wires.iter().position( | wire | wire.id == id ).ok_or( Error::Missing )?;

		self.wires.remove( index );

		Ok( id )
	}
}
