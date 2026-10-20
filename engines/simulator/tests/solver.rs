use simulator::{ solve, Element, Error, History, Kind, Point, Reading, Segment, Solution };


const STEP : f64 = 1e-3;


fn element( kind : Kind, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { id : 0, kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } } }
}

fn near( a : f64, b : f64 ) -> bool
{
	( a - b ).abs() < 1e-9
}

fn loop_with( resistors : &[ Element ] ) -> Vec<Element>
{
	let mut elements = vec![
		element( Kind::Battery { volts : 5.0 }, ( 0, 4 ), ( 0, 0 ) ),
		element( Kind::Wire, ( 0, 0 ), ( 4, 0 ) ),
		element( Kind::Wire, ( 0, 4 ), ( 4, 4 ) ),
		element( Kind::Ground, ( 0, 4 ), ( 0, 4 ) ),
	];

	elements.extend_from_slice( resistors );

	elements
}


#[test]
fn it_solves_an_empty_circuit()
{
	let solution = solve( &[], &History::default(), STEP ).unwrap();

	assert!( solution.readings.is_empty() );
	assert!( solution.branches.is_empty() );
}

#[test]
fn it_applies_ohms_law_to_a_battery_and_a_resistor()
{
	let solution = solve( &loop_with( &[ element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 4 ) ) ] ), &History::default(), STEP ).unwrap();

	let battery = solution.readings[ 0 ];
	let resistor = solution.readings[ 4 ];

	assert_eq!( ( battery.from, battery.to ), ( 0.0, 5.0 ) );
	assert!( near( battery.current, 0.005 ) );
	assert_eq!( ( resistor.from, resistor.to ), ( 5.0, 0.0 ) );
	assert!( near( resistor.current, 0.005 ) );
}

#[test]
fn it_divides_a_voltage_between_resistors_in_series()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 2 ) ),
		element( Kind::Resistor { ohms : 3000.0 }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	assert!( near( solution.readings[ 4 ].to, 3.75 ) );
	assert!( near( solution.readings[ 5 ].current, 0.00125 ) );
}

#[test]
fn it_splits_a_current_between_resistors_in_parallel()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 4 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 2, 0 ), ( 2, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	assert!( near( solution.readings[ 0 ].current, 0.01 ) );
	assert!( near( solution.readings[ 4 ].current, 0.005 ) );
	assert!( near( solution.readings[ 5 ].current, 0.005 ) );
}

#[test]
fn it_carries_the_current_through_each_piece_of_wire()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 4 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 2, 0 ), ( 2, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	let current = | x : i32, y : i32 | solution.branches[ solution.branch_at( Point { x, y } ).unwrap() ].current.abs();

	assert!( near( current( 1, 0 ), 0.01 ) );
	assert!( near( current( 3, 0 ), 0.005 ) );
	assert!( near( current( 1, 4 ), 0.01 ) );
	assert!( near( current( 3, 4 ), 0.005 ) );
}

#[test]
fn it_gives_each_wire_the_voltage_of_its_net()
{
	let solution = solve( &loop_with( &[ element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 4 ) ) ] ), &History::default(), STEP ).unwrap();

	assert_eq!( solution.readings[ 1 ].from, 5.0 );
	assert_eq!( solution.readings[ 2 ].from, 0.0 );
	assert!( solution.readings[ 1 ].current.is_nan() );
	assert!( solution.branches.iter().all( | branch | branch.voltage == 5.0 || branch.voltage == 0.0 ) );
}

#[test]
fn it_carries_current_through_the_earth_between_grounds()
{
	let solution = solve( &[
		element( Kind::Battery { volts : 2.0 }, ( 0, 2 ), ( 0, 0 ) ),
		element( Kind::Resistor { ohms : 100.0 }, ( 0, 0 ), ( 5, 0 ) ),
		element( Kind::Ground, ( 0, 2 ), ( 0, 2 ) ),
		element( Kind::Ground, ( 5, 0 ), ( 5, 0 ) ),
	], &History::default(), STEP ).unwrap();

	assert!( near( solution.readings[ 1 ].current, 0.02 ) );
	assert!( near( solution.readings[ 2 ].current, -0.02 ) );
	assert!( near( solution.readings[ 3 ].current, 0.02 ) );
}

#[test]
fn it_references_a_floating_circuit_to_one_of_its_nets()
{
	let solution = solve( &[
		element( Kind::Battery { volts : 3.0 }, ( 0, 0 ), ( 0, 2 ) ),
		element( Kind::Resistor { ohms : 10.0 }, ( 0, 2 ), ( 0, 0 ) ),
	], &History::default(), STEP ).unwrap();

	assert_eq!( solution.readings[ 0 ], Reading { from : 0.0, to : 3.0, current : solution.readings[ 0 ].current } );
	assert!( near( solution.readings[ 0 ].current, 0.3 ) );
}

#[test]
fn it_leaves_a_resistor_without_a_source_at_rest()
{
	let solution = solve( &[ element( Kind::Resistor { ohms : 10.0 }, ( 0, 0 ), ( 0, 2 ) ) ], &History::default(), STEP ).unwrap();

	assert_eq!( solution.readings[ 0 ], Reading { from : 0.0, to : 0.0, current : 0.0 } );
}

#[test]
fn it_rejects_a_shorted_battery()
{
	let elements = [
		element( Kind::Battery { volts : 5.0 }, ( 0, 0 ), ( 0, 2 ) ),
		element( Kind::Wire, ( 0, 0 ), ( 2, 0 ) ),
		element( Kind::Wire, ( 2, 0 ), ( 2, 2 ) ),
		element( Kind::Wire, ( 2, 2 ), ( 0, 2 ) ),
	];

	assert_eq!( solve( &elements, &History::default(), STEP ), Err( Error::Singular ) );
}

#[test]
fn it_rejects_two_batteries_in_parallel()
{
	let elements = [
		element( Kind::Battery { volts : 5.0 }, ( 0, 0 ), ( 0, 2 ) ),
		element( Kind::Battery { volts : 9.0 }, ( 0, 0 ), ( 0, 2 ) ),
	];

	assert_eq!( solve( &elements, &History::default(), STEP ), Err( Error::Singular ) );
}

#[test]
fn it_charges_an_empty_capacitor_like_a_conductance()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 2 ) ),
		element( Kind::Capacitor { farads : 1e-6 }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	assert!( near( solution.readings[ 5 ].from, 2.5 ) );
	assert!( near( solution.readings[ 5 ].current, 0.0025 ) );
}

#[test]
fn it_discharges_a_capacitor_from_its_stored_voltage()
{
	let elements = [
		Element { id : 1, ..element( Kind::Capacitor { farads : 1e-6 }, ( 0, 0 ), ( 0, 2 ) ) },
		Element { id : 2, ..element( Kind::Resistor { ohms : 1000.0 }, ( 0, 2 ), ( 0, 0 ) ) },
	];

	let charged = Solution { readings : vec![ Reading { from : 2.0, to : 0.0, current : 0.0 }, Reading { from : 0.0, to : 2.0, current : 0.0 } ], branches : vec![] };

	let solution = solve( &elements, &History::record( &elements, &charged ), STEP ).unwrap();

	assert!( near( solution.readings[ 0 ].from - solution.readings[ 0 ].to, 1.0 ) );
	assert!( near( solution.readings[ 0 ].current, -0.001 ) );
	assert!( near( solution.readings[ 1 ].current, -0.001 ) );
}

#[test]
fn it_lets_current_rise_slowly_in_an_inductor()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 2 ) ),
		element( Kind::Inductor { henries : 1.0 }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	assert!( near( solution.readings[ 5 ].from, 2.5 ) );
	assert!( near( solution.readings[ 5 ].current, 0.0025 ) );
}

#[test]
fn it_keeps_the_stored_current_of_an_inductor()
{
	let elements = [
		Element { id : 1, ..element( Kind::Inductor { henries : 1.0 }, ( 0, 0 ), ( 0, 2 ) ) },
		Element { id : 2, ..element( Kind::Resistor { ohms : 1000.0 }, ( 0, 2 ), ( 0, 0 ) ) },
	];

	let flowing = Solution { readings : vec![ Reading { from : 0.0, to : 0.0, current : 0.01 }, Reading { from : 0.0, to : 0.0, current : 0.0 } ], branches : vec![] };

	let solution = solve( &elements, &History::record( &elements, &flowing ), STEP ).unwrap();

	assert!( near( solution.readings[ 0 ].current, 0.005 ) );
	assert!( near( solution.readings[ 1 ].current, 0.005 ) );
	assert!( near( solution.readings[ 0 ].to, 5.0 ) );
}

#[test]
fn it_stops_the_current_at_an_open_switch()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 2 ) ),
		element( Kind::Switch { closed : false }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	assert_eq!( solution.readings[ 4 ].current, 0.0 );
	assert_eq!( solution.readings[ 5 ], Reading { from : 5.0, to : 0.0, current : 0.0 } );
}

#[test]
fn it_carries_the_current_through_a_closed_switch()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 2 ) ),
		element( Kind::Switch { closed : true }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	assert!( near( solution.readings[ 4 ].current, 0.005 ) );
	assert!( near( solution.readings[ 5 ].current.abs(), 0.005 ) );
	assert_eq!( ( solution.readings[ 5 ].from, solution.readings[ 5 ].to ), ( 0.0, 0.0 ) );
}

#[test]
fn it_rejects_a_battery_shorted_by_a_closed_switch()
{
	let elements = [
		element( Kind::Battery { volts : 5.0 }, ( 0, 0 ), ( 0, 2 ) ),
		element( Kind::Switch { closed : true }, ( 0, 2 ), ( 0, 0 ) ),
	];

	assert_eq!( solve( &elements, &History::default(), STEP ), Err( Error::Singular ) );
}
