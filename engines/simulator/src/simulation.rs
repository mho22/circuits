use std::collections::HashMap;

use crate::flatten::STRIDE;
use crate::{ flatten, place, solve, stretch, Document, Element, Error, History, Kind, Operation, Outline, Point, Reading, Registry, Segment, Solution };


#[derive( Debug )]
pub struct Simulation
{
	layers : Vec<Document>,
	registry : Registry,
	elements : Vec<Element>,
	history : History,
	solution : Result<Solution, Error>,
	step : f64,
	undos : Vec<Vec<( usize, Operation )>>,
	redos : Vec<Vec<( usize, Operation )>>,
}

impl Simulation
{
	pub fn new( step : f64 ) -> Simulation
	{
		Simulation { layers : vec![ Document::default() ], registry : Registry::default(), elements : Vec::new(), history : History::default(), solution : Ok( Solution::default() ), step, undos : Vec::new(), redos : Vec::new() }
	}

	pub fn create( &mut self ) -> usize
	{
		self.layers.push( Document::default() );

		self.layers.len() - 1
	}

	pub fn apply( &mut self, layer : usize, operation : Operation ) -> Result<u32, Error>
	{
		self.commit( &[ ( layer, operation ) ] )
	}

	pub fn place( &mut self, layer : usize, kind : Kind, segment : Segment ) -> Result<u32, Error>
	{
		let operations = place( self.document( layer ).ok_or( Error::Layerless )?.elements(), kind, segment );
		let edits : Vec<( usize, Operation )> = operations.into_iter().map( | operation | ( layer, operation ) ).collect();

		self.commit( &edits )
	}

	pub fn transform( &mut self, layer : usize, id : u32, kind : Kind, segment : Segment, mirrored : bool ) -> Result<u32, Error>
	{
		let document = self.document( layer ).ok_or( Error::Layerless )?;
		let before = *document.elements().iter().find( | element | element.id == id ).ok_or( Error::Missing )?;
		let mut operations = stretch( document.elements(), &before, &Element { kind, segment, mirrored, ..before }, | element | self.terminals( element ) );
		let mut draft = document.clone();

		for &operation in &operations { draft.apply( operation )?; }

		operations.extend( place( draft.elements(), kind, segment ) );
		operations.pop();
		operations.push( Operation::Move { id, kind, segment, mirrored } );

		let edits : Vec<( usize, Operation )> = operations.into_iter().map( | operation | ( layer, operation ) ).collect();

		self.commit( &edits )
	}

	pub fn open( &mut self, parts : &[ ( Kind, Segment, bool ) ] ) -> Result<u32, Error>
	{
		let deletions = self.layers[ 0 ].elements().iter().map( | element | ( 0, Operation::Delete { id : element.id } ) );
		let additions = parts.iter().map( | &( kind, segment, mirrored ) | ( 0, Operation::Add { kind, segment, mirrored } ) );
		let edits : Vec<( usize, Operation )> = deletions.chain( additions ).collect();

		self.commit( &edits )
	}

	pub fn undo( &mut self ) -> Result<Option<u32>, Error>
	{
		let Some( operations ) = self.undos.pop() else { return Ok( None ); };

		let ( id, inverses ) = self.perform( &operations )?;

		self.redos.push( inverses );

		Ok( Some( id ) )
	}

	pub fn redo( &mut self ) -> Result<Option<u32>, Error>
	{
		let Some( operations ) = self.redos.pop() else { return Ok( None ); };

		let ( id, inverses ) = self.perform( &operations )?;

		self.undos.push( inverses );

		Ok( Some( id ) )
	}

	pub fn advance( &mut self, count : u32 )
	{
		for _ in 0 .. count
		{
			let solution = solve( &self.elements, &self.history, self.step );

			if let Ok( solution ) = &solution { self.history = History::record( &self.elements, solution ); }

			self.solution = solution.map( | solution | self.view( solution ) );

			if self.solution.is_err() { return; }
		}
	}

	pub fn document( &self, layer : usize ) -> Option<&Document>
	{
		self.layers.get( layer )
	}

	pub fn solution( &self ) -> Result<&Solution, Error>
	{
		self.solution.as_ref().map_err( | error | *error )
	}

	pub fn terminals( &self, element : &Element ) -> Vec<Point>
	{
		let Kind::Symbol { layer, rotation } = element.kind else { return element.terminals(); };

		self.layers.get( layer as usize ).map_or( Vec::new(), | document | Outline::of( document.elements() ).terminals( element.segment.from, rotation ) )
	}

	fn commit( &mut self, edits : &[ ( usize, Operation ) ] ) -> Result<u32, Error>
	{
		let ( id, inverses ) = self.perform( edits )?;

		self.undos.push( inverses );
		self.redos.clear();

		Ok( id )
	}

	fn perform( &mut self, edits : &[ ( usize, Operation ) ] ) -> Result<( u32, Vec<( usize, Operation )> ), Error>
	{
		let mut inverses = Vec::new();
		let mut result = Ok( 0 );

		for &( layer, operation ) in edits
		{
			result = self.edit( layer, operation ).map( | ( id, inverse ) | { inverses.push( ( layer, inverse ) ); id } );

			if result.is_err() { break; }
		}

		let result = result.and_then( | id | flatten( &self.layers, &mut self.registry ).map( | elements | ( id, elements ) ) );

		let ( id, elements ) = match result
		{
			Ok( pair ) => pair,
			Err( error ) =>
			{
				for &( layer, inverse ) in inverses.iter().rev() { self.layers[ layer ].apply( inverse ).ok(); }

				return Err( error );
			},
		};

		inverses.reverse();

		self.elements = elements;
		self.solution = solve( &self.elements, &self.history, self.step ).map( | solution | self.view( solution ) );

		Ok( ( id, inverses ) )
	}

	fn edit( &mut self, layer : usize, operation : Operation ) -> Result<( u32, Operation ), Error>
	{
		let document = self.layers.get_mut( layer ).ok_or( Error::Layerless )?;
		let inverse = document.inverse( operation )?;

		Ok( ( document.apply( operation )?, inverse ) )
	}

	fn view( &mut self, solution : Solution ) -> Solution
	{
		let positions : HashMap<u32, usize> = self.elements.iter().enumerate().map( | ( index, element ) | ( element.id, index ) ).collect();
		let registry = &mut self.registry;
		let layers = &self.layers;

		let readings = layers[ 0 ].elements().iter().map( | element |
		{
			let Kind::Symbol { layer, .. } = element.kind else { return solution.readings[ positions[ &registry.id( &[ element.id ] ) ] ].clone(); };

			let joins : Vec<&Reading> = layers[ layer as usize ].elements().iter().filter( | part | part.kind == Kind::Pin ).map( | pin | &solution.readings[ positions[ &registry.id( &[ element.id, pin.id ] ) ] ] ).collect();

			Reading { potentials : joins.iter().map( | join | join.potentials[ 1 ] ).collect(), currents : joins.iter().map( | join | join.currents[ 1 ] ).collect() }
		} ).collect();

		let branches = solution.branches.into_iter().filter( | branch | branch.segment.from.x.abs() < STRIDE / 2 ).collect();

		Solution { readings, branches }
	}
}
