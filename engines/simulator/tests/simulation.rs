use simulator::{ Element, Error, Kind, Mode, Operation, Outline, Point, Segment, Shape, Simulation, Solution, DELAY };


const STEP : f64 = 1e-6;


type Part = ( Kind, ( i32, i32 ), ( i32, i32 ) );


fn segment( from : ( i32, i32 ), to : ( i32, i32 ) ) -> Segment
{
	Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }
}

fn point( x : i32, y : i32 ) -> Point
{
	Point { x, y }
}

fn add( simulation : &mut Simulation, kind : Kind, from : ( i32, i32 ), to : ( i32, i32 ) ) -> u32
{
	simulation.apply( 0, Operation::Add { kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }, mirrored : false } ).unwrap()
}

fn added( parts : &[ ( usize, Kind, Segment, bool ) ] ) -> Vec<( usize, Operation )>
{
	parts.iter().map( | &( layer, kind, segment, mirrored ) | ( layer, Operation::Add { kind, segment, mirrored } ) ).collect()
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

fn measure( simulation : &mut Simulation )
{
	while simulation.proceed().unwrap() {}
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
fn it_pastes_parts_shifted_with_their_mirror_and_their_label_and_cuts_the_wire_under_them_in_one_step()
{
	let mut simulation = Simulation::new( STEP );

	add( &mut simulation, Kind::Wire, ( 10, 0 ), ( 10, 8 ) );

	let before = simulation.document( 0 ).unwrap().elements().to_vec();
	let element = | id, kind, from, to, mirrored | Element { id, kind, segment : segment( from, to ), mirrored };
	let parts = [
		( element( 7, Kind::Nmos, ( 0, 0 ), ( 2, 0 ), true ), String::new() ),
		( element( 8, Kind::Input { high : false }, ( 0, 4 ), ( 1, 4 ), false ), "foo".to_string() ),
		( element( 9, Kind::Resistor { ohms : 1000.0 }, ( 0, 2 ), ( 0, 6 ), false ), String::new() ),
	];

	simulation.paste( 0, &parts, 10, 0 ).unwrap();

	let document = simulation.document( 0 ).unwrap();
	let kinds : Vec<( Kind, Segment, bool )> = document.elements().iter().map( | element | ( element.kind, element.segment, element.mirrored ) ).collect();
	let input = document.elements().iter().find( | element | element.kind.pin() ).unwrap();

	assert!( kinds.contains( &( Kind::Nmos, segment( ( 10, 0 ), ( 12, 0 ) ), true ) ) );
	assert!( kinds.contains( &( Kind::Resistor { ohms : 1000.0 }, segment( ( 10, 2 ), ( 10, 6 ) ), false ) ) );
	assert!( kinds.contains( &( Kind::Wire, segment( ( 10, 0 ), ( 10, 2 ) ), false ) ) && kinds.contains( &( Kind::Wire, segment( ( 10, 6 ), ( 10, 8 ) ), false ) ) );
	assert_eq!( ( input.segment, document.label( input.id ) ), ( segment( ( 10, 4 ), ( 11, 4 ) ), "foo" ) );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );
	assert_eq!( simulation.paste( 0, &[], 0, 0 ), Err( Error::Unselected ) );
}

#[test]
fn it_lists_a_toggle_that_reads_its_wire_and_shows_the_level_of_the_wire()
{
	let mut simulation = Simulation::new( STEP );
	let toggle = add( &mut simulation, Kind::Toggle { high : false }, ( 1, 0 ), ( 2, 0 ) );

	add( &mut simulation, Kind::Wire, ( 2, 0 ), ( 6, 0 ) );
	add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 6, 0 ), ( 6, 4 ) );
	add( &mut simulation, Kind::Ground, ( 6, 4 ), ( 6, 4 ) );

	assert!( simulation.listening().is_empty() );

	let supply = add( &mut simulation, Kind::Supply { volts : 5.0 }, ( 6, 0 ), ( 6, 0 ) );

	assert_eq!( simulation.listening(), vec![ toggle ] );
	assert!( near( simulation.solution().unwrap().readings[ toggle as usize ].potentials[ 1 ], 5.0, 1e-6 ) );

	simulation.apply( 0, Operation::Delete { id : supply } ).unwrap();

	assert!( simulation.listening().is_empty() );
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
	let parts = [ ( 0, Kind::Wire, segment( ( 0, 0 ), ( 2, 0 ) ), false ), ( 0, Kind::Npn, segment( ( 2, 0 ), ( 4, 0 ) ), true ) ];

	simulation.open( added( &parts ) ).unwrap();

	let opened : Vec<( usize, Kind, Segment, bool )> = simulation.document( 0 ).unwrap().elements().iter().map( | element | ( 0, element.kind, element.segment, element.mirrored ) ).collect();

	assert_eq!( opened, parts );
}

#[test]
fn it_brings_the_drawing_back_with_one_undo_after_an_open()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );
	let before = simulation.document( 0 ).unwrap().elements().to_vec();

	simulation.open( Vec::new() ).unwrap();

	assert!( simulation.document( 0 ).unwrap().elements().is_empty() );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );
}

#[test]
fn it_opens_the_layers_of_the_symbols_in_place_of_every_layer()
{
	let mut simulation = Simulation::new( STEP );
	let layer = resistor( &mut simulation, 1000.0 );

	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 2, 0 ), ( 2, 0 ) );

	let parts = [
		( 0, Kind::Symbol { layer : 2, rotation : 1 }, segment( ( 5, 5 ), ( 5, 5 ) ), false ),
		( 2, Kind::Input { high : false }, segment( ( -1, 0 ), ( 0, 0 ) ), false ),
		( 2, Kind::Wire, segment( ( 0, 0 ), ( 2, 0 ) ), false ),
		( 2, Kind::Output, segment( ( 3, 0 ), ( 2, 0 ) ), false ),
	];

	simulation.open( added( &parts ) ).unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 1 );
	assert!( simulation.document( 1 ).unwrap().elements().is_empty() );
	assert_eq!( simulation.document( 2 ).unwrap().elements().len(), 3 );
	assert_eq!( simulation.elements_at( 0, Point { x : 5, y : 4 } ), vec![ 1 ] );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( 1 ).unwrap().elements().len(), 3 );
	assert!( simulation.document( 2 ).unwrap().elements().is_empty() );
}

fn gate() -> Vec<( usize, Operation )>
{
	let parts = [
		( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		( Kind::Supply { volts : 5.0 }, ( 3, -1 ), ( 3, -1 ) ),
		( Kind::Output, ( 6, 0 ), ( 5, 0 ) ),
		( Kind::Wire, ( 0, -2 ), ( 0, 0 ) ),
		( Kind::Wire, ( 0, 0 ), ( 0, 2 ) ),
		( Kind::Pmos, ( 0, -2 ), ( 2, -2 ) ),
		( Kind::Nmos, ( 0, 2 ), ( 2, 2 ) ),
		( Kind::Wire, ( 3, -3 ), ( 5, -3 ) ),
		( Kind::Wire, ( 5, -3 ), ( 5, 0 ) ),
		( Kind::Wire, ( 5, 0 ), ( 5, 1 ) ),
		( Kind::Wire, ( 5, 1 ), ( 3, 1 ) ),
		( Kind::Ground, ( 3, 3 ), ( 3, 3 ) ),
	].map( | ( kind, from, to ) | ( 1, kind, segment( from, to ), false ) );

	[ added( &[ ( 0, Kind::Symbol { layer : 1, rotation : 0 }, segment( ( 10, 10 ), ( 10, 10 ) ), false ) ] ), added( &parts ), vec![ ( 1, Operation::Describe { name : "foo".to_string(), drawing : Vec::new(), layout : Vec::new() } ) ] ].concat()
}

#[test]
fn it_loads_a_list_of_parts_one_step_at_a_time_and_opens_it_at_the_last_step()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );

	simulation.load( gate() ).unwrap();

	assert_eq!( simulation.progress(), 0.0 );
	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 5 );

	let mut steps = vec![ simulation.progress() ];

	while simulation.proceed().unwrap() { steps.push( simulation.progress() ); }

	assert!( steps.windows( 2 ).all( | pair | pair[ 0 ] <= pair[ 1 ] ) );
	assert_eq!( simulation.progress(), 1.0 );
	assert_eq!( simulation.proceed(), Ok( false ) );
	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 1 );
	assert_eq!( simulation.document( 1 ).unwrap().name(), "foo" );
	assert_eq!( simulation.delay( 1 ), Some( DELAY ) );
}

#[test]
fn it_rejects_a_load_with_an_invalid_part_and_keeps_the_drawing()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );

	simulation.load( gate() ).unwrap();

	assert_eq!( simulation.load( added( &[ ( 0, Kind::Wire, segment( ( 0, 0 ), ( 2, 1 ) ), false ) ] ) ), Err( Error::Crooked ) );
	assert_eq!( ( simulation.progress(), simulation.proceed() ), ( 1.0, Ok( false ) ) );
	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 5 );
}

#[test]
fn it_reports_an_open_that_fails_at_the_last_step_of_a_load()
{
	let mut simulation = Simulation::new( STEP );

	simulation.load( added( &[ ( 0, Kind::Symbol { layer : 3, rotation : 0 }, segment( ( 0, 0 ), ( 0, 0 ) ), false ), ( 1, Kind::Wire, segment( ( 0, 0 ), ( 2, 0 ) ), false ) ] ) ).unwrap();

	let mut result = simulation.proceed();

	while result == Ok( true ) { result = simulation.proceed(); }

	assert_eq!( ( result, simulation.progress() ), ( Err( Error::Layerless ), 1.0 ) );
	assert!( simulation.document( 1 ).is_none() );
}

#[test]
fn it_adds_no_layer_when_an_open_fails()
{
	let mut simulation = Simulation::new( STEP );

	assert_eq!( simulation.open( added( &[ ( 0, Kind::Symbol { layer : 3, rotation : 0 }, segment( ( 0, 0 ), ( 0, 0 ) ), false ), ( 1, Kind::Wire, segment( ( 0, 0 ), ( 2, 0 ) ), false ) ] ) ), Err( Error::Layerless ) );
	assert!( simulation.document( 1 ).is_none() );
}

#[test]
fn it_keeps_the_drawing_when_a_part_is_invalid()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );
	let before = simulation.document( 0 ).unwrap().elements().to_vec();

	assert_eq!( simulation.open( added( &[ ( 0, Kind::Wire, segment( ( 0, 0 ), ( 2, 0 ) ), false ), ( 0, Kind::Resistor { ohms : 0.0 }, segment( ( 2, 0 ), ( 4, 0 ) ), false ) ] ) ), Err( Error::Resistance ) );
	assert_eq!( simulation.open( added( &[ ( 0, Kind::Wire, segment( ( 0, 0 ), ( 2, 1 ) ), false ) ] ) ), Err( Error::Crooked ) );
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
fn it_shifts_a_selection_with_its_wires_in_one_undo_step()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );
	let before = simulation.document( 0 ).unwrap().elements().to_vec();

	simulation.shift( 0, point( 4, 0 ), point( 4, 4 ), 4, 0 ).unwrap();

	let elements = simulation.document( 0 ).unwrap().elements();
	let index = elements.iter().position( | element | element.id == 2 ).unwrap();

	assert_eq!( elements[ index ].segment, segment( ( 8, 0 ), ( 8, 4 ) ) );
	assert!( near( simulation.solution().unwrap().readings[ index ].currents[ 0 ], 0.0025, 1e-12 ) );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );
}

#[test]
fn it_shifts_a_whole_circuit_without_a_new_wire()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );

	simulation.shift( 0, point( 0, 0 ), point( 4, 4 ), 10, -2 ).unwrap();

	let elements = simulation.document( 0 ).unwrap().elements();

	assert_eq!( elements.len(), 5 );
	assert_eq!( elements[ 0 ].segment, segment( ( 10, 2 ), ( 10, -2 ) ) );
	assert!( near( simulation.solution().unwrap().readings[ 2 ].currents[ 0 ], 0.0025, 1e-12 ) );
}

#[test]
fn it_splits_a_wire_under_a_shifted_part()
{
	let mut simulation = series( Kind::Wire );

	add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 9, 0 ), ( 9, 2 ) );
	simulation.shift( 0, point( 9, 0 ), point( 9, 2 ), -5, 1 ).unwrap();

	let index = simulation.document( 0 ).unwrap().elements().iter().position( | element | element.id == 5 ).unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 7 );
	assert!( near( simulation.solution().unwrap().readings[ index ].currents[ 0 ], 0.0025, 1e-12 ) );
}

#[test]
fn it_keeps_the_wire_to_a_terminal_that_a_moved_part_now_covers()
{
	let mut simulation = Simulation::new( STEP );

	add( &mut simulation, Kind::Supply { volts : 5.0 }, ( 3, 0 ), ( 3, 0 ) );

	let toggle = add( &mut simulation, Kind::Toggle { high : false }, ( 5, 0 ), ( 3, 0 ) );

	simulation.transform( 0, toggle, Kind::Toggle { high : false }, segment( ( 4, 0 ), ( 2, 0 ) ), false ).unwrap();

	let wires : Vec<Segment> = simulation.document( 0 ).unwrap().elements().iter().filter( | element | element.kind == Kind::Wire ).map( | element | element.segment ).collect();

	assert_eq!( wires, vec![ segment( ( 3, 0 ), ( 2, 0 ) ) ] );
	assert_eq!( simulation.listening(), vec![ toggle ] );
}

#[test]
fn it_keeps_the_wire_to_a_terminal_that_a_shifted_part_now_covers()
{
	let mut simulation = Simulation::new( STEP );

	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 3, 0 ), ( 3, 4 ) );

	let toggle = add( &mut simulation, Kind::Toggle { high : false }, ( 5, 0 ), ( 3, 0 ) );

	simulation.shift( 0, point( 3, -1 ), point( 5, 1 ), -1, 0 ).unwrap();

	let wires : Vec<Segment> = simulation.document( 0 ).unwrap().elements().iter().filter( | element | element.kind == Kind::Wire ).map( | element | element.segment ).collect();

	assert_eq!( wires, vec![ segment( ( 3, 0 ), ( 2, 0 ) ) ] );
	assert_eq!( simulation.listening(), vec![ toggle ] );
}

#[test]
fn it_rejects_a_shift_of_an_empty_or_a_cut_selection()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );

	assert_eq!( simulation.shift( 0, point( 20, 20 ), point( 22, 22 ), 1, 0 ), Err( Error::Unselected ) );
	assert_eq!( simulation.shift( 0, point( -1, -1 ), point( 2, 2 ), 1, 0 ), Err( Error::Straddled ) );
	assert_eq!( simulation.shift( 1, point( 0, 0 ), point( 4, 4 ), 1, 0 ), Err( Error::Layerless ) );
	assert_eq!( simulation.undo(), Ok( Some( 4 ) ) );
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

fn define( simulation : &mut Simulation, parts : &[ Part ] ) -> u32
{
	let layer = simulation.create();

	for &( kind, from, to ) in parts { simulation.apply( layer, Operation::Add { kind, segment : segment( from, to ), mirrored : false } ).unwrap(); }

	measure( simulation );

	layer as u32
}

fn resistor( simulation : &mut Simulation, ohms : f64 ) -> u32
{
	define( simulation, &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Resistor { ohms }, ( 0, 0 ), ( 4, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] )
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

	measure( &mut simulation );

	assert!( near( simulation.solution().unwrap().readings[ 1 ].currents[ 0 ], 0.005, 1e-12 ) );
	assert!( near( simulation.solution().unwrap().readings[ 2 ].potentials[ 0 ], 2.5, 1e-9 ) );

	simulation.undo().unwrap();

	measure( &mut simulation );

	assert_eq!( simulation.document( layer as usize ).unwrap().elements()[ 1 ].kind, Kind::Resistor { ohms : 1000.0 } );
	assert!( near( simulation.solution().unwrap().readings[ 1 ].currents[ 0 ], 0.0025, 1e-12 ) );
}

#[test]
fn it_focuses_a_copy_of_a_symbol_and_solves_its_layer_alone()
{
	let mut simulation = Simulation::new( STEP );
	let layer = define( &mut simulation, &[ ( Kind::Input { high : true }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Wire, ( 0, 0 ), ( 2, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 2, 0 ), ( 4, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] );

	add( &mut simulation, Kind::Battery { volts : 2.0 }, ( 0, 4 ), ( 0, 0 ) );
	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 2, 0 ), ( 2, 0 ) );
	add( &mut simulation, Kind::Wire, ( 4, 0 ), ( 4, 4 ) );
	add( &mut simulation, Kind::Wire, ( 4, 4 ), ( 0, 4 ) );
	add( &mut simulation, Kind::Ground, ( 0, 4 ), ( 0, 4 ) );

	let whole = simulation.solution().unwrap().clone();

	assert!( near( whole.readings[ 1 ].currents[ 0 ], 0.002, 1e-12 ) );
	assert_eq!( simulation.focus( &[ 0 ] ), Err( Error::Missing ) );
	assert_eq!( simulation.focus( &[ 9 ] ), Err( Error::Missing ) );
	assert_eq!( simulation.focus( &[ 1 ] ), Ok( () ) );
	assert_eq!( simulation.path(), &[ 1 ] );

	let alone = simulation.solution().unwrap();

	assert_eq!( alone.readings.len(), 4 );
	assert!( near( alone.readings[ 0 ].potentials[ 0 ], 5.0, 1e-9 ) && near( alone.readings[ 3 ].potentials[ 0 ], 5.0, 1e-9 ) );
	assert!( near( alone.readings[ 2 ].currents[ 0 ], 0.0, 1e-12 ) );
	assert!( alone.branches.iter().all( | branch | branch.segment.from.x.abs() < 100 ) );

	simulation.focus( &[] ).unwrap();

	assert_eq!( simulation.solution().unwrap().readings[ 1 ], whole.readings[ 1 ] );
	assert_eq!( simulation.solution().unwrap().branches, whole.branches );
}

#[test]
fn it_leaves_a_focused_symbol_that_an_undo_removes()
{
	let mut simulation = Simulation::new( STEP );

	inverter( &mut simulation );
	simulation.focus( &[ 0 ] ).unwrap();
	simulation.undo().unwrap();

	assert_eq!( simulation.path(), &[] as &[ u32 ] );
	assert!( simulation.solution().unwrap().readings.is_empty() );
}

#[test]
fn it_reads_each_pin_of_a_symbol_in_the_order_of_its_layer()
{
	let mut simulation = Simulation::new( STEP );
	let layer = define( &mut simulation, &[
		( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) ),
		( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 8, 0 ) ),
		( Kind::Output, ( 9, 0 ), ( 8, 0 ) ),
		( Kind::Output, ( 4, -1 ), ( 4, 0 ) ),
	] );

	simulation.apply( layer as usize, Operation::Describe { name : String::new(), drawing : Vec::new(), layout : vec![ Point { x : -2, y : 0 }, Point { x : 2, y : 0 }, Point { x : 0, y : -2 } ] } ).unwrap();

	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 8, 14 ), ( 8, 10 ) );
	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) );
	add( &mut simulation, Kind::Wire, ( 12, 10 ), ( 12, 14 ) );
	add( &mut simulation, Kind::Wire, ( 12, 14 ), ( 8, 14 ) );
	add( &mut simulation, Kind::Ground, ( 8, 14 ), ( 8, 14 ) );

	let symbol = &simulation.solution().unwrap().readings[ 1 ];

	assert!( near( symbol.potentials[ 0 ], 5.0, 1e-9 ) && near( symbol.potentials[ 1 ], 0.0, 1e-9 ) && near( symbol.potentials[ 2 ], 2.5, 1e-9 ) );
	assert!( near( symbol.currents[ 2 ], 0.0, 1e-12 ) );
}

fn inverter( simulation : &mut Simulation ) -> u32
{
	let layer = define( simulation, &[
		( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		( Kind::Supply { volts : 5.0 }, ( 3, -1 ), ( 3, -1 ) ),
		( Kind::Output, ( 6, 0 ), ( 5, 0 ) ),
		( Kind::Wire, ( 0, -2 ), ( 0, 0 ) ),
		( Kind::Wire, ( 0, 0 ), ( 0, 2 ) ),
		( Kind::Pmos, ( 0, -2 ), ( 2, -2 ) ),
		( Kind::Nmos, ( 0, 2 ), ( 2, 2 ) ),
		( Kind::Wire, ( 3, -3 ), ( 5, -3 ) ),
		( Kind::Wire, ( 5, -3 ), ( 5, 0 ) ),
		( Kind::Wire, ( 5, 0 ), ( 5, 1 ) ),
		( Kind::Wire, ( 5, 1 ), ( 3, 1 ) ),
		( Kind::Ground, ( 3, 3 ), ( 3, 3 ) ),
	] );

	add( simulation, Kind::Symbol { layer, rotation : 0 }, ( 10, 10 ), ( 10, 10 ) );

	layer
}

#[test]
fn it_captions_each_toggle_with_the_label_of_a_pin_on_its_net_after_the_label_of_its_symbol()
{
	let mut simulation = Simulation::new( STEP );
	let layer = inverter( &mut simulation ) as usize;
	let end = Outline::of( simulation.document( layer ).unwrap() ).terminals( point( 10, 10 ), 0 )[ 0 ];

	simulation.apply( layer, Operation::Label { id : 0, label : "foo".to_string() } ).unwrap();
	add( &mut simulation, Kind::Toggle { high : false }, ( end.x - 4, end.y ), ( end.x - 3, end.y ) );
	add( &mut simulation, Kind::Wire, ( end.x - 3, end.y ), ( end.x, end.y ) );
	add( &mut simulation, Kind::Toggle { high : false }, ( 0, 20 ), ( 1, 20 ) );

	let output = add( &mut simulation, Kind::Output, ( 2, 20 ), ( 1, 20 ) );

	assert_eq!( simulation.captions(), vec![ "foo".to_string(), String::new() ] );

	simulation.apply( 0, Operation::Label { id : output, label : "bar".to_string() } ).unwrap();

	assert_eq!( simulation.captions(), vec![ "foo".to_string(), "bar".to_string() ] );

	simulation.apply( 0, Operation::Label { id : 0, label : "CTR 1-".to_string() } ).unwrap();

	assert_eq!( simulation.captions(), vec![ "CTR 1-foo".to_string(), "bar".to_string() ] );
}

#[test]
fn it_gives_the_delay_of_a_layer_that_runs_as_a_gate_and_none_for_an_analog_layer()
{
	let mut simulation = Simulation::new( STEP );
	let layer = inverter( &mut simulation );
	let analog = define( &mut simulation, &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] );

	add( &mut simulation, Kind::Symbol { layer : analog, rotation : 0 }, ( 30, 10 ), ( 30, 10 ) );

	assert_eq!( [ layer, analog, 9 ].map( | layer | simulation.delay( layer as usize ) ), [ Some( DELAY ), None, None ] );
}

#[test]
fn it_freezes_the_view_while_a_symbol_is_measured_and_solves_it_at_the_last_step()
{
	let mut simulation = Simulation::new( STEP );
	let layer = inverter( &mut simulation ) as usize;

	simulation.apply( layer, Operation::Add { kind : Kind::Wire, segment : segment( ( 20, 20 ), ( 22, 20 ) ), mirrored : false } ).unwrap();
	simulation.advance( 10 );

	assert!( simulation.frozen() );
	assert_eq!( simulation.solution(), Ok( &Solution::default() ) );

	measure( &mut simulation );

	assert!( ! simulation.frozen() );
	assert_eq!( simulation.progress(), 1.0 );
	assert_eq!( simulation.solution().unwrap().readings.len(), 1 );
}

#[test]
fn it_keeps_the_measure_of_a_symbol_through_an_edit_of_the_drawing()
{
	let mut simulation = Simulation::new( STEP );
	let layer = inverter( &mut simulation ) as usize;

	simulation.apply( layer, Operation::Add { kind : Kind::Wire, segment : segment( ( 20, 20 ), ( 22, 20 ) ), mirrored : false } ).unwrap();

	for _ in 0 .. 2 { simulation.proceed().unwrap(); }

	let progress = simulation.progress();

	add( &mut simulation, Kind::Wire, ( 0, 20 ), ( 4, 20 ) );

	assert!( simulation.frozen() && progress > 0.0 );
	assert_eq!( simulation.progress(), progress );
}

#[test]
fn it_keeps_the_model_of_a_symbol_whose_edit_is_undone_before_its_measure_ends()
{
	let mut simulation = Simulation::new( STEP );
	let layer = inverter( &mut simulation ) as usize;

	simulation.apply( layer, Operation::Add { kind : Kind::Wire, segment : segment( ( 20, 20 ), ( 22, 20 ) ), mirrored : false } ).unwrap();
	simulation.undo().unwrap();

	assert_eq!( ( simulation.frozen(), simulation.progress() ), ( false, 1.0 ) );
	assert_eq!( simulation.delay( layer ), Some( DELAY ) );
}

#[test]
fn it_solves_at_once_a_symbol_entered_while_it_is_measured()
{
	let mut simulation = Simulation::new( STEP );
	let layer = inverter( &mut simulation ) as usize;

	simulation.apply( layer, Operation::Add { kind : Kind::Wire, segment : segment( ( 20, 20 ), ( 22, 20 ) ), mirrored : false } ).unwrap();
	simulation.focus( &[ 0 ] ).unwrap();

	assert!( ! simulation.frozen() && simulation.progress() < 1.0 );
	assert_eq!( simulation.solution().unwrap().readings.len(), simulation.document( layer ).unwrap().elements().len() );
}

#[test]
fn it_solves_a_symbol_again_once_the_symbols_inside_it_are_measured()
{
	let mut simulation = Simulation::new( STEP );
	let layer = inverter( &mut simulation );
	let outer = define( &mut simulation, &[ ( Kind::Input { high : false }, ( -3, 0 ), ( -2, 0 ) ), ( Kind::Symbol { layer, rotation : 0 }, ( 0, 0 ), ( 0, 0 ) ), ( Kind::Output, ( 7, 0 ), ( 6, 0 ) ) ] );
	let symbol = add( &mut simulation, Kind::Symbol { layer : outer, rotation : 0 }, ( 30, 10 ), ( 30, 10 ) );

	measure( &mut simulation );

	simulation.apply( layer as usize, Operation::Add { kind : Kind::Wire, segment : segment( ( 20, 20 ), ( 22, 20 ) ), mirrored : false } ).unwrap();
	simulation.focus( &[ symbol ] ).unwrap();

	assert!( simulation.frozen() );

	while simulation.frozen() { simulation.proceed().unwrap(); }

	assert!( simulation.progress() < 1.0 );
	assert_eq!( simulation.solution().unwrap().readings.len(), 3 );
}

#[test]
fn it_runs_a_long_chain_of_inverters_as_gates()
{
	let mut simulation = Simulation::new( STEP );
	let layer = inverter( &mut simulation );

	for index in 1 .. 200 { add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 10 + 4 * index, 10 ), ( 10 + 4 * index, 10 ) ); }

	let output = | simulation : &Simulation, index : usize | simulation.solution().unwrap().readings[ index ].potentials[ 1 ];

	assert!( near( output( &simulation, 0 ), 5.0, 1e-3 ) && near( output( &simulation, 199 ), 0.0, 1e-3 ) );

	simulation.apply( layer as usize, Operation::SetHigh { id : 0, high : true } ).unwrap();

	assert!( near( output( &simulation, 0 ), 0.0, 1e-3 ) && near( output( &simulation, 199 ), 5.0, 1e-3 ) );
}

#[test]
fn it_steps_by_a_twentieth_of_the_period_of_the_fastest_clock()
{
	let mut simulation = Simulation::new( STEP );

	assert_eq!( ( simulation.period(), simulation.step() ), ( None, STEP ) );

	add( &mut simulation, Kind::Clock { hertz : 1e6 }, ( 0, 0 ), ( 2, 0 ) );
	add( &mut simulation, Kind::Clock { hertz : 1e7 }, ( 0, 4 ), ( 2, 4 ) );

	assert!( near( simulation.period().unwrap(), 1e-7, 1e-21 ) );
	assert!( near( simulation.step(), 5e-9, 1e-21 ) );
}

#[test]
fn it_names_the_engine_that_runs_the_view()
{
	let mut simulation = Simulation::new( STEP );

	assert_eq!( simulation.mode(), Mode::Analog );

	inverter( &mut simulation );

	assert_eq!( simulation.mode(), Mode::Mixed );

	add( &mut simulation, Kind::Clock { hertz : 1e6 }, ( 4, 10 ), ( 8, 10 ) );

	assert_eq!( simulation.mode(), Mode::Digital );
}

#[test]
fn it_runs_a_gate_that_a_toggle_drives_as_digital()
{
	let mut simulation = Simulation::new( STEP );

	inverter( &mut simulation );

	let toggle = add( &mut simulation, Kind::Toggle { high : false }, ( 4, 10 ), ( 8, 10 ) );
	let output = | simulation : &Simulation | simulation.solution().unwrap().readings[ 0 ].potentials[ 1 ];

	assert_eq!( simulation.mode(), Mode::Digital );
	assert!( near( output( &simulation ), 5.0, 1e-3 ) );

	simulation.apply( 0, Operation::SetHigh { id : toggle, high : true } ).unwrap();

	assert!( near( output( &simulation ), 0.0, 1e-3 ) );
}

#[test]
fn it_inverts_a_level_with_a_cmos_symbol_fed_by_its_own_vcc()
{
	let mut simulation = Simulation::new( STEP );

	inverter( &mut simulation );

	let input = add( &mut simulation, Kind::Battery { volts : 0.0 }, ( 4, 12 ), ( 4, 10 ) );

	add( &mut simulation, Kind::Wire, ( 4, 10 ), ( 8, 10 ) );
	add( &mut simulation, Kind::Ground, ( 4, 12 ), ( 4, 12 ) );

	let output = | simulation : &Simulation | simulation.solution().unwrap().readings[ 0 ].potentials[ 1 ];

	assert!( near( output( &simulation ), 5.0, 1e-3 ) );

	simulation.apply( 0, Operation::SetValue { id : input, value : 5.0 } ).unwrap();

	assert!( near( output( &simulation ), 0.0, 1e-3 ) );
	assert!( simulation.solution().unwrap().branches.iter().all( | branch | branch.segment.from.x.abs() < 100 ) );
}

#[test]
fn it_drives_the_free_input_of_a_symbol_with_its_level()
{
	let mut simulation = Simulation::new( STEP );
	let layer = inverter( &mut simulation ) as usize;
	let output = | simulation : &Simulation | simulation.solution().unwrap().readings[ 0 ].potentials[ 1 ];

	assert!( near( output( &simulation ), 5.0, 1e-3 ) );

	simulation.apply( layer, Operation::SetHigh { id : 0, high : true } ).unwrap();

	assert!( near( simulation.solution().unwrap().readings[ 0 ].potentials[ 0 ], 5.0, 1e-9 ) );
	assert!( near( output( &simulation ), 0.0, 1e-3 ) );

	simulation.undo().unwrap();

	assert!( near( output( &simulation ), 5.0, 1e-3 ) );
}

#[test]
fn it_solves_a_focused_symbol_alone_while_a_battery_drives_its_input_outside()
{
	let mut simulation = Simulation::new( STEP );

	inverter( &mut simulation );
	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 4, 12 ), ( 4, 10 ) );
	add( &mut simulation, Kind::Wire, ( 4, 10 ), ( 8, 10 ) );
	add( &mut simulation, Kind::Ground, ( 4, 12 ), ( 4, 12 ) );

	assert!( near( simulation.solution().unwrap().readings[ 0 ].potentials[ 1 ], 0.0, 1e-3 ) );

	simulation.focus( &[ 0 ] ).unwrap();

	assert!( near( simulation.solution().unwrap().readings[ 0 ].potentials[ 0 ], 0.0, 1e-9 ) );
	assert!( near( simulation.solution().unwrap().readings[ 2 ].potentials[ 0 ], 5.0, 1e-3 ) );
}

#[test]
fn it_drives_every_input_of_the_drawing()
{
	let mut simulation = Simulation::new( STEP );

	add( &mut simulation, Kind::Input { high : true }, ( -1, 0 ), ( 0, 0 ) );
	add( &mut simulation, Kind::Wire, ( 0, 0 ), ( 4, 0 ) );

	assert!( near( simulation.solution().unwrap().readings[ 1 ].potentials[ 1 ], 5.0, 1e-9 ) );
}

#[test]
fn it_keeps_the_charge_inside_a_symbol_after_a_change_elsewhere()
{
	let mut simulation = Simulation::new( STEP );
	let layer = define( &mut simulation, &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Capacitor { farads : 1e-6 }, ( 0, 0 ), ( 4, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] );

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
fn it_reads_the_voltage_at_the_end_of_the_lead_of_an_output_without_a_current()
{
	let mut simulation = series( Kind::Wire );

	add( &mut simulation, Kind::Output, ( 5, 2 ), ( 4, 2 ) );

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

fn ladder( simulation : &mut Simulation )
{
	add( simulation, Kind::Battery { volts : 5.0 }, ( 0, 6 ), ( 0, 0 ) );
	add( simulation, Kind::Wire, ( 0, 0 ), ( 2, 0 ) );
	add( simulation, Kind::Input { high : false }, ( 2, 1 ), ( 2, 0 ) );
	add( simulation, Kind::Resistor { ohms : 1000.0 }, ( 2, 0 ), ( 6, 0 ) );
	add( simulation, Kind::Output, ( 6, 1 ), ( 6, 0 ) );
	add( simulation, Kind::Wire, ( 6, 0 ), ( 8, 0 ) );
	add( simulation, Kind::Resistor { ohms : 1000.0 }, ( 8, 0 ), ( 8, 6 ) );
	add( simulation, Kind::Wire, ( 8, 6 ), ( 0, 6 ) );
	add( simulation, Kind::Ground, ( 0, 6 ), ( 0, 6 ) );
}

#[test]
fn it_encloses_a_selection_into_a_symbol_that_solves_the_same()
{
	let mut simulation = Simulation::new( STEP );

	ladder( &mut simulation );

	let before = simulation.document( 0 ).unwrap().elements().to_vec();
	let symbol = simulation.enclose( 0, Point { x : 1, y : -2 }, Point { x : 7, y : 2 }, "", &[], &[] ).unwrap();

	measure( &mut simulation );

	let drawing = simulation.document( 0 ).unwrap().elements();
	let wires : Vec<Segment> = drawing.iter().filter( | element | element.kind == Kind::Wire ).map( | element | element.segment ).collect();
	let index = drawing.iter().position( | element | element.id == symbol ).unwrap();

	assert_eq!( drawing[ index ].kind, Kind::Symbol { layer : 1, rotation : 0 } );
	assert_eq!( drawing[ index ].segment, segment( ( 4, 0 ), ( 4, 0 ) ) );
	assert_eq!( wires, vec![ segment( ( 0, 0 ), ( 2, 0 ) ), segment( ( 6, 0 ), ( 8, 0 ) ), segment( ( 8, 6 ), ( 0, 6 ) ) ] );
	assert_eq!( simulation.document( 1 ).unwrap().elements().len(), 3 );
	assert!( near( simulation.solution().unwrap().readings[ index ].currents[ 0 ], 0.0025, 1e-12 ) );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );
	assert!( simulation.document( 1 ).unwrap().elements().is_empty() );
}

#[test]
fn it_keeps_a_vcc_inside_an_enclosed_selection()
{
	let mut simulation = Simulation::new( STEP );

	add( &mut simulation, Kind::Supply { volts : 5.0 }, ( 2, 0 ), ( 2, 0 ) );
	add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 2, 0 ), ( 6, 0 ) );
	add( &mut simulation, Kind::Output, ( 6, 1 ), ( 6, 0 ) );
	add( &mut simulation, Kind::Wire, ( 6, 0 ), ( 10, 0 ) );
	add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 10, 0 ), ( 10, 4 ) );
	add( &mut simulation, Kind::Ground, ( 10, 4 ), ( 10, 4 ) );

	let symbol = simulation.enclose( 0, Point { x : 1, y : -2 }, Point { x : 7, y : 2 }, "", &[], &[] ).unwrap();

	measure( &mut simulation );

	let drawing = simulation.document( 0 ).unwrap().elements();
	let index = drawing.iter().position( | element | element.id == symbol ).unwrap();

	assert!( simulation.document( 1 ).unwrap().elements().iter().any( | element | element.kind == Kind::Supply { volts : 5.0 } ) );
	assert!( drawing.iter().all( | element | element.kind != Kind::Supply { volts : 5.0 } ) );
	assert!( near( simulation.solution().unwrap().readings[ index ].currents[ 0 ].abs(), 0.0025, 1e-12 ) );
}

#[test]
fn it_rejects_a_selection_across_a_part_without_a_new_layer()
{
	let mut simulation = Simulation::new( STEP );

	ladder( &mut simulation );

	assert_eq!( simulation.enclose( 0, Point { x : 3, y : -2 }, Point { x : 7, y : 2 }, "", &[], &[] ), Err( Error::Straddled ) );
	assert_eq!( simulation.enclose( 1, Point { x : 1, y : -2 }, Point { x : 7, y : 2 }, "", &[], &[] ), Err( Error::Layerless ) );
	assert!( simulation.document( 1 ).is_none() );
	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 9 );
}

#[test]
fn it_finds_the_parts_at_a_point_topmost_first()
{
	let mut simulation = Simulation::new( STEP );

	add( &mut simulation, Kind::Wire, ( 0, 0 ), ( 4, 0 ) );
	add( &mut simulation, Kind::Wire, ( 4, 0 ), ( 4, 4 ) );
	add( &mut simulation, Kind::Ground, ( 4, 4 ), ( 4, 4 ) );

	assert_eq!( simulation.elements_at( 0, Point { x : 2, y : 0 } ), vec![ 0 ] );
	assert_eq!( simulation.elements_at( 0, Point { x : 4, y : 0 } ), vec![ 1, 0 ] );
	assert_eq!( simulation.elements_at( 0, Point { x : 4, y : 4 } ), vec![ 2, 1 ] );
	assert!( simulation.elements_at( 0, Point { x : 9, y : 9 } ).is_empty() );
	assert!( simulation.elements_at( 1, Point { x : 2, y : 0 } ).is_empty() );
}

#[test]
fn it_finds_a_transistor_at_its_collector_and_its_emitter()
{
	let mut simulation = Simulation::new( STEP );

	add( &mut simulation, Kind::Npn, ( 0, 0 ), ( 2, 0 ) );

	assert_eq!( simulation.elements_at( 0, Point { x : 1, y : 0 } ), vec![ 0 ] );
	assert_eq!( simulation.elements_at( 0, Point { x : 3, y : -1 } ), vec![ 0 ] );
	assert_eq!( simulation.elements_at( 0, Point { x : 3, y : 1 } ), vec![ 0 ] );
	assert!( simulation.elements_at( 0, Point { x : 3, y : 0 } ).is_empty() );
}

#[test]
fn it_finds_a_symbol_anywhere_on_its_turned_box()
{
	let mut simulation = Simulation::new( STEP );
	let layer = define( &mut simulation, &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Input { high : false }, ( -1, 2 ), ( 0, 2 ) ), ( Kind::Input { high : false }, ( -1, 4 ), ( 0, 4 ) ), ( Kind::Output, ( 5, 2 ), ( 4, 2 ) ) ] );

	simulation.apply( layer as usize, Operation::Describe { name : String::new(), drawing : Vec::new(), layout : vec![ Point { x : -2, y : -4 }, Point { x : -2, y : 0 }, Point { x : -2, y : 4 }, Point { x : 2, y : 0 } ] } ).unwrap();

	add( &mut simulation, Kind::Symbol { layer, rotation : 1 }, ( 10, 10 ), ( 10, 10 ) );

	assert_eq!( simulation.elements_at( 0, Point { x : 13, y : 12 } ), vec![ 0 ] );
	assert!( simulation.elements_at( 0, Point { x : 12, y : 13 } ).is_empty() );
}

#[test]
fn it_finds_a_symbol_on_its_drawing_but_not_on_a_wire_beside_its_center_away_from_the_drawing()
{
	let mut simulation = Simulation::new( STEP );
	let layer = define( &mut simulation, &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] );

	simulation.apply( layer as usize, Operation::Describe { name : String::new(), drawing : vec![ Shape::Line { from : Point { x : -32, y : -28 }, to : Point { x : -12, y : -8 } } ], layout : vec![ Point { x : -8, y : -6 }, Point { x : -3, y : -5 } ] } ).unwrap();

	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 24, 23 ), ( 24, 23 ) );
	add( &mut simulation, Kind::Wire, ( 21, 18 ), ( 24, 18 ) );

	assert_eq!( simulation.elements_at( 0, Point { x : 19, y : 20 } ), vec![ 0 ] );
	assert_eq!( simulation.elements_at( 0, Point { x : 23, y : 18 } ), vec![ 1 ] );
	assert!( simulation.elements_at( 0, Point { x : 24, y : 23 } ).is_empty() );
}

#[test]
fn it_shifts_a_symbol_whose_box_and_pins_are_inside_the_selection_but_whose_center_is_not()
{
	let mut simulation = Simulation::new( STEP );
	let layer = define( &mut simulation, &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] );

	simulation.apply( layer as usize, Operation::Describe { name : String::new(), drawing : vec![ Shape::Line { from : Point { x : -32, y : -28 }, to : Point { x : -12, y : -8 } } ], layout : vec![ Point { x : -8, y : -6 }, Point { x : -3, y : -5 } ] } ).unwrap();

	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 24, 23 ), ( 24, 23 ) );

	simulation.shift( 0, point( 15, 15 ), point( 22, 21 ), 0, -10 ).unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements()[ 0 ].segment, segment( ( 24, 13 ), ( 24, 13 ) ) );
}

#[test]
fn it_encloses_a_symbol_whose_box_and_pins_are_inside_the_selection_but_whose_center_is_not()
{
	let mut simulation = Simulation::new( STEP );
	let layer = define( &mut simulation, &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] );

	simulation.apply( layer as usize, Operation::Describe { name : String::new(), drawing : vec![ Shape::Line { from : Point { x : -32, y : -28 }, to : Point { x : -12, y : -8 } } ], layout : vec![ Point { x : -8, y : -6 }, Point { x : -3, y : -5 } ] } ).unwrap();

	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 24, 23 ), ( 24, 23 ) );

	assert!( simulation.enclose( 0, point( 15, 15 ), point( 22, 21 ), "", &[], &[] ).is_ok() );
}

#[test]
fn it_erases_several_parts_with_one_undo_step()
{
	let mut simulation = series( Kind::Resistor { ohms : 1000.0 } );
	let before = simulation.document( 0 ).unwrap().elements().to_vec();

	assert_eq!( simulation.erase( 0, &[ 1, 3 ] ), Ok( 3 ) );
	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 3 );
	assert_eq!( simulation.erase( 0, &[] ), Err( Error::Unselected ) );
	assert_eq!( simulation.erase( 0, &[ 0, 7 ] ), Err( Error::Missing ) );
	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 3 );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements(), before.as_slice() );
}

#[test]
fn it_turns_an_enclosed_symbol_and_routes_its_wires_around_its_box()
{
	let mut simulation = Simulation::new( STEP );

	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 10, 16 ), ( 10, 10 ) );
	add( &mut simulation, Kind::Wire, ( 10, 10 ), ( 12, 10 ) );
	add( &mut simulation, Kind::Input { high : false }, ( 12, 11 ), ( 12, 10 ) );
	add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 12, 10 ), ( 16, 10 ) );
	add( &mut simulation, Kind::Output, ( 16, 11 ), ( 16, 10 ) );
	add( &mut simulation, Kind::Wire, ( 16, 10 ), ( 18, 10 ) );
	add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 18, 10 ), ( 18, 16 ) );
	add( &mut simulation, Kind::Wire, ( 18, 16 ), ( 10, 16 ) );
	add( &mut simulation, Kind::Ground, ( 10, 16 ), ( 10, 16 ) );

	let symbol = simulation.enclose( 0, Point { x : 11, y : 8 }, Point { x : 17, y : 12 }, "", &[], &[] ).unwrap();

	measure( &mut simulation );

	let layer = 1;

	simulation.transform( 0, symbol, Kind::Symbol { layer, rotation : 1 }, segment( ( 14, 10 ), ( 14, 10 ) ), false ).unwrap();

	let drawing = simulation.document( 0 ).unwrap().elements();
	let index = drawing.iter().position( | element | element.id == symbol ).unwrap();
	let inside = | point : Point | point.x > 12 && point.x < 16 && point.y > 8 && point.y < 12;

	assert!( drawing.iter().filter( | element | element.kind == Kind::Wire ).all( | element | ! inside( element.segment.from ) && ! inside( element.segment.to ) ) );
	assert!( near( simulation.solution().unwrap().readings[ index ].currents[ 0 ], 0.0025, 1e-12 ) );
}

#[test]
fn it_encloses_a_named_and_drawn_symbol_with_the_labels_and_the_layout_of_its_pins_in_one_undo_step()
{
	let mut simulation = Simulation::new( STEP );
	let low = Point { x : 2, y : -2 };
	let high = Point { x : 8, y : 2 };

	add( &mut simulation, Kind::Battery { volts : 5.0 }, ( 0, 4 ), ( 0, 0 ) );
	add( &mut simulation, Kind::Wire, ( 0, 0 ), ( 4, 0 ) );

	let input = add( &mut simulation, Kind::Input { high : false }, ( 4, 1 ), ( 4, 0 ) );

	add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 6, 0 ) );

	let output = add( &mut simulation, Kind::Output, ( 6, 1 ), ( 6, 0 ) );

	add( &mut simulation, Kind::Wire, ( 6, 0 ), ( 10, 0 ) );
	add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 10, 0 ), ( 10, 4 ) );
	add( &mut simulation, Kind::Wire, ( 10, 4 ), ( 0, 4 ) );
	add( &mut simulation, Kind::Ground, ( 0, 4 ), ( 0, 4 ) );

	simulation.apply( 0, Operation::Label { id : input, label : "foo".to_string() } ).unwrap();
	simulation.apply( 0, Operation::Label { id : output, label : "bar".to_string() } ).unwrap();

	let drawing = [ Shape::Circle { center : Point { x : 0, y : 0 }, radius : 2 } ];
	let layout = [ Point { x : -3, y : 0 }, Point { x : 3, y : 0 } ];
	let symbol = simulation.enclose( 0, low, high, "qux", &drawing, &layout ).unwrap();

	measure( &mut simulation );

	let layer = simulation.document( 1 ).unwrap();
	let labels : Vec<&str> = layer.elements().iter().filter( | element | element.kind.pin() ).map( | element | layer.label( element.id ) ).collect();
	let wires : Vec<Segment> = simulation.document( 0 ).unwrap().elements().iter().filter( | element | element.kind == Kind::Wire ).map( | element | element.segment ).collect();
	let index = simulation.document( 0 ).unwrap().elements().iter().position( | element | element.id == symbol ).unwrap();

	assert_eq!( ( layer.name(), layer.drawing(), layer.layout() ), ( "qux", drawing.as_slice(), layout.as_slice() ) );
	assert_eq!( labels, vec![ "foo", "bar" ] );
	assert_eq!( Outline::of( layer ).pins, layout.to_vec() );
	assert!( wires.contains( &segment( ( 0, 0 ), ( 2, 0 ) ) ) && wires.contains( &segment( ( 8, 0 ), ( 10, 0 ) ) ) );
	assert!( near( simulation.solution().unwrap().readings[ index ].currents[ 0 ].abs(), 0.0025, 1e-12 ) );

	simulation.undo().unwrap();

	assert_eq!( ( simulation.document( 1 ).unwrap().name(), simulation.document( 1 ).unwrap().layout() ), ( "", [].as_slice() ) );
	assert!( simulation.document( 1 ).unwrap().elements().is_empty() );
}

#[test]
fn it_centers_a_described_icon_on_its_box_and_keeps_each_copy_in_place_in_one_undo_step()
{
	let mut simulation = Simulation::new( STEP );
	let layer = define( &mut simulation, &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] );
	let drawing = Shape::Line { from : Point { x : -32, y : -28 }, to : Point { x : -12, y : -8 } };

	simulation.apply( layer as usize, Operation::Describe { name : String::new(), drawing : vec![ drawing.clone() ], layout : vec![ point( -8, -6 ), point( -3, -5 ) ] } ).unwrap();

	let symbol = add( &mut simulation, Kind::Symbol { layer, rotation : 1 }, ( 24, 23 ), ( 24, 23 ) );
	let terminals = Outline::of( simulation.document( layer as usize ).unwrap() ).terminals( point( 24, 23 ), 1 );

	simulation.describe( layer as usize, "foo", &[ drawing.shifted( 20, 12 ) ], &[ point( -3, -3 ), point( 2, -2 ) ] ).unwrap();

	let outline = Outline::of( simulation.document( layer as usize ).unwrap() );
	let center = simulation.document( 0 ).unwrap().elements().iter().find( | element | element.id == symbol ).unwrap().segment.from;

	assert_eq!( simulation.document( layer as usize ).unwrap().drawing(), [ Shape::Line { from : Point { x : -8, y : -8 }, to : Point { x : 12, y : 12 } } ].as_slice() );
	assert_eq!( ( outline.middle(), outline.terminals( center, 1 ) ), ( point( 0, 0 ), terminals ) );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( 0 ).unwrap().elements()[ 0 ].segment, segment( ( 24, 23 ), ( 24, 23 ) ) );
	assert_eq!( simulation.document( layer as usize ).unwrap().drawing(), [ drawing ].as_slice() );
}

#[test]
fn it_encloses_a_symbol_with_its_icon_centered_on_its_box()
{
	let mut simulation = Simulation::new( STEP );

	add( &mut simulation, Kind::Input { high : false }, ( 4, 1 ), ( 4, 0 ) );
	add( &mut simulation, Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 6, 0 ) );
	add( &mut simulation, Kind::Output, ( 6, 1 ), ( 6, 0 ) );

	let symbol = simulation.enclose( 0, point( 2, -2 ), point( 8, 2 ), "", &[ Shape::Circle { center : Point { x : 20, y : 8 }, radius : 2 } ], &[ point( 3, 2 ), point( 7, 2 ) ] ).unwrap();
	let layer = simulation.document( 1 ).unwrap();

	assert_eq!( ( layer.drawing(), layer.layout() ), ( [ Shape::Circle { center : Point { x : 0, y : 0 }, radius : 2 } ].as_slice(), [ point( -2, 0 ), point( 2, 0 ) ].as_slice() ) );
	assert_eq!( simulation.document( 0 ).unwrap().elements().iter().find( | element | element.id == symbol ).unwrap().segment, segment( ( 5, 0 ), ( 5, 0 ) ) );
}

#[test]
fn it_clears_the_names_the_drawings_and_the_layouts_of_the_layers_on_an_open()
{
	let mut simulation = Simulation::new( STEP );
	let layer = resistor( &mut simulation, 1000.0 );

	simulation.apply( layer as usize, Operation::Describe { name : "foo".to_string(), drawing : vec![ Shape::Circle { center : Point { x : 0, y : 0 }, radius : 1 } ], layout : vec![ Point { x : 0, y : 0 } ] } ).unwrap();
	simulation.open( Vec::new() ).unwrap();

	assert_eq!( ( simulation.document( 1 ).unwrap().name(), simulation.document( 1 ).unwrap().drawing(), simulation.document( 1 ).unwrap().layout() ), ( "", [].as_slice(), [].as_slice() ) );
}

#[test]
fn it_purges_a_symbol_with_every_copy_on_every_layer_in_one_undo_step()
{
	let mut simulation = Simulation::new( STEP );
	let layer = resistor( &mut simulation, 1000.0 );
	let outer = define( &mut simulation, &[ ( Kind::Symbol { layer, rotation : 0 }, ( 2, 0 ), ( 2, 0 ) ) ] );

	add( &mut simulation, Kind::Symbol { layer, rotation : 0 }, ( 2, 0 ), ( 2, 0 ) );
	add( &mut simulation, Kind::Wire, ( 6, 0 ), ( 8, 0 ) );
	add( &mut simulation, Kind::Symbol { layer : outer, rotation : 0 }, ( 2, 6 ), ( 2, 6 ) );

	assert!( simulation.purge( layer as usize ).is_ok() );
	assert!( simulation.document( layer as usize ).unwrap().elements().is_empty() );
	assert!( simulation.document( outer as usize ).unwrap().elements().is_empty() );
	assert_eq!( simulation.document( 0 ).unwrap().elements().iter().map( | element | element.kind ).collect::<Vec<_>>(), vec![ Kind::Wire, Kind::Symbol { layer : outer, rotation : 0 } ] );

	simulation.undo().unwrap();

	assert_eq!( simulation.document( layer as usize ).unwrap().elements().len(), 3 );
	assert_eq!( simulation.document( outer as usize ).unwrap().elements().len(), 1 );
	assert_eq!( simulation.document( 0 ).unwrap().elements().len(), 3 );
}

#[test]
fn it_rejects_a_purge_of_the_drawing_or_a_missing_layer()
{
	let mut simulation = Simulation::new( STEP );

	assert_eq!( simulation.purge( 0 ), Err( Error::Layerless ) );
	assert_eq!( simulation.purge( 3 ), Err( Error::Layerless ) );
}

fn nand() -> Vec<Part>
{
	vec![
		( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		( Kind::Input { high : false }, ( -3, 4 ), ( -2, 4 ) ),
		( Kind::Output, ( 12, 0 ), ( 11, 0 ) ),
		( Kind::Supply { volts : 5.0 }, ( 3, -1 ), ( 3, -1 ) ),
		( Kind::Supply { volts : 5.0 }, ( 9, -1 ), ( 9, -1 ) ),
		( Kind::Pmos, ( 0, -2 ), ( 2, -2 ) ),
		( Kind::Pmos, ( 6, -2 ), ( 8, -2 ) ),
		( Kind::Nmos, ( 0, 2 ), ( 2, 2 ) ),
		( Kind::Nmos, ( 0, 6 ), ( 2, 6 ) ),
		( Kind::Ground, ( 3, 7 ), ( 3, 7 ) ),
		( Kind::Wire, ( 0, -2 ), ( 0, 0 ) ),
		( Kind::Wire, ( 0, 0 ), ( 0, 2 ) ),
		( Kind::Wire, ( 6, -2 ), ( 6, 4 ) ),
		( Kind::Wire, ( 6, 4 ), ( -2, 4 ) ),
		( Kind::Wire, ( -2, 4 ), ( -2, 6 ) ),
		( Kind::Wire, ( -2, 6 ), ( 0, 6 ) ),
		( Kind::Wire, ( 3, -3 ), ( 9, -3 ) ),
		( Kind::Wire, ( 9, -3 ), ( 11, -3 ) ),
		( Kind::Wire, ( 11, -3 ), ( 11, 0 ) ),
		( Kind::Wire, ( 11, 0 ), ( 11, 1 ) ),
		( Kind::Wire, ( 11, 1 ), ( 3, 1 ) ),
		( Kind::Wire, ( 3, 3 ), ( 3, 5 ) ),
	]
}

fn flip_flop() -> Vec<Part>
{
	let input = | gate : i32, side : i32 | ( 10 * gate - 2, side );
	let output = | gate : i32 | ( 10 * gate + 2, 0 );
	let nets = [
		vec![ ( -20, 0 ), input( 1, -1 ) ],
		vec![ ( -16, 0 ), input( 0, -1 ), input( 0, 1 ), input( 5, 1 ), input( 6, 1 ) ],
		vec![ output( 0 ), input( 1, 1 ), input( 2, 1 ) ],
		vec![ output( 1 ), input( 2, -1 ), input( 3, -1 ) ],
		vec![ output( 2 ), input( 4, -1 ) ],
		vec![ output( 3 ), input( 4, 1 ), input( 5, -1 ) ],
		vec![ output( 4 ), input( 3, 1 ) ],
		vec![ output( 5 ), input( 6, -1 ), input( 7, -1 ) ],
		vec![ output( 6 ), input( 8, -1 ) ],
		vec![ output( 7 ), input( 8, 1 ), ( 100, 0 ) ],
		vec![ output( 8 ), input( 7, 1 ), ( 104, 0 ) ],
	];
	let pins = [
		( Kind::Input { high : false }, ( -21, 0 ), ( -20, 0 ) ),
		( Kind::Input { high : false }, ( -17, 0 ), ( -16, 0 ) ),
		( Kind::Output, ( 101, 0 ), ( 100, 0 ) ),
		( Kind::Output, ( 105, 0 ), ( 104, 0 ) ),
	];
	let gates = ( 0 .. 9 ).map( | gate | ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10 * gate, 0 ), ( 10 * gate, 0 ) ) );
	let wires = nets.iter().zip( 3 .. ).flat_map( | ( net, lane ) |
	{
		let stubs = net.iter().flat_map( move | &( x, side ) | match side
		{
			0 => vec![ ( Kind::Wire, ( x, - lane ), ( x, 0 ) ), ( Kind::Wire, ( x, 0 ), ( x, lane ) ) ],
			_ => vec![ ( Kind::Wire, ( x, side ), ( x, side.signum() * lane ) ) ],
		} );
		let span = | y : i32, sign : i32 |
		{
			let mut columns : Vec<i32> = net.iter().filter( | &&( _, side ) | side * sign >= 0 ).map( | &( x, _ ) | x ).collect();

			columns.sort();

			columns.windows( 2 ).map( | pair | ( Kind::Wire, ( pair[ 0 ], y ), ( pair[ 1 ], y ) ) ).collect::<Vec<_>>()
		};

		stubs.chain( span( - lane, -1 ) ).chain( span( lane, 1 ) ).collect::<Vec<_>>()
	} );

	pins.into_iter().chain( gates ).chain( wires ).collect()
}

fn counter() -> Simulation
{
	let mut simulation = Simulation::new( STEP );
	let mut parts : Vec<Part> = ( 0 .. 4 ).map( | bit | ( Kind::Symbol { layer : 2, rotation : 0 }, ( 20 * bit, 0 ), ( 20 * bit, 0 ) ) ).collect();

	parts.push( ( Kind::Clock { hertz : 1e6 }, ( -8, 1 ), ( -2, 1 ) ) );
	parts.extend( ( 0 .. 4 ).map( | bit | 20 * bit ).flat_map( | x | [ ( Kind::Wire, ( x + 2, 1 ), ( x + 2, 4 ) ), ( Kind::Wire, ( x + 2, 4 ), ( x - 4, 4 ) ), ( Kind::Wire, ( x - 4, 4 ), ( x - 4, -1 ) ), ( Kind::Wire, ( x - 4, -1 ), ( x - 2, -1 ) ) ] ) );
	parts.extend( ( 0 .. 3 ).map( | bit | ( Kind::Wire, ( 20 * bit + 2, 1 ), ( 20 * bit + 18, 1 ) ) ) );

	let layers = [ parts, nand(), flip_flop() ];
	let additions = layers.iter().enumerate().flat_map( | ( layer, parts ) | parts.iter().map( move | &( kind, from, to ) | ( layer, Operation::Add { kind, segment : segment( from, to ), mirrored : false } ) ) ).collect();

	simulation.open( additions ).unwrap();

	measure( &mut simulation );

	simulation
}

fn count( simulation : &Simulation ) -> usize
{
	let readings = &simulation.solution().unwrap().readings;

	( 0 .. 4 ).map( | bit | ( ( readings[ bit ].potentials[ 2 ] > 2.5 ) as usize ) << bit ).sum()
}

#[test]
fn it_counts_the_rising_edges_of_a_clock_with_flip_flops_and_keeps_the_count_after_an_edit()
{
	let mut simulation = counter();
	let start = count( &simulation );

	assert_eq!( simulation.mode(), Mode::Digital );

	simulation.advance( 100 );

	assert_eq!( count( &simulation ), ( start + 5 ) % 16 );

	add( &mut simulation, Kind::Wire, ( 0, 20 ), ( 4, 20 ) );

	assert_eq!( count( &simulation ), ( start + 5 ) % 16 );

	simulation.advance( 60 );

	assert_eq!( count( &simulation ), ( start + 8 ) % 16 );
}

#[test]
fn it_keeps_the_count_of_the_flip_flops_after_a_visit_inside_one_of_them()
{
	let mut simulation = counter();
	let start = count( &simulation );

	simulation.advance( 100 );
	simulation.focus( &[ 1 ] ).unwrap();
	simulation.focus( &[] ).unwrap();

	assert_eq!( count( &simulation ), ( start + 5 ) % 16 );
}
