use crate::{ driven, Document, Element, Error, Kind, Outline, Point, Registry, Segment };


pub const STRIDE : i32 = 1 << 16;
const HIGH : f64 = 5.0;


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
	depth : usize,
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
				if element.kind.pin() && path.len() > self.depth { continue; }

				let ( kind, from ) = match element.kind
				{
					Kind::Toggle { high } => ( Kind::Supply { volts : if high { HIGH } else { 0.0 } }, element.segment.to ),
					kind => ( kind, element.segment.from ),
				};

				self.elements.push( Element { id : self.registry.id( &key ), kind, segment : Segment { from : shift( from, offset ), to : shift( element.segment.to, offset ) }, mirrored : element.mirrored } );

				if let Kind::Input { high } = element.kind { self.drive( &key, shift( element.segment.to, offset ), high ); }

				continue;
			};

			let definition = layers[ inner as usize ].elements();
			let ends = Outline::of( &layers[ inner as usize ] ).terminals( element.segment.from, rotation );

			self.scopes += 1;

			let scope = self.scopes * STRIDE;

			for ( pin, end ) in definition.iter().filter( | part | part.kind.pin() ).zip( ends )
			{
				let join = [ key.as_slice(), &[ pin.id ] ].concat();
				let id = self.registry.id( &join );

				self.elements.push( Element { id, kind : Kind::Switch { closed : true }, segment : Segment { from : shift( pin.segment.to, scope ), to : shift( end, offset ) }, mirrored : false } );

				let Kind::Input { high } = pin.kind else { continue; };

				if driven( layers, layer, element.id, end ) { continue; }

				self.drive( &join, shift( pin.segment.to, scope ), high );
			}

			self.expand( inner as usize, &key, scope );
		}
	}

	fn drive( &mut self, key : &[ u32 ], point : Point, high : bool )
	{
		let id = self.registry.id( &[ key, &[ 0 ] ].concat() );

		self.elements.push( Element { id, kind : Kind::Supply { volts : if high { HIGH } else { 0.0 } }, segment : Segment { from : point, to : point }, mirrored : false } );
	}
}


pub fn flatten( layers : &[ Document ], registry : &mut Registry, layer : usize, path : &[ u32 ] ) -> Result<Vec<Element>, Error>
{
	let mut done = Vec::new();

	for layer in 0 .. layers.len() { visit( layers, layer, &mut Vec::new(), &mut done )?; }

	let mut walk = Walk { layers, registry, elements : Vec::new(), depth : path.len(), scopes : 0 };

	walk.expand( layer, path, 0 );

	Ok( walk.elements )
}
