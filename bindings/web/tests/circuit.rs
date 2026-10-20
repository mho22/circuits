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
	assert_eq!( Circuit::snap( 0, 0, 4, 5 ), vec![ 0, 5 ] );
}

#[wasm_bindgen_test]
fn it_lists_each_part_with_its_reading()
{
	let elements = powered().elements();

	assert_eq!( elements.len(), 65 );
	assert_eq!( &elements[ 0 .. 9 ], &[ 0.0, 2.0, 0.0, 2.0, 0.0, 0.0, 5.0, 0.0, 5.0 ] );
	assert!( elements[ 9 ].is_nan() );
	assert!( ( elements[ 10 ] - 0.005 ).abs() < 1e-12 );
	assert_eq!( &elements[ 13 .. 22 ], &[ 1.0, 1.0, 0.0, 0.0, 2.0, 0.0, 1000.0, 5.0, 0.0 ] );
	assert_eq!( &elements[ 52 .. 59 ], &[ 4.0, 3.0, 0.0, 2.0, 0.0, 2.0, 0.0 ] );
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
	assert!( ( circuit.elements()[ 23 ] - 0.01 ).abs() < 1e-12 );
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

	assert!( circuit.add( 12, 0.0, 0, 0, 1, 0 ).is_err() );
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
	assert!( circuit.elements()[ 10 ].is_nan() );
}

#[wasm_bindgen_test]
fn it_lists_a_capacitor_an_inductor_and_a_switch_with_their_values()
{
	let mut circuit = Circuit::new();

	circuit.add( 4, 1e-5, 0, 0, 2, 0 ).ok();
	circuit.add( 5, 1.0, 0, 2, 2, 2 ).ok();
	circuit.add( 6, 0.0, 0, 4, 2, 4 ).ok();

	let elements = circuit.elements();

	assert_eq!( [ elements[ 1 ], elements[ 6 ], elements[ 14 ], elements[ 19 ], elements[ 27 ], elements[ 32 ] ], [ 4.0, 1e-5, 5.0, 1.0, 6.0, 0.0 ] );
}

#[wasm_bindgen_test]
fn it_closes_a_switch_and_solves_again()
{
	let mut circuit = powered();

	circuit.add( 6, 0.0, 2, 2, 2, 4 ).ok();

	assert_eq!( circuit.set_closed( 5, true ).ok(), Some( 5 ) );
	assert_eq!( circuit.elements()[ 71 ], 1.0 );
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

	assert!( ( circuit.elements()[ 33 ] - 5.0 * ( 1.0 - ( -1.0_f64 ).exp() ) ).abs() < 1e-2 );
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
	let drop = elements[ 20 ] - elements[ 21 ];

	assert_eq!( elements[ 14 ], 7.0 );
	assert!( drop > 0.6 && drop < 0.7 );
}

#[wasm_bindgen_test]
fn it_lists_a_transistor_with_the_voltage_and_the_current_of_each_terminal()
{
	let mut circuit = Circuit::new();

	circuit.add( 2, 5.0, 0, 6, 0, 0 ).ok();
	circuit.add( 0, 0.0, 0, 0, 8, 0 ).ok();
	circuit.add( 1, 200_000.0, 4, 0, 4, 3 ).ok();
	circuit.add( 1, 1000.0, 7, 0, 7, 2 ).ok();
	circuit.add( 8, 0.0, 4, 3, 6, 3 ).ok();
	circuit.add( 0, 0.0, 7, 4, 7, 6 ).ok();
	circuit.add( 0, 0.0, 0, 6, 7, 6 ).ok();
	circuit.add( 3, 0.0, 0, 6, 0, 6 ).ok();

	let elements = circuit.elements();
	let transistor = &elements[ 52 .. 65 ];

	assert_eq!( transistor[ 1 ], 8.0 );
	assert!( transistor[ 7 ] > 0.6 && transistor[ 7 ] < 0.7 );
	assert_eq!( transistor[ 9 ], 0.0 );
	assert!( ( transistor[ 11 ] / transistor[ 10 ] - 100.0 ).abs() < 0.01 );
	assert!( ( transistor[ 10 ] + transistor[ 11 ] + transistor[ 12 ] ).abs() < 1e-12 );
	assert!( elements[ 9 ].is_nan() && elements[ 12 ].is_nan() );
	assert_eq!( circuit.elements_at( 7, 2 ), vec![ 4, 3 ] );
}

#[wasm_bindgen_test]
fn it_lists_a_mosfet_with_its_drain_current()
{
	let mut circuit = Circuit::new();

	circuit.add( 2, 5.0, 0, 6, 0, 0 ).ok();
	circuit.add( 0, 0.0, 0, 0, 8, 0 ).ok();
	circuit.add( 2, 2.0, 4, 6, 4, 3 ).ok();
	circuit.add( 1, 1000.0, 7, 0, 7, 2 ).ok();
	circuit.add( 10, 0.0, 4, 3, 6, 3 ).ok();
	circuit.add( 0, 0.0, 7, 4, 7, 6 ).ok();
	circuit.add( 0, 0.0, 0, 6, 7, 6 ).ok();
	circuit.add( 3, 0.0, 0, 6, 0, 6 ).ok();
	circuit.add( 11, 0.0, 10, 3, 12, 3 ).ok();

	let elements = circuit.elements();
	let transistor = &elements[ 52 .. 65 ];

	assert_eq!( transistor[ 1 ], 10.0 );
	assert_eq!( elements[ 105 ], 11.0 );
	assert!( ( transistor[ 11 ] - 0.0025 ).abs() < 1e-9 );
	assert!( transistor[ 10 ].abs() < 1e-9 );
}

#[wasm_bindgen_test]
fn it_opens_parts_in_place_of_the_drawing()
{
	let mut circuit = powered();

	circuit.open( &[ 1.0, 220.0, 0.0, 0.0, 2.0, 0.0, 6.0, 1.0, 2.0, 0.0, 2.0, 2.0 ] ).ok();

	let elements = circuit.elements();

	assert_eq!( elements.len(), 26 );
	assert_eq!( &elements[ 1 .. 7 ], &[ 1.0, 0.0, 0.0, 2.0, 0.0, 220.0 ] );
	assert_eq!( &elements[ 14 .. 20 ], &[ 6.0, 2.0, 0.0, 2.0, 2.0, 1.0 ] );
}

#[wasm_bindgen_test]
fn it_rejects_parts_it_cannot_place()
{
	let mut circuit = powered();

	assert!( circuit.open( &[ 12.0, 0.0, 0.0, 0.0, 2.0, 0.0 ] ).is_err() );
	assert!( circuit.open( &[ 1.0, 0.0, 0.0, 0.0, 2.0, 0.0 ] ).is_err() );
	assert_eq!( circuit.elements().len(), 65 );
}

#[wasm_bindgen_test]
fn it_lists_a_vcc_with_its_voltage_and_its_current()
{
	let mut circuit = Circuit::new();

	circuit.add( 12, 5.0, 0, 0, 0, 0 ).ok();
	circuit.add( 1, 1000.0, 0, 0, 0, 2 ).ok();
	circuit.add( 3, 0.0, 0, 2, 0, 2 ).ok();

	let elements = circuit.elements();

	assert_eq!( [ elements[ 1 ], elements[ 6 ], elements[ 7 ], elements[ 8 ] ], [ 12.0, 5.0, 0.0, 5.0 ] );
	assert!( ( elements[ 10 ] - 0.005 ).abs() < 1e-12 );
}

#[wasm_bindgen_test]
fn it_translates_a_part()
{
	let mut circuit = Circuit::new();

	circuit.add( 1, 1000.0, 0, 0, 2, 0 ).ok();
	circuit.translate( 0, 3, -1 ).ok();

	assert_eq!( &circuit.elements()[ 2 .. 6 ], &[ 3.0, -1.0, 5.0, -1.0 ] );
}

#[wasm_bindgen_test]
fn it_rotates_a_part_a_quarter_clockwise()
{
	let mut circuit = Circuit::new();

	circuit.add( 1, 1000.0, 0, 0, 4, 0 ).ok();
	circuit.rotate( 0 ).ok();

	assert_eq!( &circuit.elements()[ 2 .. 6 ], &[ 2.0, -2.0, 2.0, 2.0 ] );
}

#[wasm_bindgen_test]
fn it_mirrors_a_transistor_and_lists_it_as_mirrored()
{
	let mut circuit = Circuit::new();

	circuit.add( 8, 0.0, 0, 0, 2, 0 ).ok();
	circuit.mirror( 0, true ).ok();

	assert_eq!( &circuit.elements()[ 1 .. 7 ], &[ 8.0, 0.0, 0.0, 2.0, 0.0, 1.0 ] );
	assert_eq!( circuit.elements_at( 3, 1 ), vec![ 0 ] );

	circuit.mirror( 0, false ).ok();

	assert_eq!( &circuit.elements()[ 1 .. 7 ], &[ 8.0, 2.0, 0.0, 0.0, 0.0, 0.0 ] );
}

#[wasm_bindgen_test]
fn it_opens_a_mirrored_transistor()
{
	let mut circuit = Circuit::new();

	circuit.open( &[ 10.0, 1.0, 0.0, 0.0, 2.0, 0.0, 1.0, 1000.0, 4.0, 0.0, 6.0, 0.0 ] ).ok();

	let elements = circuit.elements();

	assert_eq!( [ elements[ 6 ], elements[ 19 ] ], [ 1.0, 1000.0 ] );
}

#[wasm_bindgen_test]
fn it_rejects_a_move_of_a_missing_part()
{
	let mut circuit = Circuit::new();

	assert!( circuit.translate( 7, 1, 0 ).is_err() );
	assert!( circuit.rotate( 7 ).is_err() );
	assert!( circuit.mirror( 7, false ).is_err() );
}
