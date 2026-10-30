use std::hash::BuildHasherDefault;


const SEED : u64 = 0x51_7c_c1_b7_27_22_0a_95;


pub type Hashing = BuildHasherDefault<Hasher>;


#[derive( Clone, Copy, Debug, Default )]
pub struct Hasher
{
	hash : u64,
}

impl Hasher
{
	fn add( &mut self, word : u64 )
	{
		self.hash = ( self.hash.rotate_left( 5 ) ^ word ).wrapping_mul( SEED );
	}
}

impl std::hash::Hasher for Hasher
{
	fn write( &mut self, bytes : &[ u8 ] )
	{
		for chunk in bytes.chunks( 8 )
		{
			let mut word = [ 0; 8 ];

			word[ .. chunk.len() ].copy_from_slice( chunk );
			self.add( u64::from_le_bytes( word ) );
		}
	}

	fn write_u32( &mut self, value : u32 )
	{
		self.add( value as u64 );
	}

	fn write_i32( &mut self, value : i32 )
	{
		self.add( value as u32 as u64 );
	}

	fn write_u64( &mut self, value : u64 )
	{
		self.add( value );
	}

	fn write_usize( &mut self, value : usize )
	{
		self.add( value as u64 );
	}

	fn finish( &self ) -> u64
	{
		self.hash.rotate_left( 26 )
	}
}
