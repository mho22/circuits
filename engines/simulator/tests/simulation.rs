use simulator::{ Error, Kind, Operation, Point, Segment, Simulation };


const STEP : f64 = 1e-6;


fn add( simulation : &mut Simulation, kind : Kind, from : ( i32, i32 ), to : ( i32, i32 ) ) -> u32
{
	simulation.apply( Operation::Add { kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } } } ).unwrap()
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

	assert_eq!( simulation.document().elements().len(), 5 );
	assert!( near( simulation.solution().unwrap().readings[ 1 ].current, 0.0025, 1e-12 ) );
}

#[test]
fn it_charges_a_capacitor_along_the_rc_curve()
{
	let mut simulation = series( Kind::Capacitor { farads : 1e-6 } );

	for milliseconds in 1 ..= 3
	{
		simulation.advance( 1000 );

		let expected = 5.0 * ( 1.0 - ( - milliseconds as f64 ).exp() );

		assert!( near( simulation.solution().unwrap().readings[ 2 ].from, expected, 2e-3 ) );
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

		assert!( near( simulation.solution().unwrap().readings[ 2 ].current, expected, 2e-6 ) );
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

	simulation.apply( Operation::Delete { id : short } ).unwrap();

	assert!( near( simulation.solution().unwrap().readings[ 2 ].from, 5.0 * ( 1.0 - ( -1.0_f64 ).exp() ), 1e-2 ) );
}

#[test]
fn it_opens_and_closes_a_switch()
{
	let mut simulation = series( Kind::Switch { closed : false } );

	assert_eq!( simulation.solution().unwrap().readings[ 1 ].current, 0.0 );

	simulation.apply( Operation::SetClosed { id : 2, closed : true } ).unwrap();

	assert!( near( simulation.solution().unwrap().readings[ 1 ].current, 0.005, 1e-12 ) );
}

#[test]
fn it_rejects_an_invalid_operation()
{
	let mut simulation = Simulation::new( STEP );

	assert_eq!( simulation.apply( Operation::Delete { id : 7 } ), Err( Error::Missing ) );
	assert!( simulation.solution().unwrap().readings.is_empty() );
}
