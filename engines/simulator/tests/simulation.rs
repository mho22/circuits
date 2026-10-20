use simulator::{ Error, Kind, Operation, Point, Segment, Simulation };


const STEP : f64 = 1e-6;


fn segment( from : ( i32, i32 ), to : ( i32, i32 ) ) -> Segment
{
	Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }
}

fn add( simulation : &mut Simulation, kind : Kind, from : ( i32, i32 ), to : ( i32, i32 ) ) -> u32
{
	simulation.apply( 0, Operation::Add { kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }, mirrored : false } ).unwrap()
}

fn series( kind : Kind ) -> Simulation
{
	let mut simulation = Simulation::new( STEP );

	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 0, 4 ), ( 0, 0 ) );
	add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) );
	add( &mut simulation, kind, ( 4, 0 ), ( 4, 4 ) );
	add( &mut simulation, Kind::Wire, ( 4, 4 ), ( 0, 4 ) );
	add( &mut simulation, Kind::Ground, ( 0, 4 ), ( 0, 4 ) );

	simulation
}

fn near( a : f64, b : f64, tolerance : f64 ) -> bool
{
	( a - b ).abs() < tolerance
}


#[test]
fn it_solves_the_circuit_after_each_operation()
{
	let simulation = series( Kind::Resistor { ohms : 1000.0 } );

	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 5 );
	assert!( near( simulation.solution().unwrap().readings[ 1 ].currents[ 0 ], 0.0025, 1e-12 ) );
}

#[test]
fn it_charges_a_capacitor_along_the_rc_curve()
{
	let mut simulation = series( Kind::Capacitor { farads : 1e-6 } );

	for milliseconds in 1 ..= 3
	{
		simulation.advance( 1000 );

		let expected = 5.0 * ( 1.0 - ( - milliseconds as f64 ).exp() );

		assert!( near( simulation.solution().unwrap().readings[ 2 ].potentials[ 0 ], expected, 2e-3 ) );
	}
}

#[test]
fn it_raises_the_current_of_an_inductor_along_the_rl_curve()
{
	let mut simulation = series( Kind::Inductor { henries : 1.0 } );

	for milliseconds in 1 ..= 3
	{
		simulation.advance( 1000 );

		let expected = 0.005 * ( 1.0 - ( - milliseconds as f64 ).exp() );

		assert!( near( simulation.solution().unwrap().readings[ 2 ].currents[ 0 ], expected, 2e-6 ) );
	}
}

#[test]
fn it_keeps_the_charge_while_the_circuit_has_no_solution()
{
	let mut simulation = series( Kind::Capacitor { farads : 1e-6 } );

	simulation.advance( 1000 );

	let short = add( &mut simulation, Kind::Switch { closed : true }, ( 0, 4 ), ( 0, 0 ) );

	simulation.advance( 1000 );

	assert_eq!( simulation.solution(), Err( Error::Singular ) );

	simulation.apply( 0, Operation::Delete { id : short } ).unwrap();

	assert!( near( simulation.solution().unwrap().readings[ 2 ].potentials[ 0 ], 5.0 * ( 1.0 - ( -1.0_f64 ).exp() ), 1e-2 ) );
}

#[test]
fn it_opens_and_closes_a_switch()
{
	let mut simulation = series( Kind::Switch { closed : false } );

	assert_eq!( simulation.solution().unwrap().readings[ 1 ].currents[ 0 ], 0.0 );

	simulation.apply( 0, Operation::SetClosed { id : 2, closed : true } ).unwrap();

	assert!( near( simulation.solution().unwrap().readings[ 1 ].currents[ 0 ], 0.005, 1e-12 ) );
}

#[test]
fn it_rejects_an_invalid_operation()
{
	let mut simulation = Simulation::new( STEP );

	assert_eq!( simulation.apply( 0, Operation::Delete { id : 7 } ), Err( Error::Missing ) );
	assert!( simulation.solution().unwrap().readings.is_empty() );
}

#[test]
fn it_undoes_a_deletion()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );
	let before = simulation.document( 0 ).unwrap().elements().to_vec();

	simulation.apply( 0, Operation::Delete { id : 3 } ).unwrap();

	assert_eq!( simulation.undo(), Ok( Some( 3 ) ) );
	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );
	assert!( near( simulation.solution().unwrap().readings[ 1 ].currents[ 0 ], 0.0025, 1e-12 ) );
}

#[test]
fn it_undoes_each_operation_from_the_last_one()
{
	let mut simulation = series( Kind::Switch { closed : false } );

	simulation.apply( 0, Operation::SetValue { id : 1, value : 2200.0 } ).unwrap();
	simulation.apply( 0, Operation::SetClosed { id : 2, closed : true } ).unwrap();

	assert_eq!( simulation.undo(), Ok( Some( 2 ) ) );
	assert_eq!( simulation.document( 0 ).unwrap().elements()[ 2 ].kind, Kind::Switch { closed : false } );
	assert_eq!( simulation.undo(), Ok( Some( 1 ) ) );
	assert_eq!( simulation.document( 0 ).unwrap().elements()[ 1 ].kind, Kind::Resistor { ohms : 1000.0 } );

	for _ in 0 .. 5 { simulation.undo().unwrap(); }

	assert!( simulation.document( 0 ).unwrap().elements().is_empty() );
	assert_eq!( simulation.undo(), Ok( None ) );
}

#[test]
fn it_redoes_what_it_undid()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );
	let after = simulation.document( 0 ).unwrap().elements().to_vec();

	simulation.undo().unwrap();
	simulation.undo().unwrap();

	assert_eq!( simulation.redo(), Ok( Some( 3 ) ) );
	assert_eq!( simulation.redo(), Ok( Some( 4 ) ) );
	assert_eq!( simulation.document( 0 ).unwrap().elements(), after.as_slice() );
	assert_eq!( simulation.redo(), Ok( None ) );
}

#[test]
fn it_forgets_the_redo_after_a_new_operation()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );

	simulation.undo().unwrap();
	add( &mut simulation, Kind::Wire, ( 9, 9 ), ( 9, 8 ) );

	assert_eq!( simulation.redo(), Ok( None ) );
}

#[test]
fn it_keeps_the_undo_untouched_by_an_invalid_operation()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );

	assert!( simulation.apply( 0, Operation::Delete { id : 7 } ).is_err() );
	assert_eq!( simulation.undo(), Ok( Some( 4 ) ) );
}

#[test]
fn it_places_a_part_on_a_wire_and_carries_the_current_through_it()
{
	let mut simulation = series( Kind::Wire );

	let resistor = simulation.place( 0, Kind::Resistor { ohms : 1000.0 }, Segment { from : Point { x : 4, y : 1 }, to : Point { x : 4, y : 3 } } ).unwrap();
	let solution = simulation.solution().unwrap();
	let index = simulation.document( 0 ).unwrap().elements().iter().position( | element | element.id == resistor ).unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 7 );
	assert!( near( solution.readings[ index ].currents[ 0 ], 0.0025, 1e-12 ) );
}

#[test]
fn it_undoes_a_split_in_one_step()
{
	let mut simulation = series( Kind::Wire );
	let before = simulation.document( 0 ).unwrap().elements().to_vec();

	simulation.place( 0, Kind::Resistor { ohms : 1000.0 }, Segment { from : Point { x : 4, y : 1 }, to : Point { x : 4, y : 3 } } ).unwrap();
	simulation.undo().unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );

	simulation.redo().unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 7 );
}

#[test]
fn it_leaves_the_wire_whole_when_the_part_is_invalid()
{
	let mut simulation = series( Kind::Wire );
	let before = simulation.document( 0 ).unwrap().elements().to_vec();

	assert_eq!( simulation.place( 0, Kind::Resistor { ohms : 0.0 }, Segment { from : Point { x : 4, y : 1 }, to : Point { x : 4, y : 3 } } ), Err( Error::Resistance ) );
	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );
	assert_eq!( simulation.undo(), Ok( Some( 4 ) ) );
}

#[test]
fn it_keeps_the_drop_of_a_diode_while_time_runs()
{
	let mut simulation = series( Kind::Diode );
	let drop = | simulation : &Simulation | { let reading = &simulation.solution().unwrap().readings[ 2 ]; reading.potentials[ 0 ] - reading.potentials[ 1 ] };
	let before = drop( &simulation );

	simulation.advance( 100 );

	assert!( before > 0.6 && before < 0.7 );
	assert!( near( drop( &simulation ), before, 1e-6 ) );
}

#[test]
fn it_opens_a_list_of_parts_in_place_of_the_drawing()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );
	let parts = [ ( Kind::Wire, segment( ( 0, 0 ), ( 2, 0 ) ), false ), ( Kind::Npn, segment( ( 2, 0 ), ( 4, 0 ) ), true ) ];

	simulation.open( &parts ).unwrap();

	let opened : Vec<( Kind, Segment, bool )> = simulation.document( 0 ).unwrap().elements().iter().map( | element | ( element.kind, element.segment, element.mirrored ) ).collect();

	assert_eq!( opened, parts );
}

#[test]
fn it_brings_the_drawing_back_with_one_undo_after_an_open()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );
	let before = simulation.document( 0 ).unwrap().elements().to_vec();

	simulation.open( &[] ).unwrap();

	assert!( simulation.document( 0 ).unwrap().elements().is_empty() );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );
}

#[test]
fn it_keeps_the_drawing_when_a_part_is_invalid()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );
	let before = simulation.document( 0 ).unwrap().elements().to_vec();

	assert_eq!( simulation.open( &[ ( Kind::Wire, segment( ( 0, 0 ), ( 2, 0 ) ), false ), ( Kind::Resistor { ohms : 0.0 }, segment( ( 2, 0 ), ( 4, 0 ) ), false ) ] ), Err( Error::Resistance ) );
	assert_eq!( simulation.open( &[ ( Kind::Wire, segment( ( 0, 0 ), ( 2, 1 ) ), false ) ] ), Err( Error::Crooked ) );
	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );
	assert_eq!( simulation.undo(), Ok( Some( 4 ) ) );
}

#[test]
fn it_moves_a_part_with_its_wires_in_one_undo_step()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );
	let before = simulation.document( 0 ).unwrap().elements().to_vec();

	simulation.transform( 0, 2, Kind::Resistor { ohms : 1000.0 }, segment( ( 8, 0 ), ( 8, 4 ) ), false ).unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements()[ 2 ].segment, segment( ( 8, 0 ), ( 8, 4 ) ) );
	assert!( near( simulation.solution().unwrap().readings[ 2 ].currents[ 0 ], 0.0025, 1e-12 ) );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );
}

#[test]
fn it_splits_a_wire_under_a_moved_part()
{
	let mut simulation = series( Kind::Wire );
	let resistor = add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 9, 0 ), ( 9, 2 ) );

	simulation.transform( 0, resistor, Kind::Resistor { ohms : 1000.0 }, segment( ( 4, 1 ), ( 4, 3 ) ), false ).unwrap();

	let index = simulation.document( 0 ).unwrap().elements().iter().position( | element | element.id == resistor ).unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 7 );
	assert!( near( simulation.solution().unwrap().readings[ index ].currents[ 0 ], 0.0025, 1e-12 ) );
}

#[test]
fn it_rejects_a_move_of_a_missing_part()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );

	assert_eq!( simulation.transform( 0, 7, Kind::Resistor { ohms : 1000.0 }, segment( ( 0, 0 ), ( 2, 0 ) ), false ), Err( Error::Missing ) );
	assert_eq!( simulation.undo(), Ok( Some( 4 ) ) );
}

#[test]
fn it_slides_a_part_along_its_wire()
{
	let mut simulation = Simulation::new( STEP );

	add( &mut simulation, Kind::Wire, ( 0, 0 ), ( 8, 0 ) );

	let resistor = simulation.place( 0, Kind::Resistor { ohms : 1000.0 }, segment( ( 2, 0 ), ( 4, 0 ) ) ).unwrap();

	simulation.transform( 0, resistor, Kind::Resistor { ohms : 1000.0 }, segment( ( 5, 0 ), ( 7, 0 ) ), false ).unwrap();

	let wires : Vec<Segment> = simulation.document( 0 ).unwrap().elements().iter().filter( | element | element.kind == Kind::Wire ).map( | element | element.segment ).collect();

	assert_eq!( wires, vec![ segment( ( 0, 0 ), ( 5, 0 ) ), segment( ( 7, 0 ), ( 8, 0 ) ) ] );
}

fn define( simulation : &mut Simulation, parts : &[ ( Kind, ( i32, i32 ), ( i32, i32 ) ) ] ) -> u32
{
	let layer = simulation.create();

	for &( kind, from, to ) in parts { simulation.apply( layer, Operation::Add { kind, segment : segment( from, to ), mirrored : false } ).unwrap(); }

	layer as u32
}

fn resistor( simulation : &mut Simulation, ohms : f64 ) -> u32
{
	define( simulation, &[ ( Kind::Pin, ( 0, 0 ), ( 0, 0 ) ), ( Kind::Resistor { ohms }, ( 0, 0 ), ( 4, 0 ) ), ( Kind::Pin, ( 4, 0 ), ( 4, 0 ) ) ] )
}

#[test]
fn it_solves_a_symbol_with_the_circuit_of_its_layer()
{
	let mut simulation = Simulation::new( STEP );
	let layer = resistor( &mut simulation, 1000.0 );

	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 0, 4 ), ( 0, 0 ) );
	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 2, 0 ), ( 2, 0 ) );
	add( &mut simulation, Kind::Wire, ( 4, 0 ), ( 4, 4 ) );
	add( &mut simulation, Kind::Wire, ( 4, 4 ), ( 0, 4 ) );
	add( &mut simulation, Kind::Ground, ( 0, 4 ), ( 0, 4 ) );

	let solution = simulation.solution().unwrap();
	let symbol = &solution.readings[ 1 ];

	assert_eq!( solution.readings.len(), 5 );
	assert!( near( symbol.potentials[ 0 ], 5.0, 1e-9 ) && near( symbol.potentials[ 1 ], 0.0, 1e-9 ) );
	assert!( near( symbol.currents[ 0 ], 0.005, 1e-12 ) && near( symbol.currents[ 1 ], -0.005, 1e-12 ) );
	assert!( near( solution.readings[ 0 ].currents[ 1 ], -0.005, 1e-12 ) );
}

#[test]
fn it_changes_every_instance_with_its_layer()
{
	let mut simulation = Simulation::new( STEP );
	let layer = resistor( &mut simulation, 1000.0 );

	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 0, 4 ), ( 0, 0 ) );
	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 2, 0 ), ( 2, 0 ) );
	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 6, 0 ), ( 6, 0 ) );
	add( &mut simulation, Kind::Wire, ( 8, 0 ), ( 8, 4 ) );
	add( &mut simulation, Kind::Wire, ( 8, 4 ), ( 0, 4 ) );
	add( &mut simulation, Kind::Ground, ( 0, 4 ), ( 0, 4 ) );

	assert!( near( simulation.solution().unwrap().readings[ 1 ].currents[ 0 ], 0.0025, 1e-12 ) );

	simulation.apply( layer as usize, Operation::SetValue { id : 1, value : 500.0 } ).unwrap();

	assert!( near( simulation.solution().unwrap().readings[ 1 ].currents[ 0 ], 0.005, 1e-12 ) );
	assert!( near( simulation.solution().unwrap().readings[ 2 ].potentials[ 0 ], 2.5, 1e-9 ) );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( layer as usize ).unwrap().elements()[ 1 ].kind, Kind::Resistor { ohms : 1000.0 } );
	assert!( near( simulation.solution().unwrap().readings[ 1 ].currents[ 0 ], 0.0025, 1e-12 ) );
}

#[test]
fn it_reads_each_pin_of_a_symbol_in_the_order_of_its_layer()
{
	let mut simulation = Simulation::new( STEP );
	let layer = define( &mut simulation, &[
		( Kind::Pin, ( 0, 0 ), ( 0, 0 ) ),
		( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) ),
		( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 8, 0 ) ),
		( Kind::Pin, ( 8, 0 ), ( 8, 0 ) ),
		( Kind::Pin, ( 4, 0 ), ( 4, 0 ) ),
	] );

	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 8, 14 ), ( 8, 10 ) );
	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) );
	add( &mut simulation, Kind::Wire, ( 12, 10 ), ( 12, 14 ) );
	add( &mut simulation, Kind::Wire, ( 12, 14 ), ( 8, 14 ) );
	add( &mut simulation, Kind::Ground, ( 8, 14 ), ( 8, 14 ) );

	let symbol = &simulation.solution().unwrap().readings[ 1 ];

	assert!( near( symbol.potentials[ 0 ], 5.0, 1e-9 ) && near( symbol.potentials[ 1 ], 0.0, 1e-9 ) && near( symbol.potentials[ 2 ], 2.5, 1e-9 ) );
	assert!( near( symbol.currents[ 2 ], 0.0, 1e-12 ) );
}

#[test]
fn it_inverts_a_level_with_a_cmos_symbol_fed_by_its_vcc_pin()
{
	let mut simulation = Simulation::new( STEP );
	let layer = define( &mut simulation, &[
		( Kind::Pin, ( 0, 0 ), ( 0, 0 ) ),
		( Kind::Pin, ( 3, -1 ), ( 3, -1 ) ),
		( Kind::Pin, ( 5, 0 ), ( 5, 0 ) ),
		( Kind::Wire, ( 0, -2 ), ( 0, 2 ) ),
		( Kind::Pmos, ( 0, -2 ), ( 2, -2 ) ),
		( Kind::Nmos, ( 0, 2 ), ( 2, 2 ) ),
		( Kind::Wire, ( 3, -3 ), ( 5, -3 ) ),
		( Kind::Wire, ( 5, -3 ), ( 5, 1 ) ),
		( Kind::Wire, ( 5, 1 ), ( 3, 1 ) ),
		( Kind::Ground, ( 3, 3 ), ( 3, 3 ) ),
	] );

	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) );
	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 6, 8 ), ( 6, 6 ) );
	add( &mut simulation, Kind::Wire, ( 6, 6 ), ( 10, 6 ) );
	add( &mut simulation, Kind::Wire, ( 10, 6 ), ( 10, 8 ) );
	add( &mut simulation, Kind::Ground, ( 6, 8 ), ( 6, 8 ) );

	let input = add( &mut simulation, Kind::Battery { volts : 0.0 }, ( 4, 12 ), ( 4, 10 ) );

	add( &mut simulation, Kind::Wire, ( 4, 10 ), ( 8, 10 ) );
	add( &mut simulation, Kind::Ground, ( 4, 12 ), ( 4, 12 ) );

	let output = | simulation : &Simulation | simulation.solution().unwrap().readings[ 0 ].potentials[ 2 ];

	assert!( near( output( &simulation ), 5.0, 1e-3 ) );

	simulation.apply( 0, Operation::SetValue { id : input, value : 5.0 } ).unwrap();

	assert!( near( output( &simulation ), 0.0, 1e-3 ) );
	assert!( simulation.solution().unwrap().branches.iter().all( | branch | branch.segment.from.x.abs() < 100 ) );
}

#[test]
fn it_keeps_the_charge_inside_a_symbol_after_a_change_elsewhere()
{
	let mut simulation = Simulation::new( STEP );
	let layer = define( &mut simulation, &[ ( Kind::Pin, ( 0, 0 ), ( 0, 0 ) ), ( Kind::Capacitor { farads : 1e-6 }, ( 0, 0 ), ( 4, 0 ) ), ( Kind::Pin, ( 4, 0 ), ( 4, 0 ) ) ] );

	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 0, 4 ), ( 0, 0 ) );
	add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) );
	add( &mut simulation, Kind::Symbol { layer, rotation : 1 }, ( 4, 2 ), ( 4, 2 ) );
	add( &mut simulation, Kind::Wire, ( 4, 4 ), ( 0, 4 ) );
	add( &mut simulation, Kind::Ground, ( 0, 4 ), ( 0, 4 ) );

	simulation.advance( 1000 );
	simulation.apply( 0, Operation::Insert { element : simulator::Element { id : 9, kind : Kind::Wire, segment : segment( ( 9, 9 ), ( 9, 8 ) ), mirrored : false }, index : 0 } ).unwrap();

	assert!( near( simulation.solution().unwrap().readings[ 3 ].potentials[ 0 ], 5.0 * ( 1.0 - ( -1.0_f64 ).exp() ), 2e-3 ) );
}

#[test]
fn it_rejects_a_symbol_that_holds_itself_and_keeps_its_layer()
{
	let mut simulation = Simulation::new( STEP );
	let layer = resistor( &mut simulation, 1000.0 );

	assert_eq!( simulation.apply( layer as usize, Operation::Add { kind : Kind::Symbol { layer, rotation : 0 }, segment : segment( ( 9, 9 ), ( 9, 9 ) ), mirrored : false } ), Err( Error::Recursive ) );
	assert_eq!( simulation.document( layer as usize ).unwrap().elements().len(), 3 );
	assert_eq!( simulation.undo(), Ok( Some( 2 ) ) );
}

#[test]
fn it_rejects_an_edit_of_a_missing_layer()
{
	let mut simulation = Simulation::new( STEP );

	assert_eq!( simulation.apply( 1, Operation::Add { kind : Kind::Wire, segment : segment( ( 0, 0 ), ( 2, 0 ) ), mirrored : false } ), Err( Error::Layerless ) );
	assert_eq!( simulation.place( 1, Kind::Wire, segment( ( 0, 0 ), ( 2, 0 ) ) ), Err( Error::Layerless ) );
	assert_eq!( simulation.apply( 0, Operation::Add { kind : Kind::Symbol { layer : 1, rotation : 0 }, segment : segment( ( 0, 0 ), ( 0, 0 ) ), mirrored : false } ), Err( Error::Layerless ) );
	assert!( simulation.document( 0 ).unwrap().elements().is_empty() );
	assert!( simulation.document( 1 ).is_none() );
}

#[test]
fn it_reads_the_voltage_at_a_pin_of_the_drawing_without_a_current()
{
	let mut simulation = series( Kind::Wire );

	add( &mut simulation, Kind::Pin, ( 4, 2 ), ( 4, 2 ) );

	let solution = simulation.solution().unwrap();

	assert!( near( solution.readings[ 5 ].potentials[ 0 ], 0.0, 1e-9 ) );
	assert!( solution.readings[ 5 ].currents[ 0 ].is_nan() );
	assert!( solution.branches.iter().all( | branch | near( branch.current.abs(), 0.005, 1e-12 ) ) );
}

#[test]
fn it_turns_a_symbol_and_its_wires_follow_its_pins()
{
	let mut simulation = Simulation::new( STEP );
	let layer = resistor( &mut simulation, 1000.0 );

	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 0, 4 ), ( 0, 0 ) );

	let symbol = add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 2, 0 ), ( 2, 0 ) );

	add( &mut simulation, Kind::Wire, ( 4, 0 ), ( 4, 4 ) );
	add( &mut simulation, Kind::Wire, ( 4, 4 ), ( 0, 4 ) );
	add( &mut simulation, Kind::Ground, ( 0, 4 ), ( 0, 4 ) );

	let before = simulation.document( 0 ).unwrap().elements().to_vec();

	simulation.transform( 0, symbol, Kind::Symbol { layer, rotation : 1 }, segment( ( 2, 0 ), ( 2, 0 ) ), false ).unwrap();

	let turned = *simulation.document( 0 ).unwrap().elements().iter().find( | element | element.id == symbol ).unwrap();

	assert_eq!( simulation.terminals( &turned ), vec![ Point { x : 2, y : -2 }, Point { x : 2, y : 2 } ] );
	assert!( near( simulation.solution().unwrap().readings[ 1 ].currents[ 0 ], 0.005, 1e-12 ) );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );
}

#[test]
fn it_rejects_a_move_on_a_missing_layer()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );

	assert_eq!( simulation.transform( 1, 0, Kind::Resistor { ohms : 1000.0 }, segment( ( 0, 0 ), ( 2, 0 ) ), false ), Err( Error::Layerless ) );
}
