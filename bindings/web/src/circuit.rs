use simulator::{ Document, Operation, Point, Wire };
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[derive( Default )]
pub struct Circuit
{
	document : Document,
}

#[wasm_bindgen]
impl Circuit
{
	#[wasm_bindgen( constructor )]
	pub fn new() -> Circuit
	{
		Circuit::default()
	}

	pub fn snap( from_x : i32, from_y : i32, cursor_x : i32, cursor_y : i32 ) -> Vec<i32>
	{
		let point = Wire::snap( Point { x : from_x, y : from_y }, Point { x : cursor_x, y : cursor_y } );

		vec![ point.x, point.y ]
	}

	#[wasm_bindgen( js_name = addWire )]
	pub fn add_wire( &mut self, from_x : i32, from_y : i32, to_x : i32, to_y : i32 ) -> Result<u32, JsError>
	{
		let operation = Operation::AddWire { from : Point { x : from_x, y : from_y }, to : Point { x : to_x, y : to_y } };

		self.document.apply( operation ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	#[wasm_bindgen( js_name = deleteWire )]
	pub fn delete_wire( &mut self, id : u32 ) -> Result<u32, JsError>
	{
		self.document.apply( Operation::DeleteWire { id } ).map_err( | error | JsError::new( &error.to_string() ) )
	}

	#[wasm_bindgen( js_name = wireAt )]
	pub fn wire_at( &self, x : i32, y : i32 ) -> Option<u32>
	{
		self.document.wire_at( Point { x, y } )
	}

	pub fn wires( &self ) -> Vec<i32>
	{
		self.document.wires().iter().flat_map( | wire | [ wire.from.x, wire.from.y, wire.to.x, wire.to.y ] ).collect()
	}
}
