use crate::{ driven, Device, Document, Element, Error, Kind, Model, Netlist, Outline, Point, Registry, Segment, FLOOR };


pub const STRIDE : i32 = 1 << 16;
pub const HIGH : f64 = 5.0;


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

fn listen( elements : &mut [ Element ], devices : &[ Device ], toggles : &[ usize ] )
{
	let netlist = Netlist::build( elements );
	let outputs : Vec<usize> = devices.iter().flat_map( | device | device.outputs.iter().map( | &( _, resistor ) | resistor ) ).collect();
	let mut driven = vec![ false; netlist.count ];

	for ( index, element ) in elements.iter().enumerate().filter( | ( index, _ ) | ! toggles.contains( index ) )
	{
		let points = match element.kind
		{
			Kind::Supply { .. } | Kind::Battery { .. } | Kind::Ground | Kind::Clock { .. } => element.terminals(),
			Kind::Npn | Kind::Pnp | Kind::Nmos | Kind::Pmos => element.terminals()[ 1 .. ].to_vec(),
			_ if outputs.contains( &index ) => vec![ element.segment.to ],
			_ => continue,
		};

		for point in points { driven[ netlist.net( point ) ] = true; }
	}

	for &toggle in toggles
	{
		if driven[ netlist.net( elements[ toggle ].segment.to ) ] { elements[ toggle ].kind = Kind::Output; }
	}
}


struct Walk<'a>
{
	layers : &'a [ Document ],
	registry : &'a mut Registry,
	models : &'a [ Option<Model> ],
	elements : Vec<Element>,
	devices : Vec<Device>,
	depth : usize,
	scopes : i32,
	toggles : Vec<usize>,
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
					Kind::Clock { .. } => ( element.kind, element.segment.to ),
					kind => ( kind, element.segment.from ),
				};

				if let Kind::Toggle { .. } = element.kind { self.toggles.push( self.elements.len() ); }

				self.elements.push( Element { id : self.registry.id( &key ), kind, segment : Segment { from : shift( from, offset ), to : shift( element.segment.to, offset ) }, mirrored : element.mirrored } );

				if let Kind::Input { high } = element.kind { self.drive( &key, shift( element.segment.to, offset ), high ); }

				continue;
			};

			let definition = layers[ inner as usize ].elements();
			let ends = Outline::of( &layers[ inner as usize ] ).terminals( element.segment.from, rotation );

			self.scopes += 1;

			let scope = self.scopes * STRIDE;

			if matches!( self.models.get( inner as usize ), Some( Some( _ ) ) ) { self.device( layer, element, &key, offset, scope ); continue; }

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

	fn device( &mut self, layer : usize, symbol : &Element, key : &[ u32 ], offset : i32, scope : i32 )
	{
		let layers = self.layers;
		let Kind::Symbol { layer : inner, rotation } = symbol.kind else { return; };
		let inner = inner as usize;
		let ends = Outline::of( &layers[ inner ] ).terminals( symbol.segment.from, rotation );
		let mut device = Device { layer : inner, inputs : Vec::new(), outputs : Vec::new() };

		for ( ( pin, end ), index ) in layers[ inner ].elements().iter().filter( | part | part.kind.pin() ).zip( ends ).zip( 0 .. )
		{
			let join = [ key, &[ pin.id ] ].concat();
			let hidden = Point { x : scope + index, y : 0 };
			let segment = Segment { from : hidden, to : shift( end, offset ) };
			let free = matches!( pin.kind, Kind::Input { .. } ) && ! driven( layers, layer, symbol.id, end );

			if pin.kind != Kind::Output && ! free
			{
				self.elements.push( Element { id : self.registry.id( &join ), kind : Kind::Switch { closed : false }, segment, mirrored : false } );
				device.inputs.push( self.elements.len() - 1 );

				continue;
			}

			self.drive( &join, hidden, pin.kind == Kind::Input { high : true } );
			self.elements.push( Element { id : self.registry.id( &join ), kind : Kind::Resistor { ohms : FLOOR }, segment, mirrored : false } );

			let at = self.elements.len() - 1;

			if pin.kind == Kind::Output { device.outputs.push( ( at - 1, at ) ); continue; }

			device.inputs.push( at );
		}

		self.devices.push( device );
	}

	fn drive( &mut self, key : &[ u32 ], point : Point, high : bool )
	{
		let id = self.registry.id( &[ key, &[ 0 ] ].concat() );

		self.elements.push( Element { id, kind : Kind::Supply { volts : if high { HIGH } else { 0.0 } }, segment : Segment { from : point, to : point }, mirrored : false } );
	}
}


pub fn flatten( layers : &[ Document ], registry : &mut Registry, models : &[ Option<Model> ], layer : usize, path : &[ u32 ] ) -> Result<( Vec<Element>, Vec<Device> ), Error>
{
	let mut done = Vec::new();

	for layer in 0 .. layers.len() { visit( layers, layer, &mut Vec::new(), &mut done )?; }

	let mut walk = Walk { layers, registry, models, elements : Vec::new(), devices : Vec::new(), depth : path.len(), scopes : 0, toggles : Vec::new() };

	walk.expand( layer, path, 0 );

	listen( &mut walk.elements, &walk.devices, &walk.toggles );

	Ok( ( walk.elements, walk.devices ) )
}
