use std::collections::{ HashMap, HashSet };

use crate::flatten::STRIDE;
use crate::shape::FINE;
use crate::{ advance, enclose, flatten, idle, place, rush, settle, stretch, Catalog, Device, Document, Element, Enclosure, Error, History, Kind, Logic, Machine, Mode, Model, Netlist, Operation, Outline, Plan, Point, Reading, Registry, Segment, Shape, Solution };


const DIVISIONS : f64 = 20.0;


type Loading = ( Vec<Document>, Vec<( usize, Operation )>, Catalog );


fn centered( document : &Document, name : &str, drawing : &[ Shape ], layout : &[ Point ] ) -> Result<Operation, Error>
{
	let description = Operation::Describe { name : name.to_string(), drawing : drawing.to_vec(), layout : layout.to_vec() };

	if drawing.is_empty() { return Ok( description ); }

	let mut draft = document.clone();

	draft.apply( description )?;

	let outline = Outline::of( &draft );
	let middle = outline.middle();

	Ok( Operation::Describe { name : name.to_string(), drawing : drawing.iter().map( | shape | shape.shifted( - middle.x * FINE, - middle.y * FINE ) ).collect(), layout : outline.pins.iter().chain( &outline.lights ).map( | pin | Point { x : pin.x - middle.x, y : pin.y - middle.y } ).collect() } )
}

fn prefixed( prefix : &str, label : &str ) -> String
{
	if prefix.is_empty() || label.is_empty() { return label.to_string(); }

	format!( "{prefix}{label}" )
}

fn middle( segment : Segment ) -> Point
{
	Point { x : ( segment.from.x + segment.to.x ).div_euclid( 2 ), y : ( segment.from.y + segment.to.y ).div_euclid( 2 ) }
}


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
	lights : Vec<f64>,
	places : Vec<Vec<Option<usize>>>,
	glows : Vec<Option<usize>>,
	gates : Vec<Option<( usize, usize )>>,
	captions : Vec<String>,
	revision : u32,
	base : f64,
	step : f64,
	undos : Vec<Vec<( usize, Operation )>>,
	redos : Vec<Vec<( usize, Operation )>>,
	loading : Option<Loading>,
	measuring : Option<Catalog>,
}

impl Simulation
{
	pub fn new( step : f64 ) -> Simulation
	{
		Simulation { layers : vec![ Document::default() ], registry : Registry::default(), elements : Vec::new(), devices : Vec::new(), plan : Plan::default(), models : Vec::new(), sources : Vec::new(), focus : Vec::new(), history : History::default(), logic : Logic::default(), solution : Ok( Solution::default() ), lights : Vec::new(), places : Vec::new(), glows : Vec::new(), gates : Vec::new(), captions : Vec::new(), revision : 0, base : step, step, undos : Vec::new(), redos : Vec::new(), loading : None, measuring : None }
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

	pub fn amend( &mut self, layer : usize, operation : Operation ) -> Result<u32, Error>
	{
		let ( id, _ ) = self.perform( &[ ( layer, operation ) ] )?;

		self.redos.clear();

		Ok( id )
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
		let ids : HashSet<u32> = enclosure.inner.iter().map( | element | element.id ).collect();
		let others : Vec<Element> = document.elements().iter().filter( | element | ! ids.contains( &element.id ) ).copied().collect();
		let moved : Vec<Element> = enclosure.inner.iter().map( | element | Element { segment : element.segment.shifted( dx, dy ), ..*element } ).collect();
		let old : Vec<Point> = enclosure.inner.iter().flat_map( | element | self.terminals( element ) ).collect();
		let new : Vec<Point> = moved.iter().flat_map( | element | self.terminals( element ) ).collect();
		let mut operations = stretch( &others, &old, &new, Segment { from : low, to : high }.shifted( dx, dy ), | element | self.terminals( element ) );
		let mut draft = document.clone();
		let mut laid = HashSet::new();

		for operation in &operations { laid.insert( draft.apply( operation.clone() )? ); }

		let outer = | draft : &Document | -> Vec<Element> { draft.elements().iter().filter( | other | ! ids.contains( &other.id ) && ! laid.contains( &other.id ) ).copied().collect() };
		let mut outside = outer( &draft );

		for element in moved.iter().filter( | element | element.kind != Kind::Wire )
		{
			let mut cuts = place( &outside, element.kind, element.segment );

			cuts.pop();

			if cuts.is_empty() { continue; }

			for operation in &cuts { draft.apply( operation.clone() )?; }

			operations.extend( cuts );
			outside = outer( &draft );
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

	pub fn describe( &mut self, layer : usize, name : &str, drawing : &[ Shape ], layout : &[ Point ] ) -> Result<u32, Error>
	{
		let document = self.document( layer ).ok_or( Error::Layerless )?;
		let description = centered( document, name, drawing, layout )?;
		let mut draft = document.clone();

		draft.apply( description.clone() )?;

		let ( before, after ) = ( &Outline::of( document ), &Outline::of( &draft ) );
		let copy = | element : &Element | match element.kind { Kind::Symbol { layer : inner, rotation } if inner as usize == layer => Some( rotation ), _ => None };
		let moves = self.layers.iter().enumerate().flat_map( | ( index, other ) | other.elements().iter().filter_map( move | element |
		{
			let rotation = copy( element )?;
			let ( old, new ) = ( middle( before.zone( element.segment.from, rotation ) ), middle( after.zone( element.segment.from, rotation ) ) );

			if old == new { return None; }

			Some( ( index, Operation::Move { id : element.id, kind : element.kind, segment : element.segment.shifted( old.x - new.x, old.y - new.y ), mirrored : element.mirrored } ) )
		} ) );
		let edits : Vec<( usize, Operation )> = [ ( layer, description ) ].into_iter().chain( moves ).collect();

		self.commit( &edits )
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

		let focus = std::mem::take( &mut self.focus );

		self.layers.resize_with( count.max( needed ), Document::default );

		let result = self.commit( &edits );

		if result.is_err() { self.layers.truncate( count ); self.focus = focus; }

		result
	}

	pub fn load( &mut self, additions : Vec<( usize, Operation )> ) -> Result<(), Error>
	{
		let needed = additions.iter().map( | addition | addition.0 + 1 ).max().unwrap_or( 1 );
		let mut layers = vec![ Document::default(); needed ];

		self.loading = None;

		for ( layer, operation ) in &additions { layers[ *layer ].apply( operation.clone() )?; }

		let catalog = Catalog::new( &layers, &self.models, &self.sources, self.base );

		self.loading = Some( ( layers, additions, catalog ) );

		Ok( () )
	}

	pub fn proceed( &mut self ) -> Result<bool, Error>
	{
		let Some( ( layers, _, catalog ) ) = &mut self.loading else { return self.measure(); };

		if catalog.advance( layers ) { return Ok( true ); }

		let Some( ( _, additions, catalog ) ) = self.loading.take() else { return Ok( false ); };
		let previous = ( self.models.clone(), self.sources.clone() );

		( self.models, self.sources ) = catalog.finish();

		let result = self.open( additions );

		if result.is_err() { ( self.models, self.sources ) = previous; }

		result.map( | _ | false )
	}

	pub fn progress( &self ) -> f64
	{
		self.loading.as_ref().map( | ( _, _, catalog ) | catalog ).or( self.measuring.as_ref() ).map_or( 1.0, Catalog::progress )
	}

	pub fn frozen( &self ) -> bool
	{
		let Some( catalog ) = &self.measuring else { return false; };

		self.inside( self.descend( &self.focus ).unwrap_or( 0 ) ).into_iter().any( | layer | catalog.waits( layer ) )
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
		if self.frozen() { return; }

		if self.mode() == Mode::Digital { let solution = rush( Machine { elements : &self.elements, devices : &self.devices, plan : &self.plan, models : &self.models }, self.step, count, &mut self.logic ); self.show( solution ); return; }

		for _ in 0 .. count
		{
			if self.idle() { self.logic.time += self.step; continue; }

			let solution = advance( Machine { elements : &self.elements, devices : &self.devices, plan : &self.plan, models : &self.models }, &self.history, self.step, &mut self.logic );

			if let Ok( solution ) = &solution { self.history = History::record( &self.elements, solution ); }

			self.show( solution );

			if self.solution.is_err() { return; }
		}
	}

	pub fn idle( &self ) -> bool
	{
		self.solution.is_ok() && idle( &self.elements, &self.plan, self.step, &self.logic )
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

		self.layers[ layer ].elements().iter().filter( | element | matches!( element.kind, Kind::Toggle { .. } | Kind::Slider { .. } ) ).filter( probe ).map( | element | element.id ).collect()
	}

	pub fn captions( &self ) -> Vec<String>
	{
		self.captions.clone()
	}

	fn caption( &self ) -> Vec<String>
	{
		let document = &self.layers[ self.descend( &self.focus ).unwrap_or( 0 ) ];
		let mut elements = Vec::new();
		let mut pins = Vec::new();

		for element in document.elements()
		{
			let Kind::Symbol { layer, .. } = element.kind else
			{
				if element.kind.pin() { pins.push( ( element.segment.to, document.label( element.id ).to_string() ) ); }

				elements.push( *element );

				continue;
			};

			let inner = &self.layers[ layer as usize ];
			let prefix = document.label( element.id );
			let labels = inner.elements().iter().filter( | part | part.kind.pin() ).map( | part | prefixed( prefix, inner.label( part.id ) ) );

			for ( end, label ) in self.terminals( element ).into_iter().zip( labels )
			{
				pins.push( ( end, label ) );
				elements.push( Element { id : element.id, kind : Kind::Wire, segment : Segment { from : end, to : end }, mirrored : false } );
			}
		}

		let netlist = Netlist::build( &elements );
		let caption = | toggle : &Element | pins.iter().find( | ( point, label ) | ! label.is_empty() && netlist.net( *point ) == netlist.net( toggle.segment.to ) ).map_or( String::new(), | ( _, label ) | label.to_string() );

		document.elements().iter().filter( | element | matches!( element.kind, Kind::Toggle { .. } | Kind::Slider { .. } ) ).map( caption ).collect()
	}

	pub fn delay( &self, layer : usize ) -> Option<f64>
	{
		self.models.get( layer )?.as_ref().map( | model | model.delay )
	}

	pub fn solution( &self ) -> Result<&Solution, Error>
	{
		self.solution.as_ref().map_err( | error | *error )
	}

	pub fn lights( &self ) -> &[ f64 ]
	{
		&self.lights
	}

	pub fn revision( &self ) -> u32
	{
		self.revision
	}

	pub fn step( &self ) -> f64
	{
		self.step
	}

	pub fn period( &self ) -> Option<f64>
	{
		self.elements.iter().filter_map( | element | match element.kind { Kind::Clock { hertz, on : true } => Some( 1.0 / hertz ), _ => None } ).reduce( f64::min )
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

		let models = self.available();
		let ( elements, devices ) = flatten( &self.layers, &mut self.registry, &models, layer, path )?;

		self.focus = path.to_vec();

		self.rebuild( elements, devices );

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
		let mut inside : Vec<Operation> = parts.chain( labels ).collect();
		let mut inner = Document::default();

		for operation in &inside { inner.apply( operation.clone() )?; }

		let description = centered( &inner, name, drawing, layout )?;

		inner.apply( description.clone() )?;
		inside.push( description );

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
		let catalog = Catalog::new( &self.layers, &self.models, &self.sources, self.base );
		let symbolic = edits.iter().any( | ( layer, _ ) | *layer > 0 );
		let models = match &self.measuring { Some( running ) if ! symbolic => running.ready(), _ => catalog.ready() };
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

		self.focus = focus;

		if symbolic { self.measuring = Some( catalog ).filter( | catalog | ! catalog.done() ); }

		self.rebuild( elements, devices );

		Ok( ( id, inverses ) )
	}

	fn measure( &mut self ) -> Result<bool, Error>
	{
		let frozen = self.frozen();
		let Some( catalog ) = &mut self.measuring else { return Ok( false ); };
		let pending = catalog.advance( &self.layers );

		if let Some( catalog ) = self.measuring.take_if( | catalog | catalog.done() ) { ( self.models, self.sources ) = catalog.finish(); }

		if ! frozen || self.frozen() { return Ok( pending ); }

		let layer = self.descend( &self.focus )?;
		let models = self.available();
		let ( elements, devices ) = flatten( &self.layers, &mut self.registry, &models, layer, &self.focus )?;

		self.rebuild( elements, devices );

		Ok( pending )
	}

	fn available( &self ) -> Vec<Option<Model>>
	{
		self.measuring.as_ref().map_or_else( || self.models.clone(), Catalog::ready )
	}

	fn inside( &self, layer : usize ) -> Vec<usize>
	{
		let mut found : Vec<usize> = Vec::new();
		let mut open = vec![ layer ];

		while let Some( next ) = open.pop()
		{
			for element in self.layers.get( next ).map_or( &[][ .. ], Document::elements )
			{
				let Kind::Symbol { layer : inner, .. } = element.kind else { continue; };

				if found.contains( &( inner as usize ) ) { continue; }

				found.push( inner as usize );
				open.push( inner as usize );
			}
		}

		found
	}

	fn rebuild( &mut self, elements : Vec<Element>, devices : Vec<Device> )
	{
		self.plan = if self.frozen() { Plan::default() } else { Plan::build( &elements, &devices ) };
		self.elements = elements;
		self.devices = devices;
		self.revision = self.revision.wrapping_add( 1 );

		self.locate();
		self.start();
	}

	fn locate( &mut self )
	{
		let path = self.focus.as_slice();
		let layer = self.descend( path ).unwrap_or( 0 );
		let positions : HashMap<u32, usize> = self.elements.iter().enumerate().map( | ( index, element ) | ( element.id, index ) ).collect();
		let position = | key : &[ u32 ] | self.registry.find( key ).and_then( | id | positions.get( &id ).copied() );
		let symbols : Vec<( usize, u32, usize )> = self.layers[ layer ].elements().iter().enumerate().filter_map( | ( at, element ) | match element.kind { Kind::Symbol { layer : inner, .. } => Some( ( at, element.id, inner as usize ) ), _ => None } ).collect();

		self.glows = symbols.iter().flat_map( | &( _, id, inner ) | self.layers[ inner ].elements().iter().filter( | part | matches!( part.kind, Kind::Led { .. } ) ).map( move | led | ( id, led.id ) ) ).map( | ( id, led ) | position( &[ path, &[ id, led ] ].concat() ) ).collect();
		self.places = self.layers[ layer ].elements().iter().map( | element |
		{
			let key = [ path, &[ element.id ] ].concat();

			match element.kind
			{
				Kind::Symbol { layer : inner, .. } => self.layers[ inner as usize ].elements().iter().filter( | part | part.kind.pin() ).map( | pin | position( &[ key.as_slice(), &[ pin.id ] ].concat() ) ).collect(),
				_ => vec![ position( &key ) ],
			}
		} ).collect();

		let owners : HashMap<usize, usize> = self.devices.iter().enumerate().filter_map( | ( device, gate ) | gate.outputs.first().map( | &( _, resistor ) | ( resistor, device ) ) ).collect();

		self.gates = symbols.iter().flat_map( | &( at, _, inner ) |
		{
			let device = self.places[ at ].iter().flatten().find_map( | place | owners.get( place ).copied() );
			let count = self.layers[ inner ].elements().iter().filter( | part | matches!( part.kind, Kind::Led { .. } ) ).count();

			( 0 .. count ).map( move | rank | device.map( | device | ( device, rank ) ) )
		} ).collect();

		self.captions = self.caption();
	}

	fn edit( &mut self, layer : usize, operation : Operation ) -> Result<( u32, Operation ), Error>
	{
		let document = self.layers.get_mut( layer ).ok_or( Error::Layerless )?;
		let inverse = document.inverse( &operation )?;

		Ok( ( document.apply( operation )?, inverse ) )
	}

	fn start( &mut self )
	{
		if self.frozen() { self.solution = Ok( Solution::default() ); self.lights = Vec::new(); return; }

		self.step = self.period().map_or( self.base, | period | period / DIVISIONS );

		let solution = settle( Machine { elements : &self.elements, devices : &self.devices, plan : &self.plan, models : &self.models }, &self.history, self.step, &mut self.logic, true );

		self.show( solution );
	}

	fn show( &mut self, raw : Result<Solution, Error> )
	{
		self.lights = raw.as_ref().map_or_else( | _ | Vec::new(), | solution | self.glow( solution ) );

		let solution = match raw { Ok( solution ) => solution, Err( error ) => { self.solution = Err( error ); return; } };
		let mut shown = std::mem::replace( &mut self.solution, Ok( Solution::default() ) ).unwrap_or_default();

		self.view( &solution, &mut shown );
		self.solution = Ok( shown );
		self.logic.spare = solution;
	}

	fn glow( &self, solution : &Solution ) -> Vec<f64>
	{
		let lit = | ( device, rank ) : ( usize, usize ) | self.models.get( self.devices[ device ].layer ).and_then( Option::as_ref ).zip( self.logic.applied.get( device ) ).and_then( | ( model, &entry ) | model.lights.get( entry )?.get( rank ).copied() );

		self.glows.iter().zip( &self.gates ).map( | ( &index, &gate ) | index.and_then( | index | solution.readings.get( index ) ).map( | reading | reading.currents[ 0 ] ).or_else( || gate.and_then( lit ) ).unwrap_or( 0.0 ) ).collect()
	}

	fn view( &self, solution : &Solution, shown : &mut Solution )
	{
		let path = self.focus.as_slice();
		let layer = self.descend( path ).unwrap_or( 0 );
		let elements = self.layers[ layer ].elements();
		let reading = | index : Option<usize> | &solution.readings[ index.unwrap() ];

		shown.readings.resize_with( elements.len(), || Reading { potentials : Vec::new(), currents : Vec::new() } );

		for ( ( element, places ), target ) in elements.iter().zip( &self.places ).zip( &mut shown.readings )
		{
			let Kind::Symbol { .. } = element.kind else
			{
				let source = reading( places[ 0 ] );

				target.potentials.clone_from( &source.potentials );
				target.currents.clone_from( &source.currents );

				continue;
			};

			target.potentials.clear();
			target.currents.clear();
			target.potentials.extend( places.iter().map( | &index | reading( index ).potentials[ 1 ] ) );
			target.currents.extend( places.iter().map( | &index | reading( index ).currents[ 1 ] ) );
		}

		shown.branches.clear();
		shown.branches.extend( solution.branches.iter().filter( | branch | branch.segment.from.x.abs() < STRIDE / 2 ).copied() );
	}
}
