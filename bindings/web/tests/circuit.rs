use wasm_bindgen::JsError;
use wasm_bindgen_test::wasm_bindgen_test;
use web::Circuit;


fn open( circuit : &mut Circuit, data : &[ f64 ], labels : Vec<String>, layers : &[ u32 ], names : Vec<String>, drawings : &[ i32 ], layouts : &[ i32 ] ) -> Result<(), JsError>
{
	circuit.load( data, labels, layers, names, drawings, layouts )?;

	while circuit.proceed()? {}

	Ok( () )
}

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

fn blinking( hertz : f64 ) -> Vec<f64>
{
	let mut circuit = Circuit::new();

	circuit.add( 18, hertz, 0, -2, 0, 0 ).ok();
	circuit.add( 1, 1000.0, 0, 0, 4, 0 ).ok();
	circuit.add( 17, 0.0, 4, 0, 4, 4 ).ok();
	circuit.add( 3, 0.0, 4, 4, 4, 4 ).ok();

	( 0 .. 2000 ).map( | _ |
	{
		circuit.advance( 1 );

		let elements = circuit.elements();
		let led = 2 * ( 9 + 2 * 2 );

		elements[ led + 9 + elements[ led + 8 ] as usize ]
	} ).collect()
}

fn glowing( hertz : f64 ) -> Vec<f64>
{
	let mut circuit = Circuit::new();
	let data = [ 0.0, 14.0, 1.0, 0.0, 5.0, 5.0, 5.0, 5.0, 0.0, 18.0, hertz, 0.0, 1.0, 5.0, 3.0, 5.0, 1.0, 13.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1000.0, 0.0, 0.0, 0.0, 4.0, 0.0, 1.0, 17.0, 1.0, 0.0, 4.0, 0.0, 4.0, 4.0, 1.0, 3.0, 0.0, 0.0, 4.0, 4.0, 4.0, 4.0 ];

	open( &mut circuit, &data, Vec::new(), &[ 1 ], vec![ "foo".to_string() ], &[ 0 ], &[ 2, -2, 0, 0, 2 ] ).ok();

	( 0 .. 2000 ).map( | _ | { circuit.advance( 1 ); circuit.lights()[ 0 ] } ).collect()
}

fn inverting( hertz : f64 ) -> ( String, Vec<f64> )
{
	let mut circuit = Circuit::new();
	let data = [ 0.0, 14.0, 1.0, 0.0, 5.0, 5.0, 5.0, 5.0, 0.0, 18.0, hertz, 0.0, 1.0, 5.0, 3.0, 5.0, 1.0, 13.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 12.0, 5.0, 0.0, 3.0, -1.0, 3.0, -1.0, 1.0, 15.0, 0.0, 0.0, 6.0, 0.0, 5.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -2.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 1.0, 11.0, 0.0, 0.0, 0.0, -2.0, 2.0, -2.0, 1.0, 10.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 1.0, 0.0, 0.0, 0.0, 3.0, -3.0, 5.0, -3.0, 1.0, 0.0, 0.0, 0.0, 5.0, -3.0, 5.0, 0.0, 1.0, 0.0, 0.0, 0.0, 5.0, 0.0, 5.0, 1.0, 1.0, 0.0, 0.0, 0.0, 5.0, 1.0, 3.0, 1.0, 1.0, 3.0, 0.0, 0.0, 3.0, 3.0, 3.0, 3.0, 1.0, 1.0, 1000.0, 0.0, 5.0, 0.0, 9.0, 0.0, 1.0, 17.0, 1.0, 0.0, 9.0, 0.0, 9.0, 4.0, 1.0, 0.0, 0.0, 0.0, 9.0, 4.0, 9.0, 6.0, 1.0, 0.0, 0.0, 0.0, 9.0, 6.0, -2.0, 6.0, 1.0, 0.0, 0.0, 0.0, -2.0, 6.0, -2.0, 0.0, 1.0, 0.0, 0.0, 0.0, -2.0, 0.0, 0.0, 0.0 ];

	open( &mut circuit, &data, Vec::new(), &[ 1 ], vec![ "foo".to_string() ], &[ 0 ], &[ 3, -2, 0, 2, 0, 0, 2 ] ).ok();

	( circuit.mode(), ( 0 .. 200 ).map( | _ | { circuit.advance( 10 ); circuit.lights()[ 0 ] } ).collect() )
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
	let symbol = circuit.enclose( &[ 1, -2, 7, 2 ], String::new(), &[], &[] ).ok().unwrap();

	while circuit.proceed().ok().unwrap() {}

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
	let branches = powered().branches( -9, -9, 9, 9 );

	assert_eq!( branches.len(), 12 );
	assert_eq!( &branches[ 0 .. 5 ], &[ 2.0, 0.0, 2.0, 2.0, 0.0 ] );
	assert!( ( branches[ 5 ].abs() - 0.005 ).abs() < 1e-12 );
}

#[wasm_bindgen_test]
fn it_lists_only_the_pieces_of_wire_that_touch_an_area()
{
	let circuit = powered();

	assert_eq!( circuit.branches( 0, 2, 1, 3 ), circuit.branches( -9, -9, 9, 9 )[ 6 .. 12 ] );
	assert_eq!( circuit.branches( 2, 2, 2, 2 ).len(), 12 );
	assert!( circuit.branches( 3, 0, 9, 9 ).is_empty() );
}

#[wasm_bindgen_test]
fn it_reads_the_count_and_the_values_of_the_parts_at_a_list_of_indexes_and_skips_a_missing_index()
{
	let circuit = powered();
	let elements = circuit.elements();

	assert_eq!( circuit.readings( &[ 1, 9, 0 ] ), [ &elements[ 21 .. 26 ], &elements[ 8 .. 13 ] ].concat() );
}

#[wasm_bindgen_test]
fn it_gives_the_peak_voltage_and_the_peak_current()
{
	let scale = powered().scale();

	assert!( ( scale[ 0 ] - 5.0 ).abs() < 1e-12 );
	assert!( ( scale[ 1 ] - 0.005 ).abs() < 1e-12 );
}

#[wasm_bindgen_test]
fn it_counts_a_new_revision_after_a_change_of_the_parts_only()
{
	let mut circuit = powered();
	let revision = circuit.revision();

	circuit.advance( 10 );

	assert_eq!( circuit.revision(), revision );

	circuit.add( 0, 0.0, 2, 0, 4, 0 ).ok();

	assert_ne!( circuit.revision(), revision );
}

#[wasm_bindgen_test]
fn it_finds_parts_and_branches_at_a_point()
{
	let circuit = powered();

	assert_eq!( circuit.elements_at( 1, 0 ), vec![ 1 ] );
	assert_eq!( circuit.branch_at( 1, 2 ), circuit.branches( -9, -9, 9, 9 )[ 6 .. 12 ] );
	assert!( circuit.branch_at( 9, 9 ).is_empty() );
}

#[wasm_bindgen_test]
fn it_sets_a_value_and_solves_again()
{
	let mut circuit = powered();

	assert_eq!( circuit.set_value( 0, 10.0 ).ok(), Some( 0 ) );
	assert!( ( circuit.elements()[ 24 ] - 0.01 ).abs() < 1e-12 );
}

#[wasm_bindgen_test]
fn it_blends_from_the_shown_solution_after_a_change_of_a_value()
{
	let mut circuit = powered();

	circuit.advance( 10 );
	circuit.set_value( 0, 10.0 ).ok();

	assert!( ( circuit.elements()[ 24 ] - 0.005 ).abs() < 1e-12 );
	assert!( ( circuit.scale()[ 1 ] - 0.005 ).abs() < 1e-12 );

	circuit.advance( 1000 );

	assert!( ( circuit.elements()[ 24 ] - 0.01 ).abs() < 1e-12 );
	assert!( ( circuit.scale()[ 1 ] - 0.01 ).abs() < 1e-12 );
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
fn it_pastes_the_copied_parts_with_their_corner_on_a_point_and_takes_them_back_with_one_undo()
{
	let mut circuit = powered();

	assert_eq!( circuit.copy( &[ 1, 2, 9 ] ), 2 );
	assert!( circuit.paste( 10, 10 ).is_ok() );
	assert_eq!( circuit.elements().len(), 7 * 13 );
	assert_eq!( circuit.elements_at( 12, 12 ).len(), 1 );

	circuit.undo().ok();

	assert_eq!( circuit.elements().len(), 5 * 13 );
	assert_eq!( circuit.copy( &[] ), 0 );
	assert!( circuit.paste( 10, 10 ).is_err() );
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
	assert!( circuit.branches( -9, -9, 9, 9 ).is_empty() );
	assert!( circuit.branch_at( 0, 1 ).is_empty() );
	assert_eq!( circuit.scale(), [ 0.0, 0.0 ] );
	assert!( circuit.elements()[ 11 ].is_nan() );
	assert!( circuit.readings( &[ 0 ] )[ 4 ].is_nan() );
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

	circuit.advance( 5000 );

	assert!( ( circuit.elements()[ 35 ] - 5.0 * ( 1.0 - ( -5.0_f64 ).exp() ) ).abs() < 1e-2 );
}

#[wasm_bindgen_test]
fn it_lists_a_clock_with_its_frequency_and_runs_it_in_real_time()
{
	let mut circuit = Circuit::new();

	assert!( ( circuit.rate() - 1000.0 ).abs() < 1e-9 );
	assert_eq!( circuit.mode(), "Analog" );

	circuit.add( 18, 1e7, 0, -2, 0, 0 ).ok();

	assert_eq!( [ circuit.elements()[ 1 ], circuit.elements()[ 6 ] ], [ 18.0, 1e7 ] );
	assert!( ( circuit.rate() - 2e8 ).abs() < 1e-3 );

	circuit.set_value( 0, 0.1 ).ok();

	assert!( ( circuit.rate() - 2.0 ).abs() < 1e-9 );
}

#[wasm_bindgen_test]
fn it_gives_the_frequency_of_the_fastest_clock()
{
	let mut circuit = Circuit::new();

	assert_eq!( circuit.hertz(), None );

	circuit.add( 18, 8.0, 0, -2, 0, 0 ).ok();
	circuit.add( 18, 64.0, 0, 2, 0, 4 ).ok();

	assert_eq!( circuit.hertz(), Some( 64.0 ) );
}

#[wasm_bindgen_test]
fn it_lists_a_slider_with_its_voltage_and_undoes_a_set_value_and_its_amends_at_once()
{
	let mut circuit = Circuit::new();

	circuit.add( 19, 2.5, 0, -2, 0, 0 ).ok();

	assert_eq!( [ circuit.elements()[ 1 ], circuit.elements()[ 6 ] ], [ 19.0, 2.5 ] );

	circuit.set_value( 0, 3.0 ).ok();
	circuit.amend_value( 0, 4.0 ).ok();

	assert_eq!( circuit.elements()[ 6 ], 4.0 );
	assert!( circuit.amend_value( 0, 6.0 ).is_err() );
	assert_eq!( circuit.undo().ok(), Some( Some( 0 ) ) );
	assert_eq!( circuit.elements()[ 6 ], 2.5 );
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
	circuit.add( 0, 0.0, 0, 0, 4, 0 ).ok();
	circuit.add( 1, 200_000.0, 4, 0, 4, 3 ).ok();
	circuit.add( 1, 1000.0, 7, 0, 7, 2 ).ok();
	circuit.add( 8, 0.0, 4, 3, 6, 3 ).ok();
	circuit.add( 0, 0.0, 7, 4, 7, 6 ).ok();
	circuit.add( 0, 0.0, 0, 6, 7, 6 ).ok();
	circuit.add( 3, 0.0, 0, 6, 0, 6 ).ok();
	circuit.add( 0, 0.0, 4, 0, 7, 0 ).ok();

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
	circuit.add( 0, 0.0, 0, 0, 7, 0 ).ok();
	circuit.add( 2, 2.0, 4, 6, 4, 3 ).ok();
	circuit.add( 1, 1000.0, 7, 0, 7, 2 ).ok();
	circuit.add( 10, 0.0, 4, 3, 6, 3 ).ok();
	circuit.add( 0, 0.0, 7, 4, 7, 6 ).ok();
	circuit.add( 0, 0.0, 0, 6, 4, 6 ).ok();
	circuit.add( 3, 0.0, 0, 6, 0, 6 ).ok();
	circuit.add( 11, 0.0, 10, 3, 12, 3 ).ok();
	circuit.add( 0, 0.0, 4, 6, 7, 6 ).ok();

	let elements = circuit.elements();
	let transistor = &elements[ 52 .. 67 ];

	assert_eq!( transistor[ 1 ], 10.0 );
	assert_eq!( elements[ 107 ], 11.0 );
	assert!( ( transistor[ 13 ] - 0.0025 * 1.05 / 1.025 ).abs() < 1e-9 );
	assert!( transistor[ 12 ].abs() < 1e-9 );
}

#[wasm_bindgen_test]
fn it_opens_parts_in_place_of_the_drawing()
{
	let mut circuit = powered();

	open( &mut circuit, &[ 0.0, 1.0, 220.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 6.0, 1.0, 0.0, 2.0, 0.0, 2.0, 2.0 ], Vec::new(), &[], Vec::new(), &[], &[] ).ok();

	let elements = circuit.elements();

	assert_eq!( elements.len(), 26 );
	assert_eq!( &elements[ 1 .. 7 ], &[ 1.0, 0.0, 0.0, 2.0, 0.0, 220.0 ] );
	assert_eq!( &elements[ 14 .. 20 ], &[ 6.0, 2.0, 0.0, 2.0, 2.0, 1.0 ] );
}

#[wasm_bindgen_test]
fn it_rejects_parts_it_cannot_place()
{
	let mut circuit = powered();

	assert!( open( &mut circuit, &[ 0.0, 12.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0 ], Vec::new(), &[], Vec::new(), &[], &[] ).is_err() );
	assert!( open( &mut circuit, &[ 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0 ], Vec::new(), &[], Vec::new(), &[], &[] ).is_err() );
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
fn it_lists_an_led_with_its_color_and_its_forward_drop()
{
	let mut circuit = Circuit::new();

	circuit.add( 2, 5.0, 0, 2, 0, 0 ).ok();
	circuit.add( 17, 2.0, 0, 0, 2, 0 ).ok();
	circuit.add( 1, 300.0, 2, 0, 2, 2 ).ok();
	circuit.add( 0, 0.0, 2, 2, 0, 2 ).ok();

	let elements = circuit.elements();
	let drop = elements[ 22 ] - elements[ 23 ];

	assert_eq!( [ elements[ 14 ], elements[ 19 ] ], [ 17.0, 2.0 ] );
	assert!( drop > 2.9 && drop < 3.05 );
	assert_eq!( circuit.set_value( 1, 1.0 ).ok(), Some( 1 ) );
	assert_eq!( circuit.elements()[ 19 ], 1.0 );
	assert!( circuit.set_value( 1, 3.0 ).is_err() );
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

	open( &mut circuit, &[ 0.0, 10.0, 1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 1.0, 1000.0, 0.0, 4.0, 0.0, 6.0, 0.0 ], Vec::new(), &[], Vec::new(), &[], &[] ).ok();

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
	assert_eq!( circuit.outline( 1 ), vec![ -2, -2, 2, 2, 2, -2, 0, 2, 0 ] );
	assert!( circuit.outline( 2 ).is_empty() );
	assert_eq!( circuit.elements_at( 5, 1 ), vec![ symbol ] );
}

#[wasm_bindgen_test]
fn it_freezes_the_drawing_of_a_symbol_while_its_edited_layer_is_measured()
{
	let ( mut circuit, symbol ) = enclosed();

	circuit.enter( symbol ).ok();
	circuit.set_value( 1, 500.0 ).ok();

	assert!( ! circuit.frozen() );

	circuit.leave();

	assert!( circuit.frozen() );

	while circuit.proceed().ok().unwrap() {}

	assert!( ! circuit.frozen() );
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
	assert!( circuit.branches( -9, -9, 9, 9 ).is_empty() );
	assert!( circuit.branch_at( 3, 0 ).is_empty() );
	assert_eq!( circuit.elements_at( 4, 0 ), vec![ 1 ] );
	assert_eq!( circuit.set_value( 1, 500.0 ).ok(), Some( 1 ) );

	circuit.leave();

	assert_eq!( circuit.elements_at( 4, 0 ), vec![ 9 ] );
	assert_eq!( circuit.contents( 1 ).len(), 39 );
	assert!( circuit.contents( 1 )[ 24 ].is_nan() );
}

#[wasm_bindgen_test]
fn it_gives_the_layer_of_each_symbol_it_is_in()
{
	let ( mut circuit, symbol ) = enclosed();

	assert!( circuit.trail().is_empty() );

	circuit.enter( symbol ).ok();

	let inner = circuit.enclose( &[ 1, -2, 7, 2 ], String::new(), &[], &[] ).ok().unwrap();

	circuit.enter( inner ).ok();

	assert_eq!( circuit.trail(), vec![ 1, 2 ] );

	circuit.leave();

	assert_eq!( circuit.trail(), vec![ 1 ] );

	circuit.leave();

	assert!( circuit.trail().is_empty() );
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
	assert!( circuit.delay( 1 ).is_nan() );
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

	open( &mut circuit, &data, Vec::new(), &[], Vec::new(), &[], &[] ).ok();
	circuit.enter( 0 ).ok();
	open( &mut circuit, &data, Vec::new(), &[], Vec::new(), &[], &[] ).ok();

	assert_eq!( circuit.layer(), 0 );

	let elements = circuit.elements();

	assert_eq!( elements.len(), 17 );
	assert_eq!( &elements[ 1 .. 9 ], &[ 14.0, 5.0, 5.0, 5.0, 5.0, 1.0, 1.0, 4.0 ] );
	assert_eq!( circuit.symbols(), vec![ 1 ] );
}

#[wasm_bindgen_test]
fn it_gives_the_place_and_the_color_of_each_led_of_a_symbol_and_its_current()
{
	let mut circuit = Circuit::new();
	let data = [ 0.0, 14.0, 1.0, 0.0, 5.0, 5.0, 5.0, 5.0, 1.0, 13.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1000.0, 0.0, 0.0, 0.0, 4.0, 0.0, 1.0, 17.0, 1.0, 0.0, 4.0, 0.0, 4.0, 4.0, 1.0, 3.0, 0.0, 0.0, 4.0, 4.0, 4.0, 4.0 ];

	open( &mut circuit, &data, Vec::new(), &[ 1 ], vec![ "foo".to_string() ], &[ 0 ], &[ 2, -2, 0, 0, 2 ] ).ok();

	assert_eq!( circuit.outline( 1 ), vec![ -2, -2, 2, 2, 1, -2, 0, 0, 2, 1 ] );
	assert_eq!( circuit.lights().len(), 1 );
	assert!( circuit.lights()[ 0 ] > 1e-3 );
}

#[wasm_bindgen_test]
fn it_starts_a_default_circuit_on_an_empty_drawing()
{
	let circuit = Circuit::default();

	assert!( circuit.elements().is_empty() );
	assert_eq!( circuit.layer(), 0 );
}

#[wasm_bindgen_test]
fn it_encloses_a_named_symbol_with_its_drawing_and_its_layout()
{
	let mut circuit = laddered();

	assert!( circuit.enclose( &[ 1, -2, 7 ], String::new(), &[], &[] ).is_err() );
	assert!( circuit.enclose( &[ 1, -2, 7, 2 ], String::new(), &[ 3 ], &[] ).is_err() );
	assert!( circuit.enclose( &[ 1, -2, 7, 2 ], String::new(), &[ 2, 2, 0, 0 ], &[] ).is_err() );
	assert!( circuit.enclose( &[ 1, -2, 7, 2 ], String::new(), &[ 2, -1, 0, 0 ], &[] ).is_err() );
	assert!( circuit.enclose( &[ 1, -2, 7, 2 ], String::new(), &[ 1, 0, 0, 0 ], &[] ).is_err() );
	assert!( circuit.enclose( &[ 1, -2, 7, 2 ], String::new(), &[ 4, 0, 0, 2, 102 ], &[] ).is_err() );
	assert!( circuit.enclose( &[ 1, -2, 7, 2 ], String::new(), &[ 4, 0, 0, -1 ], &[] ).is_err() );
	assert!( circuit.enclose( &[ 1, -2, 7, 2 ], String::new(), &[ 4, 0, 0, 1, -1 ], &[] ).is_err() );
	assert!( circuit.enclose( &[ 1, -2, 7, 2 ], String::new(), &[ 4, 0, 0, 0 ], &[] ).is_err() );

	let drawing = [ 1, 0, 0, 2, 2, -1, 0, 1, 1, 3, -4, -4, 0, 0, -4, 4, 4, 0, 4, 3, 102, 111, 111 ];
	let symbol = circuit.enclose( &[ 1, -2, 7, 2 ], "foo".to_string(), &drawing, &[ -3, 0, 3, 0 ] ).ok().unwrap();

	assert_eq!( ( circuit.name( 1 ), circuit.drawing( 1 ), circuit.layout( 1 ) ), ( "foo".to_string(), drawing.to_vec(), vec![ -3, 0, 3, 0 ] ) );
	assert_eq!( ( circuit.name( 0 ), circuit.drawing( 9 ), circuit.layout( 9 ) ), ( String::new(), Vec::new(), Vec::new() ) );
	assert_eq!( circuit.elements_at( 1, 0 ), vec![ symbol, 9 ] );
}

#[wasm_bindgen_test]
fn it_loads_a_circuit_one_step_at_a_time_with_its_progress()
{
	let mut circuit = powered();
	let data = [
		0.0, 14.0, 1.0, 0.0, 5.0, 5.0, 5.0, 5.0,
		1.0, 13.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
		1.0, 12.0, 5.0, 0.0, 3.0, -1.0, 3.0, -1.0,
		1.0, 15.0, 0.0, 0.0, 6.0, 0.0, 5.0, 0.0,
		1.0, 0.0, 0.0, 0.0, 0.0, -2.0, 0.0, 0.0,
		1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0,
		1.0, 11.0, 0.0, 0.0, 0.0, -2.0, 2.0, -2.0,
		1.0, 10.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0,
		1.0, 0.0, 0.0, 0.0, 3.0, -3.0, 5.0, -3.0,
		1.0, 0.0, 0.0, 0.0, 5.0, -3.0, 5.0, 0.0,
		1.0, 0.0, 0.0, 0.0, 5.0, 0.0, 5.0, 1.0,
		1.0, 0.0, 0.0, 0.0, 5.0, 1.0, 3.0, 1.0,
		1.0, 3.0, 0.0, 0.0, 3.0, 3.0, 3.0, 3.0,
	];

	assert!( circuit.load( &data, Vec::new(), &[ 1 ], vec![ "foo".to_string() ], &[ 0 ], &[ 0 ] ).is_ok() );
	assert_eq!( ( circuit.progress(), circuit.elements().len() ), ( 0.0, 65 ) );

	while circuit.proceed().ok() == Some( true ) {}

	assert_eq!( ( circuit.progress(), circuit.elements().len() ), ( 1.0, 13 ) );
}

#[wasm_bindgen_test]
fn it_opens_the_names_the_drawings_the_layouts_and_the_labels_of_the_layers()
{
	let mut circuit = Circuit::new();
	let data = [ 0.0, 14.0, 1.0, 0.0, 5.0, 5.0, 5.0, 5.0, 1.0, 13.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 15.0, 0.0, 0.0, 5.0, 0.0, 4.0, 0.0 ];

	open( &mut circuit, &data, vec![ "qux".to_string(), "foo".to_string(), "bar".to_string() ], &[ 1 ], vec![ "baz".to_string() ], &[ 4, 1, 0, 0, 2 ], &[ 2, -3, 0, 3, 0 ] ).ok();

	let values : Vec<f64> = circuit.contents( 1 ).chunks( 13 ).map( | row | row[ 6 ] ).collect();

	assert_eq!( ( circuit.name( 1 ), circuit.drawing( 1 ), circuit.layout( 1 ), circuit.labels( 1 ) ), ( "baz".to_string(), vec![ 1, 0, 0, 2 ], vec![ -3, 0, 3, 0 ], vec![ "foo".to_string(), "bar".to_string() ] ) );
	assert_eq!( values, vec![ 1.0, 0.0 ] );
	assert_eq!( circuit.symbol_labels( 0 ), vec![ "qux".to_string() ] );
	assert!( open( &mut circuit, &data, Vec::new(), &[ 1 ], vec![ String::new() ], &[ 5, 1, 0, 0, 2 ], &[ 0 ] ).is_err() );
	assert!( open( &mut circuit, &data, Vec::new(), &[ 1 ], vec![ String::new() ], &[ 4, 1, 0, 0, 0 ], &[ 0 ] ).is_err() );
	assert!( open( &mut circuit, &data, Vec::new(), &[ 1 ], vec![ String::new() ], &[], &[ 0 ] ).is_err() );
	assert!( open( &mut circuit, &data, Vec::new(), &[ 1 ], vec![ String::new() ], &[ 0 ], &[ 3, -3, 0, 3, 0 ] ).is_err() );
	assert!( open( &mut circuit, &data, Vec::new(), &[ 1 ], vec![ String::new() ], &[ 0 ], &[] ).is_err() );
	assert_eq!( circuit.name( 1 ), "baz" );
}

#[wasm_bindgen_test]
fn it_sets_the_label_of_a_pin_and_undoes_it_and_the_label_of_a_symbol()
{
	let mut circuit = Circuit::new();
	let data = [ 0.0, 14.0, 1.0, 0.0, 5.0, 5.0, 5.0, 5.0, 1.0, 13.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 15.0, 0.0, 0.0, 5.0, 0.0, 4.0, 0.0 ];

	open( &mut circuit, &data, Vec::new(), &[ 1 ], vec![ String::new() ], &[ 4, 1, 0, 0, 2 ], &[ 2, -3, 0, 3, 0 ] ).ok();
	circuit.enter( 0 ).ok();

	assert_eq!( circuit.set_label( 1, "foo".to_string() ).ok(), Some( 1 ) );
	assert_eq!( circuit.labels( 1 ), vec![ String::new(), "foo".to_string() ] );
	assert!( circuit.set_label( 9, "bar".to_string() ).is_err() );

	circuit.undo().ok();

	assert_eq!( circuit.labels( 1 ), vec![ String::new(), String::new() ] );

	circuit.leave();

	assert_eq!( circuit.set_label( 0, "bar".to_string() ).ok(), Some( 0 ) );
	assert_eq!( circuit.symbol_labels( 0 ), vec![ "bar".to_string() ] );
}

#[wasm_bindgen_test]
fn it_captions_a_toggle_with_the_label_of_the_pin_on_its_net()
{
	let mut circuit = Circuit::new();

	circuit.add( 16, 0.0, 0, 0, 1, 0 ).ok();

	let output = circuit.add( 15, 0.0, 2, 0, 1, 0 ).unwrap();

	circuit.set_label( output, "foo".to_string() ).ok();

	assert_eq!( circuit.captions(), vec![ "foo".to_string() ] );
}

#[wasm_bindgen_test]
fn it_opens_sets_and_lists_the_level_of_a_free_input()
{
	let mut circuit = Circuit::new();
	let data = [ 0.0, 14.0, 1.0, 0.0, 5.0, 5.0, 5.0, 5.0, 1.0, 13.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1000.0, 0.0, 0.0, 0.0, 4.0, 0.0, 1.0, 15.0, 0.0, 0.0, 5.0, 0.0, 4.0, 0.0 ];

	open( &mut circuit, &data, Vec::new(), &[], Vec::new(), &[], &[] ).ok();
	circuit.enter( 0 ).ok();

	let reading = | circuit : &Circuit | ( circuit.elements()[ 6 ], circuit.elements()[ 9 ] );

	assert_eq!( reading( &circuit ), ( 1.0, 5.0 ) );
	assert_eq!( circuit.set_high( 0, false ).ok(), Some( 0 ) );
	assert_eq!( reading( &circuit ), ( 0.0, 0.0 ) );
	assert!( circuit.set_high( 2, true ).is_err() );
}

#[wasm_bindgen_test]
fn it_shows_each_blink_of_a_slow_led_and_the_steady_mean_of_an_led_that_blinks_faster_than_the_eye()
{
	let slow = blinking( 1.0 );
	let fast = blinking( 1e3 );
	let peak = slow.iter().copied().fold( 0.0, f64::max );

	assert!( peak > 1e-3 );
	assert!( slow.iter().copied().fold( f64::INFINITY, f64::min ) < 0.01 * peak );
	assert!( fast[ 1980 .. ].iter().all( | &current | ( current / peak - 0.5 ).abs() < 0.1 ) );
}

#[wasm_bindgen_test]
fn it_shows_the_steady_mean_of_an_led_inside_a_symbol_that_blinks_faster_than_the_eye()
{
	let slow = glowing( 1.0 );
	let fast = glowing( 1e3 );
	let peak = slow.iter().copied().fold( 0.0, f64::max );

	assert!( peak > 1e-3 );
	assert!( slow.iter().copied().fold( f64::INFINITY, f64::min ) < 0.01 * peak );
	assert!( fast[ 1980 .. ].iter().all( | &current | ( current / peak - 0.5 ).abs() < 0.1 ) );
}

#[wasm_bindgen_test]
fn it_shows_the_steady_mean_of_an_led_inside_a_gate_that_blinks_faster_than_the_eye()
{
	let ( mode, slow ) = inverting( 1.0 );
	let ( _, fast ) = inverting( 1e3 );
	let peak = slow.iter().copied().fold( 0.0, f64::max );

	assert_eq!( mode, "Digital" );
	assert!( peak > 1e-3 );
	assert!( slow.iter().copied().fold( f64::INFINITY, f64::min ) < 0.01 * peak );
	assert!( fast[ 198 .. ].iter().all( | &current | ( current / peak - 0.5 ).abs() < 0.1 ) );
}

#[wasm_bindgen_test]
fn it_shows_the_solution_of_an_edit_at_once_when_it_changes_the_parts()
{
	let mut circuit = powered();

	circuit.advance( 10 );
	circuit.add( 1, 1000.0, 2, 0, 2, 2 ).ok();

	let elements = circuit.elements();

	assert!( ( elements[ elements.len() - 2 ] - 0.0025 ).abs() < 1e-9 );

	circuit.advance( 1 );

	let elements = circuit.elements();

	assert!( ( elements[ elements.len() - 2 ] - 0.0025 ).abs() < 1e-9 );
}
