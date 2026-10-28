use std::sync::Mutex;

use crate::{ Error, Factors };


const KEPT : usize = 4;


#[derive( Debug, Default )]
pub struct Factorizations
{
	kept : Mutex<Vec<Factors>>,
}

impl Factorizations
{
	pub fn solve( &self, size : usize, entries : &[ ( usize, usize, f64 ) ], rhs : Vec<f64>, order : &[ usize ] ) -> Result<Vec<f64>, Error>
	{
		let mut kept = self.kept.lock().unwrap();
		let found = kept.iter().enumerate().filter( | ( _, factors ) | factors.near( entries ) ).find_map( | ( index, factors ) | factors.refine( entries, &rhs ).map( | solution | ( index, solution ) ) );

		if let Some( ( index, solution ) ) = found
		{
			let factors = kept.remove( index );

			kept.insert( 0, factors );

			return Ok( solution );
		}

		let factors = match kept.iter().find_map( | factors | factors.replay( entries ) )
		{
			Some( factors ) => factors,
			None => Factors::new( size, entries, order )?,
		};
		let solution = factors.solve( rhs );

		kept.insert( 0, factors );
		kept.truncate( KEPT );

		Ok( solution )
	}
}

impl Clone for Factorizations
{
	fn clone( &self ) -> Factorizations
	{
		Factorizations::default()
	}
}

impl PartialEq for Factorizations
{
	fn eq( &self, _ : &Factorizations ) -> bool
	{
		true
	}
}
