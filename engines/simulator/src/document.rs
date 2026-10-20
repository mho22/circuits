use crate::{ Element, Error, Kind, Operation, Point, Segment };


#[derive( Debug, Default )]
pub struct Document
{
	elements : Vec<Element>,
	next : u32,
}

impl Document
{
	pub fn apply( &mut self, operation : Operation ) -> Result<u32, Error>
	{
		match operation
		{
			Operation::Add { kind, segment } => self.add( kind, segment ),
			Operation::Delete { id } => self.delete( id ),
			Operation::SetValue { id, value } => self.set_value( id, value ),
			Operation::SetClosed { id, closed } => self.set_closed( id, closed ),
		}
	}

	pub fn elements( &self ) -> &[ Element ]
	{
		&self.elements
	}

	pub fn elements_at( &self, point : Point ) -> Vec<u32>
	{
		self.elements.iter().rev().filter( | element | element.segment.contains( point ) ).map( | element | element.id ).collect()
	}

	fn add( &mut self, kind : Kind, segment : Segment ) -> Result<u32, Error>
	{
		if kind == Kind::Ground && ! segment.is_empty() { return Err( Error::Misplaced ); }

		if kind != Kind::Ground && segment.is_empty() { return Err( Error::Empty ); }

		if ! segment.is_straight() { return Err( Error::Crooked ); }

		kind.check()?;

		let id = self.next;

		self.elements.push( Element { id, kind, segment } );

		self.next += 1;

		Ok( id )
	}

	fn delete( &mut self, id : u32 ) -> Result<u32, Error>
	{
		let index = self.index( id )?;

		self.elements.remove( index );

		Ok( id )
	}

	fn set_value( &mut self, id : u32, value : f64 ) -> Result<u32, Error>
	{
		let index = self.index( id )?;

		self.elements[ index ].kind = self.elements[ index ].kind.with_value( value )?;

		Ok( id )
	}

	fn set_closed( &mut self, id : u32, closed : bool ) -> Result<u32, Error>
	{
		let index = self.index( id )?;

		self.elements[ index ].kind = self.elements[ index ].kind.with_closed( closed )?;

		Ok( id )
	}

	fn index( &self, id : u32 ) -> Result<usize, Error>
	{
		self.elements.iter().position( | element | element.id == id ).ok_or( Error::Missing )
	}
}
