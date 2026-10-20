use wasm_bindgen_test::wasm_bindgen_test;
use web::Circuit;


#[wasm_bindgen_test]
fn it_snaps_a_cursor_to_a_straight_end()
{
	assert_eq!( Circuit::snap( 0, 0, 4, 5 ), vec![ 5, 5 ] );
}

#[wasm_bindgen_test]
fn it_adds_and_lists_wires()
{
	let mut circuit = Circuit::new();

	assert_eq!( circuit.add_wire( 0, 0, 4, 0 ).ok(), Some( 0 ) );
	assert_eq!( circuit.add_wire( 4, 0, 4, 4 ).ok(), Some( 1 ) );

	assert_eq!( circuit.wires(), vec![ 0, 0, 4, 0, 4, 0, 4, 4 ] );
}

#[wasm_bindgen_test]
fn it_rejects_a_crooked_wire()
{
	let mut circuit = Circuit::new();

	assert!( circuit.add_wire( 0, 0, 2, 1 ).is_err() );
	assert!( circuit.wires().is_empty() );
}

#[wasm_bindgen_test]
fn it_deletes_the_wire_at_a_point()
{
	let mut circuit = Circuit::new();

	circuit.add_wire( 0, 0, 4, 0 ).ok();

	let id = circuit.wire_at( 2, 0 ).unwrap();

	assert_eq!( circuit.delete_wire( id ).ok(), Some( 0 ) );
	assert_eq!( circuit.wire_at( 2, 0 ), None );
	assert!( circuit.delete_wire( id ).is_err() );
}
