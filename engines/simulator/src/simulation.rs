use crate::{ solve, Document, Error, History, Operation, Solution };


#[derive( Debug )]
pub struct Simulation
{
	document : Document,
	history : History,
	solution : Result<Solution, Error>,
	step : f64,
}

impl Simulation
{
	pub fn new( step : f64 ) -> Simulation
	{
		Simulation { document : Document::default(), history : History::default(), solution : Ok( Solution::default() ), step }
	}

	pub fn apply( &mut self, operation : Operation ) -> Result<u32, Error>
	{
		let id = self.document.apply( operation )?;

		self.solution = solve( self.document.elements(), &self.history, self.step );

		Ok( id )
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
}
