use simulator::{ mix, Branch, Document, Element, Kind, Mode, Operation, Outline, Point, Reading, Segment, Shape, Simulation, Solution };
use wasm_bindgen::prelude::*;


const STEP : f64 = 1e-5;
const SPEED : f64 = 1e-2;
const REAL : f64 = 1.0;
const EYE : f64 = 1.0 / 125.0;
const HOLD : f64 = 1.0 / 16.0;
const ROW : usize = 13;
const LEAK : f64 = 1e-6;


fn points( data : &[ i32 ] ) -> Vec<Point>
{
	data.chunks_exact( 2 ).map( | pair | Point { x : pair[ 0 ], y : pair[ 1 ] } ).collect()
}

fn text( codes : &[ i32 ] ) -> Option<String>
{
	codes.iter().map( | &code | u32::try_from( code ).ok().and_then( char::from_u32 ) ).collect()
}

fn shapes( data : &[ i32 ] ) -> Result<Vec<Shape>, JsError>
{
	let mut shapes = Vec::new();
	let mut rest = data;

	while let [ code, tail @ .. ] = rest
	{
		let ( size, shape ) = match ( code, tail )
		{
			( 1, [ x, y, radius, .. ] ) => ( 3, Shape::Circle { center : Point { x : *x, y : *y }, radius : *radius } ),
			( 2, [ from_x, from_y, to_x, to_y, .. ] ) => ( 4, Shape::Line { from : Point { x : *from_x, y : *from_y }, to : Point { x : *to_x, y : *to_y } } ),
			( 3, [ from_x, from_y, through_x, through_y, to_x, to_y, .. ] ) => ( 6, Shape::Curve { from : Point { x : *from_x, y : *from_y }, through : Point { x : *through_x, y : *through_y }, to : Point { x : *to_x, y : *to_y } } ),
			( 4, [ x, y, length, codes @ .. ] ) if ( 0 ..= codes.len() as i32 ).contains( length ) =>
			{
				let content = text( &codes[ .. *length as usize ] ).ok_or_else( || JsError::new( "Unknown shape." ) )?;

				( 3 + *length as usize, Shape::Text { at : Point { x : *x, y : *y }, content } )
			},
			_ => return Err( JsError::new( "Unknown shape." ) ),
		};

		shapes.push( shape );
		rest = &tail[ size .. ];
	}

	Ok( shapes )
}

fn flat( shapes : &[ Shape ] ) -> Vec<i32>
{
	shapes.iter().flat_map( | shape | match shape
	{
		Shape::Circle { center, radius } => vec![ 1, center.x, center.y, *radius ],
		Shape::Line { from, to } => vec![ 2, from.x, from.y, to.x, to.y ],
		Shape::Curve { from, through, to } => vec![ 3, from.x, from.y, through.x, through.y, to.x, to.y ],
		Shape::Text { at, content } => [ 4, at.x, at.y, content.chars().count() as i32 ].into_iter().chain( content.chars().map( | character | character as i32 ) ).collect(),
	} ).collect()
}


#[wasm_bindgen]
pub struct Circuit
{
	simulation : Simulation,
	clipboard : Vec<( Element, String )>,
	seen : Option<( u32, Solution, Vec<f64>, [ f64; 2 ] )>,
	held : Vec<( f64, f64 )>,
}

#[wasm_bindgen]
impl Circuit
{
	#[wasm_bindgen( constructor )]
	pub fn new() -> Circuit
	{
		Circuit { simulation : Simulation::new( STEP ), clipboard : Vec::new(), seen : None, held : Vec::new() }
	}

	pub fn snap( from_x : i32, from_y : i32, cursor_x : i32, cursor_y : i32 ) -> Vec<i32>
	{
		let point = Segment::snap( Point { x : from_x, y : from_y }, Point { x : cursor_x, y : cursor_y } );

		vec![ point.x, point.y ]
	}

	pub fn add( &mut self, code : u8, value : f64, from_x : i32, from_y : i32, to_x : i32, to_y : i32 ) -> Result<u32, JsError>
	{
		let kind = Circuit::kind( code, value )?;

		self.simulation.place( self.current(), kind, Segment { from : Point { x : from_x, y : from_y }, to : Point { x : to_x, y : to_y } } ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	pub fn load( &mut self, data : &[ f64 ], labels : Vec<String>, layers : &[ u32 ], names : Vec<String>, drawings : &[ i32 ], layouts : &[ i32 ] ) -> Result<(), JsError>
	{
		let mut counts : Vec<u32> = Vec::new();
		let mut edits = Vec::new();

		for ( row, label ) in data.chunks_exact( 8 ).zip( labels.into_iter().chain( std::iter::repeat( String::new() ) ) )
		{
			let layer = row[ 0 ] as usize;
			let kind = match Circuit::kind( row[ 1 ] as u8, row[ 2 ] )?
			{
				Kind::Symbol { layer, .. } => Kind::Symbol { layer, rotation : row[ 3 ] as u8 },
				Kind::Clock { hertz, .. } => Kind::Clock { hertz, on : row[ 3 ] != 0.0 },
				kind => kind,
			};
			let mirrored = matches!( kind, Kind::Npn | Kind::Pnp | Kind::Nmos | Kind::Pmos ) && row[ 2 ] != 0.0;

			counts.resize( counts.len().max( layer + 1 ), 0 );

			let id = counts[ layer ];
			let segment = Segment { from : Point { x : row[ 4 ] as i32, y : row[ 5 ] as i32 }, to : Point { x : row[ 6 ] as i32, y : row[ 7 ] as i32 } };

			counts[ layer ] += 1;
			edits.push( ( layer, Operation::Insert { element : Element { id, kind, segment, mirrored }, index : id as usize } ) );

			if kind.labeled() && ! label.is_empty() { edits.push( ( layer, Operation::Label { id, label } ) ); }
		}

		let mut rest = drawings;
		let mut places = layouts;

		for ( &layer, name ) in layers.iter().zip( names )
		{
			let [ length, tail @ .. ] = rest else { return Err( JsError::new( "Unknown shape." ) ); };
			let length = usize::try_from( *length ).ok().filter( | &length | length <= tail.len() ).ok_or_else( || JsError::new( "Unknown shape." ) )?;
			let [ count, others @ .. ] = places else { return Err( JsError::new( "Unknown layout." ) ); };
			let count = usize::try_from( *count ).ok().filter( | &count | 2 * count <= others.len() ).ok_or_else( || JsError::new( "Unknown layout." ) )?;

			edits.push( ( layer as usize, Operation::Describe { name, drawing : shapes( &tail[ .. length ] )?, layout : points( &others[ .. 2 * count ] ) } ) );

			rest = &tail[ length .. ];
			places = &others[ 2 * count .. ];
		}

		self.simulation.load( edits ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	pub fn proceed( &mut self ) -> Result<bool, JsError>
	{
		self.simulation.proceed().map_err( | error | JsError::new( &error.to_string() ) )
	}

	#[wasm_bindgen( getter )]
	pub fn progress( &self ) -> f64
	{
		self.simulation.progress()
	}

	#[wasm_bindgen( getter )]
	pub fn frozen( &self ) -> bool
	{
		self.simulation.frozen()
	}

	pub fn enclose( &mut self, area : &[ i32 ], name : String, drawing : &[ i32 ], layout : &[ i32 ] ) -> Result<u32, JsError>
	{
		let &[ low_x, low_y, high_x, high_y ] = area else { return Err( JsError::new( "Unknown area." ) ); };
		let drawing = shapes( drawing )?;

		self.simulation.enclose( self.current(), Point { x : low_x, y : low_y }, Point { x : high_x, y : high_y }, &name, &drawing, &points( layout ) ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	pub fn shift( &mut self, low_x : i32, low_y : i32, high_x : i32, high_y : i32, dx : i32, dy : i32 ) -> Result<u32, JsError>
	{
		self.simulation.shift( self.current(), Point { x : low_x, y : low_y }, Point { x : high_x, y : high_y }, dx, dy ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	pub fn describe( &mut self, name : String, drawing : &[ i32 ], layout : &[ i32 ] ) -> Result<u32, JsError>
	{
		let drawing = shapes( drawing )?;

		self.simulation.describe( self.current(), &name, &drawing, &points( layout ) ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	pub fn purge( &mut self ) -> Result<u32, JsError>
	{
		self.simulation.purge( self.current() ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	pub fn name( &self, layer : u32 ) -> String
	{
		self.simulation.document( layer as usize ).map_or( String::new(), | document | document.name().to_string() )
	}

	pub fn drawing( &self, layer : u32 ) -> Vec<i32>
	{
		self.simulation.document( layer as usize ).map_or( Vec::new(), | document | flat( document.drawing() ) )
	}

	pub fn layout( &self, layer : u32 ) -> Vec<i32>
	{
		self.simulation.document( layer as usize ).map_or( Vec::new(), | document | document.layout().iter().flat_map( | point | [ point.x, point.y ] ).collect() )
	}

	pub fn delay( &self, layer : u32 ) -> f64
	{
		self.simulation.delay( layer as usize ).unwrap_or( f64::NAN )
	}

	pub fn labels( &self, layer : u32 ) -> Vec<String>
	{
		let Some( document ) = self.simulation.document( layer as usize ) else { return Vec::new(); };

		document.elements().iter().filter( | element | element.kind.pin() ).map( | element | document.label( element.id ).to_string() ).collect()
	}

	#[wasm_bindgen( js_name = symbolLabels )]
	pub fn symbol_labels( &self, layer : u32 ) -> Vec<String>
	{
		let Some( document ) = self.simulation.document( layer as usize ) else { return Vec::new(); };

		document.elements().iter().filter( | element | matches!( element.kind, Kind::Symbol { .. } ) ).map( | element | document.label( element.id ).to_string() ).collect()
	}

	pub fn enter( &mut self, id : u32 ) -> Result<(), JsError>
	{
		let path = [ self.simulation.path(), &[ id ] ].concat();

		self.simulation.focus( &path ).map_err( | _ | JsError::new( "No symbol has this id." ) )
	}

	pub fn leave( &mut self )
	{
		let outer = self.simulation.path().split_last().map_or( Vec::new(), | ( _, outer ) | outer.to_vec() );

		self.simulation.focus( &outer ).ok();
	}

	#[wasm_bindgen( getter )]
	pub fn rate( &self ) -> f64
	{
		let speed = if self.simulation.period().is_some() { REAL } else { SPEED };

		speed / self.simulation.step()
	}

	#[wasm_bindgen( getter )]
	pub fn hertz( &self ) -> Option<f64>
	{
		self.simulation.period().map( | period | 1.0 / period )
	}

	#[wasm_bindgen( getter )]
	pub fn mode( &self ) -> String
	{
		self.simulation.mode().to_string()
	}

	#[wasm_bindgen( getter )]
	pub fn layer( &self ) -> u32
	{
		self.current() as u32
	}

	#[wasm_bindgen( getter )]
	pub fn trail( &self ) -> Vec<u32>
	{
		let path = self.simulation.path();

		( 1 ..= path.len() ).filter_map( | depth | self.simulation.descend( &path[ .. depth ] ).ok() ).map( | layer | layer as u32 ).collect()
	}

	pub fn contents( &self, layer : u32 ) -> Vec<f64>
	{
		self.simulation.document( layer as usize ).map_or( Vec::new(), | document | self.rows( document, &[] ) )
	}

	pub fn symbols( &self ) -> Vec<u32>
	{
		( 1 .. ).map_while( | layer | self.simulation.document( layer ).map( | document | ( layer, document ) ) ).filter( | ( _, document ) | ! document.elements().is_empty() ).map( | ( layer, _ ) | layer as u32 ).collect()
	}

	pub fn outline( &self, layer : u32 ) -> Vec<i32>
	{
		let Some( document ) = self.simulation.document( layer as usize ) else { return vec![]; };

		let colors = document.elements().iter().filter_map( | element | match element.kind { Kind::Led { color } => Some( color as i32 ), _ => None } );
		let outline = Outline::of( document );
		let lights = outline.lights.iter().zip( colors ).flat_map( | ( point, color ) | [ point.x, point.y, color ] );

		[ outline.low, outline.high ].iter().flat_map( | point | [ point.x, point.y ] ).chain( [ outline.pins.len() as i32 ] ).chain( outline.pins.iter().flat_map( | point | [ point.x, point.y ] ) ).chain( lights ).collect()
	}

	pub fn copy( &mut self, ids : &[ u32 ] ) -> usize
	{
		let document = self.document();

		self.clipboard = document.elements().iter().filter( | element | ids.contains( &element.id ) ).map( | element | ( *element, document.label( element.id ).to_string() ) ).collect();

		self.clipboard.len()
	}

	pub fn paste( &mut self, x : i32, y : i32 ) -> Result<u32, JsError>
	{
		let low_x = self.clipboard.iter().map( | ( element, _ ) | element.segment.from.x.min( element.segment.to.x ) ).min().unwrap_or( x );
		let low_y = self.clipboard.iter().map( | ( element, _ ) | element.segment.from.y.min( element.segment.to.y ) ).min().unwrap_or( y );

		self.simulation.paste( self.current(), &self.clipboard, x - low_x, y - low_y ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	pub fn delete( &mut self, id : u32 ) -> Result<u32, JsError>
	{
		self.apply( Operation::Delete { id } )
	}

	pub fn erase( &mut self, ids : &[ u32 ] ) -> Result<u32, JsError>
	{
		self.simulation.erase( self.current(), ids ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	#[wasm_bindgen( js_name = setValue )]
	pub fn set_value( &mut self, id : u32, value : f64 ) -> Result<u32, JsError>
	{
		self.apply( Operation::SetValue { id, value } )
	}

	#[wasm_bindgen( js_name = amendValue )]
	pub fn amend_value( &mut self, id : u32, value : f64 ) -> Result<u32, JsError>
	{
		self.simulation.amend( self.current(), Operation::SetValue { id, value } ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	#[wasm_bindgen( js_name = setClosed )]
	pub fn set_closed( &mut self, id : u32, closed : bool ) -> Result<u32, JsError>
	{
		self.apply( Operation::SetClosed { id, closed } )
	}

	#[wasm_bindgen( js_name = setHigh )]
	pub fn set_high( &mut self, id : u32, high : bool ) -> Result<u32, JsError>
	{
		self.apply( Operation::SetHigh { id, high } )
	}

	#[wasm_bindgen( js_name = setLabel )]
	pub fn set_label( &mut self, id : u32, label : String ) -> Result<u32, JsError>
	{
		self.apply( Operation::Label { id, label } )
	}

	pub fn translate( &mut self, id : u32, dx : i32, dy : i32 ) -> Result<u32, JsError>
	{
		let element = self.element( id )?;

		self.transform( id, element.kind, element.segment.shifted( dx, dy ), element.mirrored )
	}

	pub fn rotate( &mut self, id : u32 ) -> Result<u32, JsError>
	{
		let element = self.element( id )?;

		if let Kind::Symbol { layer, rotation } = element.kind { return self.transform( id, Kind::Symbol { layer, rotation : ( rotation + 1 ) % 4 }, element.segment, element.mirrored ); }

		self.transform( id, element.kind, element.segment.turned(), element.mirrored )
	}

	pub fn mirror( &mut self, id : u32, vertical : bool ) -> Result<u32, JsError>
	{
		let element = self.element( id )?;

		self.transform( id, element.kind, element.segment.mirrored( vertical ), ! element.mirrored )
	}

	pub fn undo( &mut self ) -> Result<Option<u32>, JsError>
	{
		self.simulation.undo().map_err( | error | JsError::new( &error.to_string() ) )
	}

	pub fn redo( &mut self ) -> Result<Option<u32>, JsError>
	{
		self.simulation.redo().map_err( | error | JsError::new( &error.to_string() ) )
	}

	pub fn advance( &mut self, count : u32 )
	{
		if self.simulation.frozen() { return; }

		if self.simulation.mode() == Mode::Digital
		{
			self.simulation.advance( count );

			if self.simulation.solution().is_err() { self.seen = None; return; }

			self.blend( count );

			return;
		}

		let mut steps = 0;

		for _ in 0 .. count
		{
			if ! self.simulation.idle() { self.blend( steps ); steps = 0; }

			self.simulation.advance( 1 );
			steps += 1;

			if self.simulation.solution().is_err() { self.seen = None; return; }
		}

		self.blend( steps );
	}

	#[wasm_bindgen( js_name = elementsAt )]
	pub fn elements_at( &self, x : i32, y : i32 ) -> Vec<u32>
	{
		self.simulation.elements_at( self.current(), Point { x, y } )
	}

	#[wasm_bindgen( js_name = branchAt )]
	pub fn branch_at( &self, x : i32, y : i32 ) -> Vec<f64>
	{
		let found = self.simulation.solution().ok().and_then( | solution | solution.branch_at( Point { x, y } ) );

		found.and_then( | index | self.shown()?.branches.get( index ) ).map_or( Vec::new(), | branch | Circuit::row( branch ).to_vec() )
	}

	#[wasm_bindgen( getter )]
	pub fn revision( &self ) -> u32
	{
		self.simulation.revision()
	}

	pub fn elements( &self ) -> Vec<f64>
	{
		let readings = self.shown().map_or( &[][ .. ], | solution | solution.readings.as_slice() );

		self.rows( self.document(), readings )
	}

	pub fn readings( &self, indexes : &[ u32 ] ) -> Vec<f64>
	{
		let readings = self.shown().map_or( &[][ .. ], | solution | solution.readings.as_slice() );
		let elements = self.document().elements();
		let mut values = Vec::new();

		for &index in indexes
		{
			let Some( element ) = elements.get( index as usize ) else { continue; };
			let reading = readings.get( index as usize );
			let count = self.count( element, reading );

			values.push( count as f64 );

			Circuit::read( reading, count, &mut values );
		}

		values
	}

	pub fn scale( &self ) -> Vec<f64>
	{
		let Ok( solution ) = self.simulation.solution() else { return vec![ 0.0, 0.0 ]; };

		let [ peak, current ] = self.blended().map_or_else( || solution.peaks(), | ( _, _, _, peaks ) | *peaks );

		vec![ if peak < LEAK { 0.0 } else { peak }, current ]
	}

	pub fn branches( &self, low_x : i32, low_y : i32, high_x : i32, high_y : i32 ) -> Vec<f64>
	{
		let Some( solution ) = self.shown() else { return vec![]; };

		let inside = | segment : Segment | segment.from.x.max( segment.to.x ) >= low_x && segment.from.x.min( segment.to.x ) <= high_x && segment.from.y.max( segment.to.y ) >= low_y && segment.from.y.min( segment.to.y ) <= high_y;

		solution.branches.iter().filter( | branch | inside( branch.segment ) ).flat_map( Circuit::row ).collect()
	}

	pub fn listening( &self ) -> Vec<u32>
	{
		self.simulation.listening()
	}

	pub fn lights( &self ) -> Vec<f64>
	{
		let lights = self.simulation.lights();

		self.seen.as_ref().map( | ( _, _, glow, _ ) | glow ).filter( | glow | glow.len() == lights.len() ).map_or( lights, | glow | glow ).to_vec()
	}

	pub fn captions( &self ) -> Vec<String>
	{
		self.simulation.captions()
	}

	pub fn error( &self ) -> Option<String>
	{
		self.simulation.solution().err().map( | error | error.to_string() )
	}

	fn rows( &self, document : &Document, readings : &[ Reading ] ) -> Vec<f64>
	{
		let mut rows = Vec::with_capacity( document.elements().len() * ROW );

		for ( index, element ) in document.elements().iter().enumerate()
		{
			let mirrored = if element.mirrored { 1.0 } else { 0.0 };

			let ( code, value, rotation ) = match element.kind
			{
				Kind::Wire => ( 0.0, 0.0, 0 ),
				Kind::Resistor { ohms } => ( 1.0, ohms, 0 ),
				Kind::Battery { volts } => ( 2.0, volts, 0 ),
				Kind::Ground => ( 3.0, 0.0, 0 ),
				Kind::Capacitor { farads } => ( 4.0, farads, 0 ),
				Kind::Inductor { henries } => ( 5.0, henries, 0 ),
				Kind::Switch { closed } => ( 6.0, if closed { 1.0 } else { 0.0 }, 0 ),
				Kind::Diode => ( 7.0, 0.0, 0 ),
				Kind::Npn => ( 8.0, mirrored, 0 ),
				Kind::Pnp => ( 9.0, mirrored, 0 ),
				Kind::Nmos => ( 10.0, mirrored, 0 ),
				Kind::Pmos => ( 11.0, mirrored, 0 ),
				Kind::Supply { volts } => ( 12.0, volts, 0 ),
				Kind::Input { high } => ( 13.0, if high { 1.0 } else { 0.0 }, 0 ),
				Kind::Symbol { layer, rotation } => ( 14.0, layer as f64, rotation ),
				Kind::Output => ( 15.0, 0.0, 0 ),
				Kind::Toggle { high } => ( 16.0, if high { 1.0 } else { 0.0 }, 0 ),
				Kind::Led { color } => ( 17.0, color as f64, 0 ),
				Kind::Clock { hertz, on } => ( 18.0, hertz, on as u8 ),
				Kind::Slider { volts } => ( 19.0, volts, 0 ),
			};

			let segment = element.segment;
			let reading = readings.get( index );
			let count = self.count( element, reading );

			rows.extend( [ element.id as f64, code, segment.from.x as f64, segment.from.y as f64, segment.to.x as f64, segment.to.y as f64, value, rotation as f64, count as f64 ] );

			Circuit::read( reading, count, &mut rows );
		}

		rows
	}

	fn count( &self, element : &Element, reading : Option<&Reading> ) -> usize
	{
		reading.map_or_else( || self.simulation.terminals( element ).len(), | reading | reading.potentials.len() )
	}

	fn read( reading : Option<&Reading>, count : usize, values : &mut Vec<f64> )
	{
		if let Some( reading ) = reading
		{
			values.extend( &reading.potentials );
			values.extend( &reading.currents );

			return;
		}

		values.extend( std::iter::repeat_n( 0.0, count ) );
		values.extend( std::iter::repeat_n( f64::NAN, count ) );
	}

	fn row( branch : &Branch ) -> [ f64; 6 ]
	{
		let segment = branch.segment;

		[ segment.from.x as f64, segment.from.y as f64, segment.to.x as f64, segment.to.y as f64, branch.voltage, branch.current ]
	}

	fn blend( &mut self, steps : u32 )
	{
		let ( Ok( solution ), true ) = ( self.simulation.solution(), steps > 0 ) else { return; };
		let revision = self.simulation.revision();
		let lights = self.simulation.lights();
		let elapsed = f64::from( steps ) / self.rate();
		let weight = 1.0 - ( - elapsed / EYE ).exp();

		let Some( ( stamp, seen, glow, peaks ) ) = self.seen.as_mut().filter( | ( _, _, glow, _ ) | glow.len() == lights.len() ) else
		{
			self.seen = Some( ( revision, solution.clone(), lights.to_vec(), solution.peaks() ) );
			self.held = lights.iter().map( | &light | ( light, 0.0 ) ).collect();

			return;
		};

		if *stamp != revision && ! seen.fits( solution ) { seen.clone_from( solution ); }

		*stamp = revision;
		*peaks = seen.blend( solution, weight );

		for ( ( from, &to ), held ) in glow.iter_mut().zip( lights ).zip( &mut self.held )
		{
			*held = if to > LEAK { ( to, 0.0 ) } else { ( held.0, held.1 + elapsed ) };
			*from = mix( *from, if held.1 <= HOLD { held.0 } else { to }, weight );
		}
	}

	fn shown( &self ) -> Option<&Solution>
	{
		let solution = self.simulation.solution().ok()?;

		Some( self.blended().map_or( solution, | ( _, seen, _, _ ) | seen ) )
	}

	fn blended( &self ) -> Option<&( u32, Solution, Vec<f64>, [ f64; 2 ] )>
	{
		let solution = self.simulation.solution().ok()?;

		self.seen.as_ref().filter( | ( stamp, seen, _, _ ) | *stamp == self.simulation.revision() || seen.fits( solution ) )
	}

	fn current( &self ) -> usize
	{
		self.simulation.descend( self.simulation.path() ).unwrap_or( 0 )
	}

	fn document( &self ) -> &Document
	{
		self.simulation.document( self.current() ).unwrap()
	}

	fn element( &self, id : u32 ) -> Result<Element, JsError>
	{
		self.document().elements().iter().find( | element | element.id == id ).copied().ok_or_else( || JsError::new( "No part has this id." ) )
	}

	fn transform( &mut self, id : u32, kind : Kind, segment : Segment, mirrored : bool ) -> Result<u32, JsError>
	{
		self.simulation.transform( self.current(), id, kind, segment, mirrored ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	fn apply( &mut self, operation : Operation ) -> Result<u32, JsError>
	{
		self.simulation.apply( self.current(), operation ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	fn kind( code : u8, value : f64 ) -> Result<Kind, JsError>
	{
		let kind = match code
		{
			0 => Kind::Wire,
			1 => Kind::Resistor { ohms : value },
			2 => Kind::Battery { volts : value },
			3 => Kind::Ground,
			4 => Kind::Capacitor { farads : value },
			5 => Kind::Inductor { henries : value },
			6 => Kind::Switch { closed : value != 0.0 },
			7 => Kind::Diode,
			8 => Kind::Npn,
			9 => Kind::Pnp,
			10 => Kind::Nmos,
			11 => Kind::Pmos,
			12 => Kind::Supply { volts : value },
			13 => Kind::Input { high : value != 0.0 },
			14 => Kind::Symbol { layer : value as u32, rotation : 0 },
			15 => Kind::Output,
			16 => Kind::Toggle { high : value != 0.0 },
			17 => Kind::Led { color : value as u8 },
			18 => Kind::Clock { hertz : value, on : true },
			19 => Kind::Slider { volts : value },
			_ => return Err( JsError::new( "Unknown part kind." ) ),
		};

		Ok( kind )
	}
}

impl Default for Circuit
{
	fn default() -> Circuit
	{
		Circuit::new()
	}
}
