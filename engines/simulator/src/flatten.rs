use crate::{ Document, Element, Error, Kind, Outline, Point, Registry, Segment };


pub const STRIDE : i32 = 1 << 16;


fn shift( point : Point, offset : i32 ) -> Point
{
	Point { x : point.x + offset, y : point.y }
}

fn visit( layers : &[ Document ], layer : usize, open : &mut Vec<usize>, done : &mut Vec<usize> ) -> Result<(), Error>
{
	if done.contains( &layer ) { return Ok( () ); }

	if open.contains( &layer ) { return Err( Error::Recursive ); }

	open.push( layer );

	for element in layers[ layer ].elements()
	{
		let Kind::Symbol { layer : inner, .. } = element.kind else { continue; };

		if inner as usize >= layers.len() { return Err( Error::Layerless ); }

		visit( layers, inner as usize, open, done )?;
	}

	open.pop();
	done.push( layer );

	Ok( () )
}


struct Walk<'a>
{
	layers : &'a [ Document ],
	registry : &'a mut Registry,
	elements : Vec<Element>,
	scopes : i32,
}

impl Walk<'_>
{
	fn expand( &mut self, layer : usize, path : &[ u32 ], offset : i32 )
	{
		let layers = self.layers;

		for element in layers[ layer ].elements()
		{
			let key = [ path, &[ element.id ] ].concat();

			let Kind::Symbol { layer : inner, rotation } = element.kind else
			{
				if element.kind == Kind::Pin && ! path.is_empty() { continue; }

				self.elements.push( Element { id : self.registry.id( &key ), kind : element.kind, segment : Segment { from : shift( element.segment.from, offset ), to : shift( element.segment.to, offset ) }, mirrored : element.mirrored } );

				continue;
			};

			let definition = layers[ inner as usize ].elements();
			let ends = Outline::of( definition ).terminals( element.segment.from, rotation );

			self.scopes += 1;

			let scope = self.scopes * STRIDE;

			for ( pin, end ) in definition.iter().filter( | part | part.kind == Kind::Pin ).zip( ends )
			{
				let id = self.registry.id( &[ key.as_slice(), &[ pin.id ] ].concat() );

				self.elements.push( Element { id, kind : Kind::Switch { closed : true }, segment : Segment { from : shift( pin.segment.from, scope ), to : shift( end, offset ) }, mirrored : false } );
			}

			self.expand( inner as usize, &key, scope );
		}
	}
}


pub fn flatten( layers : &[ Document ], registry : &mut Registry ) -> Result<Vec<Element>, Error>
{
	let mut done = Vec::new();

	for layer in 0 .. layers.len() { visit( layers, layer, &mut Vec::new(), &mut done )?; }

	let mut walk = Walk { layers, registry, elements : Vec::new(), scopes : 0 };

	walk.expand( 0, &[], 0 );

	Ok( walk.elements )
}
