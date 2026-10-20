use simulator::{ Kind, Operation, Point, Segment, Simulation };
use wasm_bindgen::prelude::*;


const STEP : f64 = 1e-5;


#[wasm_bindgen]
pub struct Circuit
{
	simulation : Simulation,
}

#[wasm_bindgen]
impl Circuit
{
	#[wasm_bindgen( constructor )]
	pub fn new() -> Circuit
	{
		Circuit { simulation : Simulation::new( STEP ) }
	}

	pub fn snap( from_x : i32, from_y : i32, cursor_x : i32, cursor_y : i32 ) -> Vec<i32>
	{
		let point = Segment::snap( Point { x : from_x, y : from_y }, Point { x : cursor_x, y : cursor_y } );

		vec![ point.x, point.y ]
	}

	pub fn add( &mut self, code : u8, value : f64, from_x : i32, from_y : i32, to_x : i32, to_y : i32 ) -> Result<u32, JsError>
	{
		let kind = Circuit::kind( code, value )?;

		self.simulation.place( kind, Segment { from : Point { x : from_x, y : from_y }, to : Point { x : to_x, y : to_y } } ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	pub fn open( &mut self, data : &[ f64 ] ) -> Result<u32, JsError>
	{
		let parts = data.chunks_exact( 6 ).map( | row | Ok( ( Circuit::kind( row[ 0 ] as u8, row[ 1 ] )?, Segment { from : Point { x : row[ 2 ] as i32, y : row[ 3 ] as i32 }, to : Point { x : row[ 4 ] as i32, y : row[ 5 ] as i32 } } ) ) ).collect::<Result<Vec<_>, JsError>>()?;

		self.simulation.open( &parts ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	pub fn delete( &mut self, id : u32 ) -> Result<u32, JsError>
	{
		self.apply( Operation::Delete { id } )
	}

	#[wasm_bindgen( js_name = setValue )]
	pub fn set_value( &mut self, id : u32, value : f64 ) -> Result<u32, JsError>
	{
		self.apply( Operation::SetValue { id, value } )
	}

	#[wasm_bindgen( js_name = setClosed )]
	pub fn set_closed( &mut self, id : u32, closed : bool ) -> Result<u32, JsError>
	{
		self.apply( Operation::SetClosed { id, closed } )
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
		self.simulation.advance( count );
	}

	#[wasm_bindgen( js_name = elementsAt )]
	pub fn elements_at( &self, x : i32, y : i32 ) -> Vec<u32>
	{
		self.simulation.document().elements_at( Point { x, y } )
	}

	#[wasm_bindgen( js_name = branchAt )]
	pub fn branch_at( &self, x : i32, y : i32 ) -> Option<u32>
	{
		self.simulation.solution().ok()?.branch_at( Point { x, y } ).map( | index | index as u32 )
	}

	pub fn elements( &self ) -> Vec<f64>
	{
		let readings = self.simulation.solution().map( | solution | solution.readings.as_slice() ).unwrap_or( &[] );

		self.simulation.document().elements().iter().enumerate().flat_map( | ( index, element ) |
		{
			let ( code, value ) = match element.kind
			{
				Kind::Wire => ( 0.0, 0.0 ),
				Kind::Resistor { ohms } => ( 1.0, ohms ),
				Kind::Battery { volts } => ( 2.0, volts ),
				Kind::Ground => ( 3.0, 0.0 ),
				Kind::Capacitor { farads } => ( 4.0, farads ),
				Kind::Inductor { henries } => ( 5.0, henries ),
				Kind::Switch { closed } => ( 6.0, if closed { 1.0 } else { 0.0 } ),
				Kind::Diode => ( 7.0, 0.0 ),
				Kind::Npn => ( 8.0, 0.0 ),
				Kind::Pnp => ( 9.0, 0.0 ),
				Kind::Nmos => ( 10.0, 0.0 ),
				Kind::Pmos => ( 11.0, 0.0 ),
			};

			let mut potentials = [ 0.0, 0.0, f64::NAN ];
			let mut currents = [ f64::NAN; 3 ];

			if let Some( reading ) = readings.get( index )
			{
				potentials[ .. reading.potentials.len() ].copy_from_slice( &reading.potentials );
				currents[ .. reading.currents.len() ].copy_from_slice( &reading.currents );
			}

			let segment = element.segment;

			[ element.id as f64, code, segment.from.x as f64, segment.from.y as f64, segment.to.x as f64, segment.to.y as f64, value, potentials[ 0 ], potentials[ 1 ], potentials[ 2 ], currents[ 0 ], currents[ 1 ], currents[ 2 ] ]
		} ).collect()
	}

	pub fn branches( &self ) -> Vec<f64>
	{
		let Ok( solution ) = self.simulation.solution() else { return vec![]; };

		solution.branches.iter().flat_map( | branch |
		{
			let segment = branch.segment;

			[ segment.from.x as f64, segment.from.y as f64, segment.to.x as f64, segment.to.y as f64, branch.voltage, branch.current ]
		} ).collect()
	}

	pub fn error( &self ) -> Option<String>
	{
		self.simulation.solution().err().map( | error | error.to_string() )
	}

	fn apply( &mut self, operation : Operation ) -> Result<u32, JsError>
	{
		self.simulation.apply( operation ).map_err( | error | JsError::new( &error.to_string() ) )
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
			_ => return Err( JsError::new( "Unknown part kind." ) ),
		};

		Ok( kind )
	}
}
