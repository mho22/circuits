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

fn laddered() -> Circuit
{
	let mut circuit = Circuit::new();

	circuit.add( 2, 5.0, 0, 6, 0, 0 ).ok();
	circuit.add( 0, 0.0, 0, 0, 2, 0 ).ok();
	circuit.add( 13, 0.0, 2, 1, 2, 0 ).ok();
	circuit.add( 1, 1000.0, 2, 0, 6, 0 ).ok();
	circuit.add( 15, 0.0, 6, 1, 6, 0 ).ok();
	circuit.add( 0, 0.0, 6, 0, 8, 0 ).ok();
	circuit.add( 1, 1000.0, 8, 0, 8, 6 ).ok();
	circuit.add( 0, 0.0, 8, 6, 0, 6 ).ok();
	circuit.add( 3, 0.0, 0, 6, 0, 6 ).ok();

	circuit
}

fn enclosed() -> ( Circuit, u32 )
{
	let mut circuit = laddered();
	let symbol = circuit.enclose( 1, -2, 7, 2, String::new(), &[], &[] ).ok().unwrap();

	( circuit, symbol )
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
	assert_eq!( &elements[ 0 .. 11 ], &[ 0.0, 2.0, 0.0, 2.0, 0.0, 0.0, 5.0, 0.0, 2.0, 0.0, 5.0 ] );
	assert!( ( elements[ 11 ] - 0.005 ).abs() < 1e-12 );
	assert_eq!( &elements[ 13 .. 24 ], &[ 1.0, 1.0, 0.0, 0.0, 2.0, 0.0, 1000.0, 0.0, 2.0, 5.0, 0.0 ] );
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
	assert!( ( circuit.elements()[ 24 ] - 0.01 ).abs() < 1e-12 );
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
fn it_erases_several_parts_and_brings_them_back_with_one_undo()
{
	let mut circuit = powered();

	assert_eq!( circuit.erase( &[ 1, 2 ] ).ok(), Some( 2 ) );
	assert_eq!( circuit.elements().len(), 39 );
	assert!( circuit.erase( &[] ).is_err() );

	circuit.undo().ok();

	assert_eq!( circuit.elements().len(), 65 );
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
	assert!( circuit.elements()[ 11 ].is_nan() );
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

	assert!( ( circuit.elements()[ 35 ] - 5.0 * ( 1.0 - ( -1.0_f64 ).exp() ) ).abs() < 1e-2 );
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
	let drop = elements[ 22 ] - elements[ 23 ];

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
	let transistor = &elements[ 52 .. 67 ];

	assert_eq!( transistor[ 1 ], 8.0 );
	assert_eq!( transistor[ 8 ], 3.0 );
	assert!( transistor[ 9 ] > 0.6 && transistor[ 9 ] < 0.7 );
	assert_eq!( transistor[ 11 ], 0.0 );
	assert!( ( transistor[ 13 ] / transistor[ 12 ] - 100.0 ).abs() < 0.01 );
	assert!( ( transistor[ 12 ] + transistor[ 13 ] + transistor[ 14 ] ).abs() < 1e-12 );
	assert_eq!( elements[ 8 ], 2.0 );
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
	let transistor = &elements[ 52 .. 67 ];

	assert_eq!( transistor[ 1 ], 10.0 );
	assert_eq!( elements[ 107 ], 11.0 );
	assert!( ( transistor[ 13 ] - 0.0025 ).abs() < 1e-9 );
	assert!( transistor[ 12 ].abs() < 1e-9 );
}

#[wasm_bindgen_test]
fn it_opens_parts_in_place_of_the_drawing()
{
	let mut circuit = powered();

	circuit.open( &[ 0.0, 1.0, 220.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 6.0, 1.0, 0.0, 2.0, 0.0, 2.0, 2.0 ], Vec::new(), &[], Vec::new(), &[], &[] ).ok();

	let elements = circuit.elements();

	assert_eq!( elements.len(), 26 );
	assert_eq!( &elements[ 1 .. 7 ], &[ 1.0, 0.0, 0.0, 2.0, 0.0, 220.0 ] );
	assert_eq!( &elements[ 14 .. 20 ], &[ 6.0, 2.0, 0.0, 2.0, 2.0, 1.0 ] );
}

#[wasm_bindgen_test]
fn it_rejects_parts_it_cannot_place()
{
	let mut circuit = powered();

	assert!( circuit.open( &[ 0.0, 12.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0 ], Vec::new(), &[], Vec::new(), &[], &[] ).is_err() );
	assert!( circuit.open( &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0 ], Vec::new(), &[], Vec::new(), &[], &[] ).is_err() );
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

	assert_eq!( [ elements[ 1 ], elements[ 6 ], elements[ 9 ], elements[ 10 ] ], [ 12.0, 5.0, 0.0, 5.0 ] );
	assert!( ( elements[ 11 ] - 0.005 ).abs() < 1e-12 );
}

#[wasm_bindgen_test]
fn it_lists_a_toggle_with_its_level_and_switches_it()
{
	let mut circuit = Circuit::new();

	circuit.add( 16, 1.0, 0, -2, 0, 0 ).ok();
	circuit.add( 1, 1000.0, 0, 0, 0, 2 ).ok();
	circuit.add( 3, 0.0, 0, 2, 0, 2 ).ok();

	let elements = circuit.elements();

	assert_eq!( [ elements[ 1 ], elements[ 6 ], elements[ 9 ], elements[ 10 ] ], [ 16.0, 1.0, 0.0, 5.0 ] );
	assert_eq!( circuit.set_high( 0, false ).ok(), Some( 0 ) );
	assert_eq!( [ circuit.elements()[ 6 ], circuit.elements()[ 10 ] ], [ 0.0, 0.0 ] );
}

#[wasm_bindgen_test]
fn it_lists_an_led_with_its_forward_drop()
{
	let mut circuit = Circuit::new();

	circuit.add( 2, 5.0, 0, 2, 0, 0 ).ok();
	circuit.add( 17, 0.0, 0, 0, 2, 0 ).ok();
	circuit.add( 1, 300.0, 2, 0, 2, 2 ).ok();
	circuit.add( 0, 0.0, 2, 2, 0, 2 ).ok();

	let elements = circuit.elements();
	let drop = elements[ 22 ] - elements[ 23 ];

	assert_eq!( elements[ 14 ], 17.0 );
	assert!( drop > 1.8 && drop < 2.0 );
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

	circuit.open( &[ 0.0, 10.0, 1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 1.0, 1000.0, 0.0, 4.0, 0.0, 6.0, 0.0 ], Vec::new(), &[], Vec::new(), &[], &[] ).ok();

	let elements = circuit.elements();

	assert_eq!( [ elements[ 6 ], elements[ 21 ] ], [ 1.0, 1000.0 ] );
}

#[wasm_bindgen_test]
fn it_rejects_a_move_of_a_missing_part()
{
	let mut circuit = Circuit::new();

	assert!( circuit.translate( 7, 1, 0 ).is_err() );
	assert!( circuit.rotate( 7 ).is_err() );
	assert!( circuit.mirror( 7, false ).is_err() );
}

#[wasm_bindgen_test]
fn it_encloses_a_selection_into_a_symbol_with_one_reading_per_pin()
{
	let ( circuit, symbol ) = enclosed();
	let elements = circuit.elements();
	let row = elements.chunks( 13 ).find( | row | row[ 0 ] == symbol as f64 ).unwrap();

	assert_eq!( &row[ 1 .. 9 ], &[ 14.0, 4.0, 0.0, 4.0, 0.0, 1.0, 0.0, 2.0 ] );
	assert!( ( row[ 11 ] - 0.0025 ).abs() < 1e-12 );
	assert_eq!( circuit.symbols(), vec![ 1 ] );
	assert_eq!( circuit.outline( 1 ), vec![ -2, -2, 2, 2, -2, 0, 2, 0 ] );
	assert!( circuit.outline( 2 ).is_empty() );
	assert_eq!( circuit.elements_at( 5, 1 ), vec![ symbol ] );
}

#[wasm_bindgen_test]
fn it_shifts_the_parts_inside_an_area()
{
	let mut circuit = laddered();

	assert!( circuit.shift( 20, 20, 22, 22, 1, 0 ).is_err() );
	assert!( circuit.shift( 7, -1, 9, 7, 2, 0 ).is_ok() );
	assert_eq!( circuit.elements_at( 10, 3 ), vec![ 6 ] );
	assert!( circuit.undo().is_ok() );
	assert_eq!( circuit.elements_at( 8, 3 ), vec![ 6 ] );
}

#[wasm_bindgen_test]
fn it_enters_a_symbol_and_reads_the_parts_of_its_layer()
{
	let ( mut circuit, symbol ) = enclosed();

	assert!( circuit.enter( 0 ).is_err() );
	assert_eq!( circuit.layer(), 0 );
	assert!( circuit.enter( symbol ).is_ok() );
	assert_eq!( circuit.layer(), 1 );

	let elements = circuit.elements();

	assert_eq!( elements.len(), 39 );
	assert_eq!( &elements[ 14 .. 22 ], &[ 1.0, 2.0, 0.0, 6.0, 0.0, 1000.0, 0.0, 2.0 ] );
	assert!( elements[ 24 ].abs() < 1e-12 );
	assert!( circuit.branches().is_empty() );
	assert_eq!( circuit.branch_at( 3, 0 ), None );
	assert_eq!( circuit.elements_at( 4, 0 ), vec![ 1 ] );
	assert_eq!( circuit.set_value( 1, 500.0 ).ok(), Some( 1 ) );

	circuit.leave();

	assert_eq!( circuit.elements_at( 4, 0 ), vec![ 9 ] );
	assert_eq!( circuit.contents( 1 ).len(), 39 );
	assert!( circuit.contents( 1 )[ 24 ].is_nan() );
}

#[wasm_bindgen_test]
fn it_leaves_a_symbol_that_an_undo_removes()
{
	let ( mut circuit, symbol ) = enclosed();

	circuit.enter( symbol ).ok();
	circuit.undo().ok();

	assert_eq!( circuit.layer(), 0 );
	assert_eq!( circuit.elements().chunks( 13 ).filter( | row | row[ 1 ] == 14.0 ).count(), 0 );
}

#[wasm_bindgen_test]
fn it_describes_the_symbol_it_is_in()
{
	let ( mut circuit, symbol ) = enclosed();

	circuit.enter( symbol ).ok();

	assert!( circuit.describe( "foo".to_string(), &[ 1, 0, 0, 8 ], &[ -3, 0, 3, 0 ] ).is_ok() );
	assert_eq!( [ circuit.name( 1 ), circuit.drawing( 1 ).iter().map( | value | value.to_string() ).collect::<Vec<_>>().join( "," ) ], [ "foo".to_string(), "1,0,0,8".to_string() ] );
	assert_eq!( circuit.layout( 1 ), vec![ -3, 0, 3, 0 ] );
	assert!( circuit.describe( String::new(), &[ 9 ], &[] ).is_err() );
}

#[wasm_bindgen_test]
fn it_purges_the_symbol_it_is_in_and_leaves_it()
{
	let ( mut circuit, symbol ) = enclosed();

	circuit.enter( symbol ).ok();

	assert!( circuit.purge().is_ok() );
	assert_eq!( circuit.layer(), 0 );
	assert!( circuit.symbols().is_empty() );
	assert!( circuit.purge().is_err() );

	circuit.undo().ok();

	assert_eq!( circuit.symbols(), vec![ 1 ] );
}

#[wasm_bindgen_test]
fn it_rotates_a_symbol_a_quarter_in_place()
{
	let ( mut circuit, symbol ) = enclosed();

	circuit.rotate( symbol ).ok();

	let elements = circuit.elements();
	let row = elements.chunks( 13 ).find( | row | row[ 0 ] == symbol as f64 ).unwrap();

	assert_eq!( &row[ 1 .. 8 ], &[ 14.0, 4.0, 0.0, 4.0, 0.0, 1.0, 1.0 ] );
}

#[wasm_bindgen_test]
fn it_opens_a_symbol_with_four_pins_and_its_layer()
{
	let mut circuit = Circuit::new();
	let mut data = vec![ 0.0, 14.0, 1.0, 1.0, 5.0, 5.0, 5.0, 5.0 ];

	for y in [ 0.0, 2.0 ] { data.extend( [ 1.0, 13.0, 0.0, 0.0, -1.0, y, 0.0, y, 1.0, 15.0, 0.0, 0.0, 5.0, y, 4.0, y ] ); }

	circuit.open( &data, Vec::new(), &[], Vec::new(), &[], &[] ).ok();
	circuit.enter( 0 ).ok();
	circuit.open( &data, Vec::new(), &[], Vec::new(), &[], &[] ).ok();

	assert_eq!( circuit.layer(), 0 );

	let elements = circuit.elements();

	assert_eq!( elements.len(), 17 );
	assert_eq!( &elements[ 1 .. 9 ], &[ 14.0, 5.0, 5.0, 5.0, 5.0, 1.0, 1.0, 4.0 ] );
	assert_eq!( circuit.symbols(), vec![ 1 ] );
}

#[wasm_bindgen_test]
fn it_encloses_a_named_symbol_with_its_drawing_and_its_layout()
{
	let mut circuit = laddered();

	assert!( circuit.enclose( 1, -2, 7, 2, String::new(), &[ 3 ], &[] ).is_err() );
	assert!( circuit.enclose( 1, -2, 7, 2, String::new(), &[ 2, 2, 0, 0 ], &[] ).is_err() );
	assert!( circuit.enclose( 1, -2, 7, 2, String::new(), &[ 2, -1, 0, 0 ], &[] ).is_err() );
	assert!( circuit.enclose( 1, -2, 7, 2, String::new(), &[ 1, 0, 0, 0 ], &[] ).is_err() );

	let drawing = [ 0, -4, -4, 4, 4, 1, 0, 0, 2, 2, -1, 0, 1, 1 ];
	let symbol = circuit.enclose( 1, -2, 7, 2, "foo".to_string(), &drawing, &[ -3, 0, 3, 0 ] ).ok().unwrap();

	assert_eq!( ( circuit.name( 1 ), circuit.drawing( 1 ), circuit.layout( 1 ) ), ( "foo".to_string(), drawing.to_vec(), vec![ -3, 0, 3, 0 ] ) );
	assert_eq!( ( circuit.name( 0 ), circuit.drawing( 9 ), circuit.layout( 9 ) ), ( String::new(), Vec::new(), Vec::new() ) );
	assert_eq!( circuit.elements_at( 1, 0 ), vec![ symbol, 9 ] );
}

#[wasm_bindgen_test]
fn it_opens_the_names_the_drawings_the_layouts_and_the_labels_of_the_layers()
{
	let mut circuit = Circuit::new();
	let data = [ 0.0, 14.0, 1.0, 0.0, 5.0, 5.0, 5.0, 5.0, 1.0, 13.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 15.0, 0.0, 0.0, 5.0, 0.0, 4.0, 0.0 ];

	circuit.open( &data, vec![ String::new(), "foo".to_string(), "bar".to_string() ], &[ 1 ], vec![ "baz".to_string() ], &[ 4, 1, 0, 0, 2 ], &[ 2, -3, 0, 3, 0 ] ).ok();

	let values : Vec<f64> = circuit.contents( 1 ).chunks( 13 ).map( | row | row[ 6 ] ).collect();

	assert_eq!( ( circuit.name( 1 ), circuit.drawing( 1 ), circuit.layout( 1 ), circuit.labels( 1 ) ), ( "baz".to_string(), vec![ 1, 0, 0, 2 ], vec![ -3, 0, 3, 0 ], vec![ "foo".to_string(), "bar".to_string() ] ) );
	assert_eq!( values, vec![ 1.0, 0.0 ] );
	assert!( circuit.open( &data, Vec::new(), &[ 1 ], vec![ String::new() ], &[ 5, 1, 0, 0, 2 ], &[ 0 ] ).is_err() );
	assert!( circuit.open( &data, Vec::new(), &[ 1 ], vec![ String::new() ], &[ 4, 1, 0, 0, 0 ], &[ 0 ] ).is_err() );
	assert!( circuit.open( &data, Vec::new(), &[ 1 ], vec![ String::new() ], &[], &[ 0 ] ).is_err() );
	assert!( circuit.open( &data, Vec::new(), &[ 1 ], vec![ String::new() ], &[ 0 ], &[ 3, -3, 0, 3, 0 ] ).is_err() );
	assert!( circuit.open( &data, Vec::new(), &[ 1 ], vec![ String::new() ], &[ 0 ], &[] ).is_err() );
	assert_eq!( circuit.name( 1 ), "baz" );
}

#[wasm_bindgen_test]
fn it_opens_sets_and_lists_the_level_of_a_free_input()
{
	let mut circuit = Circuit::new();
	let data = [ 0.0, 14.0, 1.0, 0.0, 5.0, 5.0, 5.0, 5.0, 1.0, 13.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1000.0, 0.0, 0.0, 0.0, 4.0, 0.0, 1.0, 15.0, 0.0, 0.0, 5.0, 0.0, 4.0, 0.0 ];

	circuit.open( &data, Vec::new(), &[], Vec::new(), &[], &[] ).ok();
	circuit.enter( 0 ).ok();

	let reading = | circuit : &Circuit | ( circuit.elements()[ 6 ], circuit.elements()[ 9 ] );

	assert_eq!( reading( &circuit ), ( 1.0, 5.0 ) );
	assert_eq!( circuit.set_high( 0, false ).ok(), Some( 0 ) );
	assert_eq!( reading( &circuit ), ( 0.0, 0.0 ) );
	assert!( circuit.set_high( 2, true ).is_err() );
}
