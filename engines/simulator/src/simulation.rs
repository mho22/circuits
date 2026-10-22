use std::collections::HashMap;

use crate::flatten::STRIDE;
use crate::{ advance, catalog, enclose, flatten, place, settle, stretch, Device, Document, Element, Enclosure, Error, History, Kind, Logic, Mode, Model, Netlist, Operation, Outline, Plan, Point, Reading, Registry, Segment, Shape, Solution };


const DIVISIONS : f64 = 20.0;


#[derive( Debug )]
pub struct Simulation
{
	layers : Vec<Document>,
	registry : Registry,
	elements : Vec<Element>,
	devices : Vec<Device>,
	plan : Plan,
	models : Vec<Option<Model>>,
	sources : Vec<Vec<Element>>,
	focus : Vec<u32>,
	history : History,
	logic : Logic,
	solution : Result<Solution, Error>,
	base : f64,
	step : f64,
	undos : Vec<Vec<( usize, Operation )>>,
	redos : Vec<Vec<( usize, Operation )>>,
}

impl Simulation
{
	pub fn new( step : f64 ) -> Simulation
	{
		Simulation { layers : vec![ Document::default() ], registry : Registry::default(), elements : Vec::new(), devices : Vec::new(), plan : Plan::default(), models : Vec::new(), sources : Vec::new(), focus : Vec::new(), history : History::default(), logic : Logic::default(), solution : Ok( Solution::default() ), base : step, step, undos : Vec::new(), redos : Vec::new() }
	}

	pub fn create( &mut self ) -> usize
	{
		self.layers.push( Document::default() );

		self.layers.len() - 1
	}

	pub fn apply( &mut self, layer : usize, operation : Operation ) -> Result<u32, Error>
	{
		self.commit( &[ ( layer, operation ) ] )
	}

	pub fn place( &mut self, layer : usize, kind : Kind, segment : Segment ) -> Result<u32, Error>
	{
		let operations = place( self.document( layer ).ok_or( Error::Layerless )?.elements(), kind, segment );
		let edits : Vec<( usize, Operation )> = operations.into_iter().map( | operation | ( layer, operation ) ).collect();

		self.commit( &edits )
	}

	pub fn transform( &mut self, layer : usize, id : u32, kind : Kind, segment : Segment, mirrored : bool ) -> Result<u32, Error>
	{
		let document = self.document( layer ).ok_or( Error::Layerless )?;
		let before = *document.elements().iter().find( | element | element.id == id ).ok_or( Error::Missing )?;
		let others : Vec<Element> = document.elements().iter().filter( | element | element.id != id ).copied().collect();
		let after = Element { kind, segment, mirrored, ..before };
		let ( old, new ) = ( self.terminals( &before ), self.terminals( &after ) );
		let mut operations = stretch( &others, &old, &new, self.zone( &after ), | element | self.terminals( element ) );
		let mut draft = document.clone();
		let mut laid = Vec::new();

		for operation in &operations { laid.push( draft.apply( operation.clone() )? ); }

		let cut : Vec<Element> = draft.elements().iter().filter( | element | ! laid.contains( &element.id ) ).copied().collect();

		operations.extend( place( &cut, kind, segment ) );
		operations.pop();
		operations.push( Operation::Move { id, kind, segment, mirrored } );

		let edits : Vec<( usize, Operation )> = operations.into_iter().map( | operation | ( layer, operation ) ).collect();

		self.commit( &edits )
	}

	pub fn shift( &mut self, layer : usize, low : Point, high : Point, dx : i32, dy : i32 ) -> Result<u32, Error>
	{
		let document = self.document( layer ).ok_or( Error::Layerless )?;
		let enclosure = enclose( document.elements(), low, high, | element | self.points( element ) )?;
		let ids : Vec<u32> = enclosure.inner.iter().map( | element | element.id ).collect();
		let others : Vec<Element> = document.elements().iter().filter( | element | ! ids.contains( &element.id ) ).copied().collect();
		let moved : Vec<Element> = enclosure.inner.iter().map( | element | Element { segment : element.segment.shifted( dx, dy ), ..*element } ).collect();
		let old : Vec<Point> = enclosure.inner.iter().flat_map( | element | self.terminals( element ) ).collect();
		let new : Vec<Point> = moved.iter().flat_map( | element | self.terminals( element ) ).collect();
		let mut operations = stretch( &others, &old, &new, Segment { from : low, to : high }.shifted( dx, dy ), | element | self.terminals( element ) );
		let mut draft = document.clone();
		let mut laid = Vec::new();

		for operation in &operations { laid.push( draft.apply( operation.clone() )? ); }

		for element in moved.iter().filter( | element | element.kind != Kind::Wire )
		{
			let outside : Vec<Element> = draft.elements().iter().filter( | other | ! ids.contains( &other.id ) && ! laid.contains( &other.id ) ).copied().collect();
			let mut cuts = place( &outside, element.kind, element.segment );

			cuts.pop();

			for operation in &cuts { draft.apply( operation.clone() )?; }

			operations.extend( cuts );
		}

		operations.extend( moved.iter().map( | element | Operation::Move { id : element.id, kind : element.kind, segment : element.segment, mirrored : element.mirrored } ) );

		let edits : Vec<( usize, Operation )> = operations.into_iter().map( | operation | ( layer, operation ) ).collect();

		self.commit( &edits )
	}

	pub fn paste( &mut self, layer : usize, parts : &[ ( Element, String ) ], dx : i32, dy : i32 ) -> Result<u32, Error>
	{
		if parts.is_empty() { return Err( Error::Unselected ); }

		let mut draft = self.document( layer ).ok_or( Error::Layerless )?.clone();
		let mut operations = Vec::new();

		for ( element, label ) in parts
		{
			let segment = element.segment.shifted( dx, dy );
			let mut steps = place( draft.elements(), element.kind, segment );

			steps.pop();
			steps.push( Operation::Add { kind : element.kind, segment, mirrored : element.mirrored } );

			let mut id = 0;

			for step in &steps { id = draft.apply( step.clone() )?; }

			operations.extend( steps );

			if label.is_empty() { continue; }

			let naming = Operation::Label { id, label : label.clone() };

			draft.apply( naming.clone() )?;
			operations.push( naming );
		}

		let edits : Vec<( usize, Operation )> = operations.into_iter().map( | operation | ( layer, operation ) ).collect();

		self.commit( &edits )
	}

	pub fn enclose( &mut self, layer : usize, low : Point, high : Point, name : &str, drawing : &[ Shape ], layout : &[ Point ] ) -> Result<u32, Error>
	{
		let ( enclosure, inside, inner ) = self.gather( layer, low, high, name, drawing, layout )?;
		let center = Point { x : ( low.x + high.x ).div_euclid( 2 ), y : ( low.y + high.y ).div_euclid( 2 ) };
		let mut draft = self.layers[ layer ].clone();

		for operation in &enclosure.outside { draft.apply( operation.clone() )?; }

		let outline = Outline::of( &inner );
		let ends = outline.terminals( center, 0 );
		let followed = stretch( draft.elements(), &enclosure.pins, &ends, outline.zone( center, 0 ), | element | self.terminals( element ) );
		let next = self.layers.len();
		let symbol = Operation::Add { kind : Kind::Symbol { layer : next as u32, rotation : 0 }, segment : Segment { from : center, to : center }, mirrored : false };
		let inside = inside.into_iter().map( | operation | ( next, operation ) );
		let outside = enclosure.outside.into_iter().chain( followed ).chain( [ symbol ] ).map( | operation | ( layer, operation ) );
		let edits : Vec<( usize, Operation )> = inside.chain( outside ).collect();

		self.layers.push( Document::default() );

		let result = self.commit( &edits );

		if result.is_err() { self.layers.pop(); }

		result
	}

	pub fn erase( &mut self, layer : usize, ids : &[ u32 ] ) -> Result<u32, Error>
	{
		if ids.is_empty() { return Err( Error::Unselected ); }

		let edits : Vec<( usize, Operation )> = ids.iter().map( | &id | ( layer, Operation::Delete { id } ) ).collect();

		self.commit( &edits )
	}

	pub fn purge( &mut self, layer : usize ) -> Result<u32, Error>
	{
		if layer == 0 || layer >= self.layers.len() { return Err( Error::Layerless ); }

		let copy = | element : &Element | matches!( element.kind, Kind::Symbol { layer : inner, .. } if inner as usize == layer );
		let copies = self.layers.iter().enumerate().flat_map( | ( index, document ) | document.elements().iter().filter( | element | copy( element ) ).map( move | element | ( index, Operation::Delete { id : element.id } ) ) );
		let parts = self.layers[ layer ].elements().iter().map( | element | ( layer, Operation::Delete { id : element.id } ) );
		let edits : Vec<( usize, Operation )> = copies.chain( parts ).collect();

		self.commit( &edits )
	}

	pub fn open( &mut self, additions : Vec<( usize, Operation )> ) -> Result<u32, Error>
	{
		let count = self.layers.len();
		let needed = additions.iter().map( | addition | addition.0 + 1 ).max().unwrap_or( 0 );
		let deletions = self.layers.iter().enumerate().flat_map( | ( layer, document ) | document.elements().iter().map( move | element | ( layer, Operation::Delete { id : element.id } ) ).chain( [ ( layer, Operation::Describe { name : String::new(), drawing : Vec::new(), layout : Vec::new() } ) ] ) );
		let edits : Vec<( usize, Operation )> = deletions.chain( additions ).collect();

		self.focus( &[] )?;
		self.layers.resize_with( count.max( needed ), Document::default );

		let result = self.commit( &edits );

		if result.is_err() { self.layers.truncate( count ); }

		result
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
			let solution = advance( &self.elements, &self.devices, &self.plan, &self.models, &self.history, self.step, &mut self.logic );

			if let Ok( solution ) = &solution { self.history = History::record( &self.elements, solution ); }

			self.show( solution );

			if self.solution.is_err() { return; }
		}
	}

	pub fn document( &self, layer : usize ) -> Option<&Document>
	{
		self.layers.get( layer )
	}

	pub fn listening( &self ) -> Vec<u32>
	{
		let path = self.focus.as_slice();
		let layer = self.descend( path ).unwrap_or( 0 );
		let probe = | element : &&Element | self.registry.find( &[ path, &[ element.id ] ].concat() ).is_some_and( | id | self.elements.iter().any( | flat | flat.id == id && flat.kind == Kind::Output ) );

		self.layers[ layer ].elements().iter().filter( | element | matches!( element.kind, Kind::Toggle { .. } ) ).filter( probe ).map( | element | element.id ).collect()
	}

	pub fn captions( &self ) -> Vec<String>
	{
		let document = &self.layers[ self.descend( &self.focus ).unwrap_or( 0 ) ];
		let mut elements = Vec::new();
		let mut pins = Vec::new();

		for element in document.elements()
		{
			let Kind::Symbol { layer, .. } = element.kind else
			{
				if element.kind.pin() { pins.push( ( element.segment.to, document.label( element.id ) ) ); }

				elements.push( *element );

				continue;
			};

			let inner = &self.layers[ layer as usize ];
			let labels = inner.elements().iter().filter( | part | part.kind.pin() ).map( | part | inner.label( part.id ) );

			for ( end, label ) in self.terminals( element ).into_iter().zip( labels )
			{
				pins.push( ( end, label ) );
				elements.push( Element { id : element.id, kind : Kind::Wire, segment : Segment { from : end, to : end }, mirrored : false } );
			}
		}

		let netlist = Netlist::build( &elements );
		let caption = | toggle : &Element | pins.iter().find( | ( point, label ) | ! label.is_empty() && netlist.net( *point ) == netlist.net( toggle.segment.to ) ).map_or( String::new(), | ( _, label ) | label.to_string() );

		document.elements().iter().filter( | element | matches!( element.kind, Kind::Toggle { .. } ) ).map( caption ).collect()
	}

	pub fn delay( &self, layer : usize ) -> Option<f64>
	{
		self.models.get( layer )?.as_ref().map( | model | model.delay )
	}

	pub fn solution( &self ) -> Result<&Solution, Error>
	{
		self.solution.as_ref().map_err( | error | *error )
	}

	pub fn step( &self ) -> f64
	{
		self.step
	}

	pub fn period( &self ) -> Option<f64>
	{
		self.elements.iter().filter_map( | element | match element.kind { Kind::Clock { hertz } => Some( 1.0 / hertz ), _ => None } ).reduce( f64::min )
	}

	pub fn mode( &self ) -> Mode
	{
		Mode::of( &self.devices, &self.plan )
	}

	pub fn path( &self ) -> &[ u32 ]
	{
		&self.focus
	}

	pub fn focus( &mut self, path : &[ u32 ] ) -> Result<(), Error>
	{
		let layer = self.descend( path )?;

		let ( elements, devices ) = flatten( &self.layers, &mut self.registry, &self.models, layer, path )?;

		self.plan = Plan::build( &elements, &devices );
		self.elements = elements;
		self.devices = devices;
		self.focus = path.to_vec();

		self.start();

		Ok( () )
	}

	pub fn descend( &self, path : &[ u32 ] ) -> Result<usize, Error>
	{
		path.iter().try_fold( 0, | layer, &id |
		{
			let element = self.layers[ layer ].elements().iter().find( | element | element.id == id ).ok_or( Error::Missing )?;
			let Kind::Symbol { layer : inner, .. } = element.kind else { return Err( Error::Missing ); };

			Ok( inner as usize )
		} )
	}

	pub fn elements_at( &self, layer : usize, point : Point ) -> Vec<u32>
	{
		let Some( document ) = self.document( layer ) else { return Vec::new(); };

		document.elements().iter().rev().filter( | element | self.covers( element, point ) ).map( | element | element.id ).collect()
	}

	pub fn terminals( &self, element : &Element ) -> Vec<Point>
	{
		let Kind::Symbol { layer, rotation } = element.kind else { return element.terminals(); };

		self.layers.get( layer as usize ).map_or( Vec::new(), | document | Outline::of( document ).terminals( element.segment.from, rotation ) )
	}

	fn gather( &self, layer : usize, low : Point, high : Point, name : &str, drawing : &[ Shape ], layout : &[ Point ] ) -> Result<( Enclosure, Vec<Operation>, Document ), Error>
	{
		let document = self.document( layer ).ok_or( Error::Layerless )?;
		let enclosure = enclose( document.elements(), low, high, | element | self.points( element ) )?;
		let parts = enclosure.inner.iter().map( | element | Operation::Add { kind : element.kind, segment : element.segment, mirrored : element.mirrored } );
		let labels = enclosure.inner.iter().zip( 0 .. ).filter( | ( element, _ ) | ! document.label( element.id ).is_empty() ).map( | ( element, id ) | Operation::Label { id, label : document.label( element.id ).to_string() } );
		let description = Operation::Describe { name : name.to_string(), drawing : drawing.to_vec(), layout : layout.to_vec() };
		let inside : Vec<Operation> = parts.chain( labels ).chain( [ description ] ).collect();
		let mut inner = Document::default();

		for operation in &inside { inner.apply( operation.clone() )?; }

		Ok( ( enclosure, inside, inner ) )
	}

	fn points( &self, element : &Element ) -> Vec<Point>
	{
		let Segment { from, to } = self.zone( element );

		[ self.terminals( element ), vec![ from, to ] ].concat()
	}

	fn zone( &self, element : &Element ) -> Segment
	{
		let Kind::Symbol { layer, rotation } = element.kind else { return element.segment; };

		self.layers.get( layer as usize ).map_or( element.segment, | document | Outline::of( document ).zone( element.segment.from, rotation ) )
	}

	fn covers( &self, element : &Element, point : Point ) -> bool
	{
		if ! matches!( element.kind, Kind::Symbol { .. } ) { return element.segment.contains( point ) || element.terminals().contains( &point ); }

		let Segment { from, to } = self.zone( element );

		( from.x - 1 ..= to.x + 1 ).contains( &point.x ) && ( from.y - 1 ..= to.y + 1 ).contains( &point.y )
	}

	fn commit( &mut self, edits : &[ ( usize, Operation ) ] ) -> Result<u32, Error>
	{
		let ( id, inverses ) = self.perform( edits )?;

		self.undos.push( inverses );
		self.redos.clear();

		Ok( id )
	}

	fn perform( &mut self, edits : &[ ( usize, Operation ) ] ) -> Result<( u32, Vec<( usize, Operation )> ), Error>
	{
		let mut inverses = Vec::new();
		let mut result = Ok( 0 );

		for ( layer, operation ) in edits
		{
			result = self.edit( *layer, operation.clone() ).map( | ( id, inverse ) | { inverses.push( ( *layer, inverse ) ); id } );

			if result.is_err() { break; }
		}

		let mut focus = self.focus.clone();

		while self.descend( &focus ).is_err() { focus.pop(); }

		let layer = self.descend( &focus )?;
		let ( models, sources ) = catalog( &self.layers, &self.models, &self.sources, self.base );
		let result = result.and_then( | id | flatten( &self.layers, &mut self.registry, &models, layer, &focus ).map( | flat | ( id, flat ) ) );

		let ( id, ( elements, devices ) ) = match result
		{
			Ok( pair ) => pair,
			Err( error ) =>
			{
				for ( layer, inverse ) in inverses.iter().rev() { self.layers[ *layer ].apply( inverse.clone() ).ok(); }

				return Err( error );
			},
		};

		inverses.reverse();

		self.plan = Plan::build( &elements, &devices );
		self.elements = elements;
		self.devices = devices;
		self.models = models;
		self.sources = sources;
		self.focus = focus;

		self.start();

		Ok( ( id, inverses ) )
	}

	fn edit( &mut self, layer : usize, operation : Operation ) -> Result<( u32, Operation ), Error>
	{
		let document = self.layers.get_mut( layer ).ok_or( Error::Layerless )?;
		let inverse = document.inverse( &operation )?;

		Ok( ( document.apply( operation )?, inverse ) )
	}

	fn start( &mut self )
	{
		self.step = self.period().map_or( self.base, | period | period / DIVISIONS );

		let solution = settle( &self.elements, &self.devices, &self.plan, &self.models, &self.history, self.step, &mut self.logic );

		self.show( solution );
	}

	fn show( &mut self, raw : Result<Solution, Error> )
	{
		self.solution = raw.map( | solution | self.view( &solution ) );
	}

	fn view( &self, solution : &Solution ) -> Solution
	{
		let path = self.focus.as_slice();
		let layer = self.descend( path ).unwrap_or( 0 );
		let positions : HashMap<u32, usize> = self.elements.iter().enumerate().map( | ( index, element ) | ( element.id, index ) ).collect();
		let reading = | key : &[ u32 ] | &solution.readings[ positions[ &self.registry.find( key ).unwrap() ] ];

		let readings = self.layers[ layer ].elements().iter().map( | element |
		{
			let key = [ path, &[ element.id ] ].concat();

			match element.kind
			{
				Kind::Symbol { layer : inner, .. } =>
				{
					let joins : Vec<&Reading> = self.layers[ inner as usize ].elements().iter().filter( | part | part.kind.pin() ).map( | pin | reading( &[ key.as_slice(), &[ pin.id ] ].concat() ) ).collect();

					Reading { potentials : joins.iter().map( | join | join.potentials[ 1 ] ).collect(), currents : joins.iter().map( | join | join.currents[ 1 ] ).collect() }
				},
				_ => reading( &key ).clone(),
			}
		} ).collect();

		let branches = solution.branches.iter().filter( | branch | branch.segment.from.x.abs() < STRIDE / 2 ).copied().collect();

		Solution { readings, branches }
	}
}
