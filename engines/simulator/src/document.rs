use std::collections::HashMap;

use crate::{ Element, Error, Kind, Operation, Point, Segment, Shape };


#[derive( Clone, Debug, Default )]
pub struct Document
{
	elements : Vec<Element>,
	next : u32,
	name : String,
	drawing : Vec<Shape>,
	layout : Vec<Point>,
	labels : HashMap<u32, String>,
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
			Operation::SetHigh { id, high } => self.set_high( id, high ),
			Operation::Move { id, kind, segment, mirrored } => self.move_to( id, kind, segment, mirrored ),
			Operation::Describe { name, drawing, layout } => self.describe( name, drawing, layout ),
			Operation::Label { id, label } => self.set_label( id, label ),
		}
	}

	pub fn inverse( &self, operation : &Operation ) -> Result<Operation, Error>
	{
		match *operation
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
			Operation::SetHigh { id, .. } =>
			{
				let ( Kind::Input { high } | Kind::Toggle { high } | Kind::Clock { on : high, .. } ) = self.elements[ self.index( id )? ].kind else { return Err( Error::Inputless ); };

				Ok( Operation::SetHigh { id, high } )
			},
			Operation::Move { id, .. } =>
			{
				let element = self.elements[ self.index( id )? ];

				Ok( Operation::Move { id, kind : element.kind, segment : element.segment, mirrored : element.mirrored } )
			},
			Operation::Describe { .. } => Ok( Operation::Describe { name : self.name.clone(), drawing : self.drawing.clone(), layout : self.layout.clone() } ),
			Operation::Label { id, .. } =>
			{
				if ! self.elements[ self.index( id )? ].kind.labeled() { return Err( Error::Labelless ); }

				Ok( Operation::Label { id, label : self.label( id ).to_string() } )
			},
		}
	}

	pub fn name( &self ) -> &str
	{
		&self.name
	}

	pub fn drawing( &self ) -> &[ Shape ]
	{
		&self.drawing
	}

	pub fn layout( &self ) -> &[ Point ]
	{
		&self.layout
	}

	pub fn label( &self, id : u32 ) -> &str
	{
		self.labels.get( &id ).map_or( "", | label | label.as_str() )
	}

	pub fn elements( &self ) -> &[ Element ]
	{
		&self.elements
	}

	fn check( kind : Kind, segment : Segment ) -> Result<(), Error>
	{
		let point = matches!( kind, Kind::Ground | Kind::Supply { .. } | Kind::Symbol { .. } );

		if point && ! segment.is_empty() { return Err( Error::Misplaced ); }

		if ! point && segment.is_empty() { return Err( Error::Empty ); }

		if ! segment.is_straight() { return Err( Error::Crooked ); }

		if matches!( kind, Kind::Slider { .. } ) && segment.from.y == segment.to.y && segment.length() < 2 { return Err( Error::Short ); }

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
		if element.id < self.next && self.index( element.id ).is_ok() { return Err( Error::Taken ); }

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

	fn set_high( &mut self, id : u32, high : bool ) -> Result<u32, Error>
	{
		let index = self.index( id )?;

		self.elements[ index ].kind = self.elements[ index ].kind.with_high( high )?;

		Ok( id )
	}

	fn move_to( &mut self, id : u32, kind : Kind, segment : Segment, mirrored : bool ) -> Result<u32, Error>
	{
		let index = self.index( id )?;

		Document::check( kind, segment )?;

		self.elements[ index ].kind = kind;
		self.elements[ index ].segment = segment;
		self.elements[ index ].mirrored = mirrored;

		Ok( id )
	}

	fn describe( &mut self, name : String, drawing : Vec<Shape>, layout : Vec<Point> ) -> Result<u32, Error>
	{
		for shape in &drawing { shape.check()?; }

		self.name = name;
		self.drawing = drawing;
		self.layout = layout;

		Ok( 0 )
	}

	fn set_label( &mut self, id : u32, label : String ) -> Result<u32, Error>
	{
		let index = self.index( id )?;

		if ! self.elements[ index ].kind.labeled() { return Err( Error::Labelless ); }

		self.labels.insert( id, label );

		Ok( id )
	}

	fn index( &self, id : u32 ) -> Result<usize, Error>
	{
		self.elements.iter().position( | element | element.id == id ).ok_or( Error::Missing )
	}
}
