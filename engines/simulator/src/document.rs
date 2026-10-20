use crate::{ Element, Error, Kind, Operation, Point, Segment };


#[derive( Clone, Debug, Default )]
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
			Operation::Add { kind, segment, mirrored } => self.add( kind, segment, mirrored ),
			Operation::Insert { element, index } => self.insert( element, index ),
			Operation::Delete { id } => self.delete( id ),
			Operation::SetValue { id, value } => self.set_value( id, value ),
			Operation::SetClosed { id, closed } => self.set_closed( id, closed ),
			Operation::Move { id, segment, mirrored } => self.move_to( id, segment, mirrored ),
		}
	}

	pub fn inverse( &self, operation : Operation ) -> Result<Operation, Error>
	{
		match operation
		{
			Operation::Add { .. } => Ok( Operation::Delete { id : self.next } ),
			Operation::Insert { element, .. } => Ok( Operation::Delete { id : element.id } ),
			Operation::Delete { id } =>
			{
				let index = self.index( id )?;

				Ok( Operation::Insert { element : self.elements[ index ], index } )
			},
			Operation::SetValue { id, .. } =>
			{
				let value = self.elements[ self.index( id )? ].kind.value().ok_or( Error::Valueless )?;

				Ok( Operation::SetValue { id, value } )
			},
			Operation::SetClosed { id, .. } =>
			{
				let Kind::Switch { closed } = self.elements[ self.index( id )? ].kind else { return Err( Error::Switchless ); };

				Ok( Operation::SetClosed { id, closed } )
			},
			Operation::Move { id, .. } =>
			{
				let element = self.elements[ self.index( id )? ];

				Ok( Operation::Move { id, segment : element.segment, mirrored : element.mirrored } )
			},
		}
	}

	pub fn elements( &self ) -> &[ Element ]
	{
		&self.elements
	}

	pub fn elements_at( &self, point : Point ) -> Vec<u32>
	{
		self.elements.iter().rev().filter( | element | element.segment.contains( point ) || element.terminals().contains( &point ) ).map( | element | element.id ).collect()
	}

	fn check( kind : Kind, segment : Segment ) -> Result<(), Error>
	{
		let point = matches!( kind, Kind::Ground | Kind::Supply { .. } );

		if point && ! segment.is_empty() { return Err( Error::Misplaced ); }

		if ! point && segment.is_empty() { return Err( Error::Empty ); }

		if ! segment.is_straight() { return Err( Error::Crooked ); }

		kind.check()
	}

	fn add( &mut self, kind : Kind, segment : Segment, mirrored : bool ) -> Result<u32, Error>
	{
		Document::check( kind, segment )?;

		let id = self.next;

		self.elements.push( Element { id, kind, segment, mirrored } );

		self.next += 1;

		Ok( id )
	}

	fn insert( &mut self, element : Element, index : usize ) -> Result<u32, Error>
	{
		if self.index( element.id ).is_ok() { return Err( Error::Taken ); }

		Document::check( element.kind, element.segment )?;

		self.elements.insert( index.min( self.elements.len() ), element );

		self.next = self.next.max( element.id + 1 );

		Ok( element.id )
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

	fn move_to( &mut self, id : u32, segment : Segment, mirrored : bool ) -> Result<u32, Error>
	{
		let index = self.index( id )?;

		Document::check( self.elements[ index ].kind, segment )?;

		self.elements[ index ].segment = segment;
		self.elements[ index ].mirrored = mirrored;

		Ok( id )
	}

	fn index( &self, id : u32 ) -> Result<usize, Error>
	{
		self.elements.iter().position( | element | element.id == id ).ok_or( Error::Missing )
	}
}
