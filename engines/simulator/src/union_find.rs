pub struct UnionFind
{
	parents : Vec<usize>,
}

impl UnionFind
{
	pub fn new( size : usize ) -> UnionFind
	{
		UnionFind { parents : ( 0 .. size ).collect() }
	}

	pub fn find( &mut self, item : usize ) -> usize
	{
		let parent = self.parents[ item ];

		if parent == item { return item; }

		let root = self.find( parent );

		self.parents[ item ] = root;

		root
	}

	pub fn union( &mut self, a : usize, b : usize )
	{
		let a = self.find( a );
		let b = self.find( b );

		self.parents[ b ] = a;
	}
}
