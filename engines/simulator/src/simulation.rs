use crate::{ solve, Document, Error, History, Operation, Solution };


#[derive( Debug )]
pub struct Simulation
{
	document : Document,
	history : History,
	solution : Result<Solution, Error>,
	step : f64,
	undos : Vec<Vec<Operation>>,
	redos : Vec<Vec<Operation>>,
}

impl Simulation
{
	pub fn new( step : f64 ) -> Simulation
	{
		Simulation { document : Document::default(), history : History::default(), solution : Ok( Solution::default() ), step, undos : Vec::new(), redos : Vec::new() }
	}

	pub fn apply( &mut self, operation : Operation ) -> Result<u32, Error>
	{
		self.commit( &[ operation ] )
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
			self.solution = solve( self.document.elements(), &self.history, self.step );

			let Ok( solution ) = &self.solution else { return; };

			self.history = History::record( self.document.elements(), solution );
		}
	}

	pub fn document( &self ) -> &Document
	{
		&self.document
	}

	pub fn solution( &self ) -> Result<&Solution, Error>
	{
		self.solution.as_ref().map_err( | error | *error )
	}

	fn commit( &mut self, operations : &[ Operation ] ) -> Result<u32, Error>
	{
		let ( id, inverses ) = self.perform( operations )?;

		self.undos.push( inverses );
		self.redos.clear();

		Ok( id )
	}

	fn perform( &mut self, operations : &[ Operation ] ) -> Result<( u32, Vec<Operation> ), Error>
	{
		let mut inverses = Vec::new();
		let mut id = 0;

		for &operation in operations
		{
			let result = self.document.inverse( operation ).and_then( | inverse | self.document.apply( operation ).map( | applied | ( applied, inverse ) ) );

			let ( applied, inverse ) = match result
			{
				Ok( pair ) => pair,
				Err( error ) =>
				{
					for &inverse in inverses.iter().rev() { self.document.apply( inverse ).ok(); }

					return Err( error );
				},
			};

			id = applied;

			inverses.push( inverse );
		}

		inverses.reverse();

		self.solution = solve( self.document.elements(), &self.history, self.step );

		Ok( ( id, inverses ) )
	}
}
