use wasm_bindgen_test::wasm_bindgen_test;
use web::Circuit;


fn powered() -> Circuit
{
	let mut circuit = Circuit::new();

	circuit.add( 2, 5.0, 0, 2, 0, 0 ).ok();
	circuit.add( 1, 1000.0, 0, 0, 2, 0 ).ok();
	circuit.add( 0, 0.0, 2, 0, 2, 2 ).ok();
	circuit.add( 0, 0.0, 2, 2, 0, 2 ).ok();
	circuit.add( 3, 0.0, 0, 2, 0, 2 ).ok();

	circuit
}


#[wasm_bindgen_test]
fn it_snaps_a_cursor_to_a_straight_end()
{
	assert_eq!( Circuit::snap( 0, 0, 4, 5 ), vec![ 5, 5 ] );
}

#[wasm_bindgen_test]
fn it_lists_each_part_with_its_reading()
{
	let elements = powered().elements();

	assert_eq!( elements.len(), 50 );
	assert_eq!( &elements[ 0 .. 9 ], &[ 0.0, 2.0, 0.0, 2.0, 0.0, 0.0, 5.0, 0.0, 5.0 ] );
	assert!( ( elements[ 9 ] - 0.005 ).abs() < 1e-12 );
	assert_eq!( &elements[ 10 .. 19 ], &[ 1.0, 1.0, 0.0, 0.0, 2.0, 0.0, 1000.0, 5.0, 0.0 ] );
	assert_eq!( &elements[ 40 .. 47 ], &[ 4.0, 3.0, 0.0, 2.0, 0.0, 2.0, 0.0 ] );
}

#[wasm_bindgen_test]
fn it_lists_each_piece_of_wire_with_its_reading()
{
	let branches = powered().branches();

	assert_eq!( branches.len(), 12 );
	assert_eq!( &branches[ 0 .. 5 ], &[ 2.0, 0.0, 2.0, 2.0, 0.0 ] );
	assert!( ( branches[ 5 ].abs() - 0.005 ).abs() < 1e-12 );
}

#[wasm_bindgen_test]
fn it_finds_parts_and_branches_at_a_point()
{
	let circuit = powered();

	assert_eq!( circuit.elements_at( 1, 0 ), vec![ 1 ] );
	assert_eq!( circuit.branch_at( 1, 2 ), Some( 1 ) );
	assert_eq!( circuit.branch_at( 9, 9 ), None );
}

#[wasm_bindgen_test]
fn it_sets_a_value_and_solves_again()
{
	let mut circuit = powered();

	assert_eq!( circuit.set_value( 0, 10.0 ).ok(), Some( 0 ) );
	assert!( ( circuit.elements()[ 19 ] - 0.01 ).abs() < 1e-12 );
}

#[wasm_bindgen_test]
fn it_deletes_a_part()
{
	let mut circuit = powered();

	assert_eq!( circuit.delete( 1 ).ok(), Some( 1 ) );
	assert!( circuit.elements_at( 1, 0 ).is_empty() );
	assert!( circuit.delete( 1 ).is_err() );
}

#[wasm_bindgen_test]
fn it_rejects_an_invalid_part()
{
	let mut circuit = Circuit::new();

	assert!( circuit.add( 9, 0.0, 0, 0, 1, 0 ).is_err() );
	assert!( circuit.add( 0, 0.0, 0, 0, 2, 1 ).is_err() );
	assert!( circuit.elements().is_empty() );
}

#[wasm_bindgen_test]
fn it_reports_a_circuit_without_solution()
{
	let mut circuit = Circuit::new();

	assert_eq!( circuit.error(), None );

	circuit.add( 2, 5.0, 0, 0, 0, 2 ).ok();
	circuit.add( 0, 0.0, 0, 0, 0, 2 ).ok();

	assert!( circuit.error().unwrap().starts_with( "The circuit has no single solution" ) );
	assert!( circuit.branches().is_empty() );
	assert_eq!( circuit.branch_at( 0, 1 ), None );
	assert!( circuit.elements()[ 9 ].is_nan() );
}

#[wasm_bindgen_test]
fn it_lists_a_capacitor_an_inductor_and_a_switch_with_their_values()
{
	let mut circuit = Circuit::new();

	circuit.add( 4, 1e-5, 0, 0, 2, 0 ).ok();
	circuit.add( 5, 1.0, 0, 2, 2, 2 ).ok();
	circuit.add( 6, 0.0, 0, 4, 2, 4 ).ok();

	let elements = circuit.elements();

	assert_eq!( [ elements[ 1 ], elements[ 6 ], elements[ 11 ], elements[ 16 ], elements[ 21 ], elements[ 26 ] ], [ 4.0, 1e-5, 5.0, 1.0, 6.0, 0.0 ] );
}

#[wasm_bindgen_test]
fn it_closes_a_switch_and_solves_again()
{
	let mut circuit = powered();

	circuit.add( 6, 0.0, 2, 2, 2, 4 ).ok();

	assert_eq!( circuit.set_closed( 5, true ).ok(), Some( 5 ) );
	assert_eq!( circuit.elements()[ 56 ], 1.0 );
	assert!( circuit.set_closed( 1, true ).is_err() );
}

#[wasm_bindgen_test]
fn it_advances_the_time_step_by_step()
{
	let mut circuit = Circuit::new();

	circuit.add( 2, 5.0, 0, 2, 0, 0 ).ok();
	circuit.add( 1, 1000.0, 0, 0, 2, 0 ).ok();
	circuit.add( 4, 1e-5, 2, 0, 2, 2 ).ok();
	circuit.add( 0, 0.0, 2, 2, 0, 2 ).ok();

	circuit.advance( 1000 );

	assert!( ( circuit.elements()[ 27 ] - 5.0 * ( 1.0 - ( -1.0_f64 ).exp() ) ).abs() < 1e-2 );
}

#[wasm_bindgen_test]
fn it_undoes_and_redoes_a_deletion()
{
	let mut circuit = powered();

	circuit.delete( 1 ).ok();

	assert_eq!( circuit.undo().ok(), Some( Some( 1 ) ) );
	assert_eq!( circuit.elements_at( 1, 0 ), vec![ 1 ] );
	assert_eq!( circuit.redo().ok(), Some( Some( 1 ) ) );
	assert!( circuit.elements_at( 1, 0 ).is_empty() );
	assert_eq!( circuit.redo().ok(), Some( None ) );
}

#[wasm_bindgen_test]
fn it_splits_a_wire_under_a_new_part()
{
	let mut circuit = Circuit::new();

	circuit.add( 0, 0.0, 0, 0, 6, 0 ).ok();

	assert_eq!( circuit.add( 1, 1000.0, 2, 0, 4, 0 ).ok(), Some( 3 ) );
	assert_eq!( circuit.elements_at( 1, 0 ), vec![ 1 ] );
	assert_eq!( circuit.elements_at( 3, 0 ), vec![ 3 ] );
	assert_eq!( circuit.elements_at( 5, 0 ), vec![ 2 ] );
}

#[wasm_bindgen_test]
fn it_lists_a_diode_with_its_forward_drop()
{
	let mut circuit = Circuit::new();

	circuit.add( 2, 5.0, 0, 2, 0, 0 ).ok();
	circuit.add( 7, 0.0, 0, 0, 2, 0 ).ok();
	circuit.add( 1, 1000.0, 2, 0, 2, 2 ).ok();
	circuit.add( 0, 0.0, 2, 2, 0, 2 ).ok();

	let elements = circuit.elements();
	let drop = elements[ 17 ] - elements[ 18 ];

	assert_eq!( elements[ 11 ], 7.0 );
	assert!( drop > 0.6 && drop < 0.7 );
}
