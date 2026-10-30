#[derive( Clone, Debug, PartialEq )]
pub struct Device
{
	pub layer : usize,
	pub inputs : Vec<usize>,
	pub outputs : Vec<( usize, usize )>,
	pub scopes : Vec<( usize, i32 )>,
}
