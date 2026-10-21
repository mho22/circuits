#[derive( Clone, Copy, Debug, PartialEq )]
pub enum Action
{
	Fire { device : usize, stamp : u64 },
	Edge { clock : usize, count : u64 },
}
