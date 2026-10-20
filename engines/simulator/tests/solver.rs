use simulator::{ solve, Element, Error, History, Kind, Point, Reading, Segment, Solution };


const STEP : f64 = 1e-3;


fn element( kind : Kind, from : ( i32, i32 ), to : ( i32, i32 ) ) -> Element
{
	Element { id : 0, kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }, mirrored : false }
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

	let battery = &solution.readings[ 0 ];
	let resistor = &solution.readings[ 4 ];

	assert_eq!( ( battery.potentials[ 0 ], battery.potentials[ 1 ] ), ( 0.0, 5.0 ) );
	assert!( near( battery.currents[ 0 ], 0.005 ) );
	assert_eq!( ( resistor.potentials[ 0 ], resistor.potentials[ 1 ] ), ( 5.0, 0.0 ) );
	assert!( near( resistor.currents[ 0 ], 0.005 ) );
}

#[test]
fn it_divides_a_voltage_between_resistors_in_series()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 2 ) ),
		element( Kind::Resistor { ohms : 3000.0 }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	assert!( near( solution.readings[ 4 ].potentials[ 1 ], 3.75 ) );
	assert!( near( solution.readings[ 5 ].currents[ 0 ], 0.00125 ) );
}

#[test]
fn it_splits_a_current_between_resistors_in_parallel()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 4 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 2, 0 ), ( 2, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	assert!( near( solution.readings[ 0 ].currents[ 0 ], 0.01 ) );
	assert!( near( solution.readings[ 4 ].currents[ 0 ], 0.005 ) );
	assert!( near( solution.readings[ 5 ].currents[ 0 ], 0.005 ) );
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

	assert_eq!( solution.readings[ 1 ].potentials[ 0 ], 5.0 );
	assert_eq!( solution.readings[ 2 ].potentials[ 0 ], 0.0 );
	assert!( solution.readings[ 1 ].currents[ 0 ].is_nan() );
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

	assert!( near( solution.readings[ 1 ].currents[ 0 ], 0.02 ) );
	assert!( near( solution.readings[ 2 ].currents[ 0 ], -0.02 ) );
	assert!( near( solution.readings[ 3 ].currents[ 0 ], 0.02 ) );
}

#[test]
fn it_references_a_floating_circuit_to_one_of_its_nets()
{
	let solution = solve( &[
		element( Kind::Battery { volts : 3.0 }, ( 0, 0 ), ( 0, 2 ) ),
		element( Kind::Resistor { ohms : 10.0 }, ( 0, 2 ), ( 0, 0 ) ),
	], &History::default(), STEP ).unwrap();

	assert_eq!( solution.readings[ 0 ], Reading { potentials : vec![ 0.0, 3.0 ], currents : solution.readings[ 0 ].currents.clone() } );
	assert!( near( solution.readings[ 0 ].currents[ 0 ], 0.3 ) );
}

#[test]
fn it_leaves_a_resistor_without_a_source_at_rest()
{
	let solution = solve( &[ element( Kind::Resistor { ohms : 10.0 }, ( 0, 0 ), ( 0, 2 ) ) ], &History::default(), STEP ).unwrap();

	assert_eq!( solution.readings[ 0 ], Reading { potentials : vec![ 0.0, 0.0 ], currents : vec![ 0.0, 0.0 ] } );
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

	assert!( near( solution.readings[ 5 ].potentials[ 0 ], 2.5 ) );
	assert!( near( solution.readings[ 5 ].currents[ 0 ], 0.0025 ) );
}

#[test]
fn it_discharges_a_capacitor_from_its_stored_voltage()
{
	let elements = [
		Element { id : 1, ..element( Kind::Capacitor { farads : 1e-6 }, ( 0, 0 ), ( 0, 2 ) ) },
		Element { id : 2, ..element( Kind::Resistor { ohms : 1000.0 }, ( 0, 2 ), ( 0, 0 ) ) },
	];

	let charged = Solution { readings : vec![ Reading { potentials : vec![ 2.0, 0.0 ], currents : vec![ 0.0, 0.0 ] }, Reading { potentials : vec![ 0.0, 2.0 ], currents : vec![ 0.0, 0.0 ] } ], branches : vec![] };

	let solution = solve( &elements, &History::record( &elements, &charged ), STEP ).unwrap();

	assert!( near( solution.readings[ 0 ].potentials[ 0 ] - solution.readings[ 0 ].potentials[ 1 ], 1.0 ) );
	assert!( near( solution.readings[ 0 ].currents[ 0 ], -0.001 ) );
	assert!( near( solution.readings[ 1 ].currents[ 0 ], -0.001 ) );
}

#[test]
fn it_lets_current_rise_slowly_in_an_inductor()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 2 ) ),
		element( Kind::Inductor { henries : 1.0 }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	assert!( near( solution.readings[ 5 ].potentials[ 0 ], 2.5 ) );
	assert!( near( solution.readings[ 5 ].currents[ 0 ], 0.0025 ) );
}

#[test]
fn it_keeps_the_stored_current_of_an_inductor()
{
	let elements = [
		Element { id : 1, ..element( Kind::Inductor { henries : 1.0 }, ( 0, 0 ), ( 0, 2 ) ) },
		Element { id : 2, ..element( Kind::Resistor { ohms : 1000.0 }, ( 0, 2 ), ( 0, 0 ) ) },
	];

	let flowing = Solution { readings : vec![ Reading { potentials : vec![ 0.0, 0.0 ], currents : vec![ 0.01, -0.01 ] }, Reading { potentials : vec![ 0.0, 0.0 ], currents : vec![ 0.0, 0.0 ] } ], branches : vec![] };

	let solution = solve( &elements, &History::record( &elements, &flowing ), STEP ).unwrap();

	assert!( near( solution.readings[ 0 ].currents[ 0 ], 0.005 ) );
	assert!( near( solution.readings[ 1 ].currents[ 0 ], 0.005 ) );
	assert!( near( solution.readings[ 0 ].potentials[ 1 ], 5.0 ) );
}

#[test]
fn it_stops_the_current_at_an_open_switch()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 2 ) ),
		element( Kind::Switch { closed : false }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	assert_eq!( solution.readings[ 4 ].currents[ 0 ], 0.0 );
	assert_eq!( solution.readings[ 5 ], Reading { potentials : vec![ 5.0, 0.0 ], currents : vec![ 0.0, 0.0 ] } );
}

#[test]
fn it_carries_the_current_through_a_closed_switch()
{
	let solution = solve( &loop_with( &[
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 0 ), ( 4, 2 ) ),
		element( Kind::Switch { closed : true }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	assert!( near( solution.readings[ 4 ].currents[ 0 ], 0.005 ) );
	assert!( near( solution.readings[ 5 ].currents[ 0 ].abs(), 0.005 ) );
	assert_eq!( ( solution.readings[ 5 ].potentials[ 0 ], solution.readings[ 5 ].potentials[ 1 ] ), ( 0.0, 0.0 ) );
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

#[test]
fn it_drops_about_0_65_v_across_a_diode_in_the_forward_direction()
{
	let solution = solve( &loop_with( &[
		element( Kind::Diode, ( 4, 0 ), ( 4, 2 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	let diode = &solution.readings[ 4 ];
	let drop = diode.potentials[ 0 ] - diode.potentials[ 1 ];

	assert!( drop > 0.6 && drop < 0.7 );
	assert!( ( diode.currents[ 0 ] - ( 5.0 - drop ) / 1000.0 ).abs() < 1e-9 );
	assert!( ( diode.currents[ 0 ] - 1e-14 * ( ( drop / 0.025852 ).exp() - 1.0 ) ).abs() < 1e-7 );
}

#[test]
fn it_blocks_the_current_through_a_diode_in_the_reverse_direction()
{
	let solution = solve( &loop_with( &[
		element( Kind::Diode, ( 4, 2 ), ( 4, 0 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	let diode = &solution.readings[ 4 ];

	assert!( diode.currents[ 0 ].abs() < 1e-9 );
	assert!( ( diode.potentials[ 1 ] - diode.potentials[ 0 ] - 5.0 ).abs() < 1e-6 );
}

#[test]
fn it_carries_the_current_through_two_diodes_in_series()
{
	let solution = solve( &loop_with( &[
		element( Kind::Diode, ( 4, 0 ), ( 4, 1 ) ),
		element( Kind::Diode, ( 4, 1 ), ( 4, 2 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 2 ), ( 4, 4 ) ),
	] ), &History::default(), STEP ).unwrap();

	let drop = solution.readings[ 4 ].potentials[ 0 ] - solution.readings[ 5 ].potentials[ 1 ];

	assert!( drop > 1.2 && drop < 1.4 );
	assert!( ( solution.readings[ 6 ].currents[ 0 ] - ( 5.0 - drop ) / 1000.0 ).abs() < 1e-9 );
}

#[test]
fn it_gives_up_on_diodes_that_do_not_settle()
{
	let elements = loop_with( &[
		Element { id : 1, ..element( Kind::Diode, ( 4, 0 ), ( 4, 2 ) ) },
		element( Kind::Resistor { ohms : 1000.0 }, ( 4, 2 ), ( 4, 4 ) ),
	] );

	let broken = Solution { readings : vec![ Reading { potentials : vec![ f64::NAN, 0.0 ], currents : vec![ 0.0, 0.0 ] }; 6 ], branches : vec![] };

	assert_eq!( solve( &elements, &History::record( &elements, &broken ), STEP ), Err( Error::Unsettled ) );
}

fn amplifier( kind : Kind, volts : f64, base : f64 ) -> Vec<Element>
{
	vec![
		element( Kind::Battery { volts }, ( 0, 6 ), ( 0, 0 ) ),
		element( Kind::Wire, ( 0, 0 ), ( 8, 0 ) ),
		element( Kind::Resistor { ohms : base }, ( 4, 0 ), ( 4, 3 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 7, 0 ), ( 7, 2 ) ),
		element( kind, ( 4, 3 ), ( 6, 3 ) ),
		element( Kind::Wire, ( 7, 4 ), ( 7, 6 ) ),
		element( Kind::Wire, ( 0, 6 ), ( 7, 6 ) ),
		element( Kind::Ground, ( 0, 6 ), ( 0, 6 ) ),
	]
}

#[test]
fn it_amplifies_the_base_current_of_an_npn_transistor_by_its_gain()
{
	let solution = solve( &amplifier( Kind::Npn, 5.0, 200_000.0 ), &History::default(), STEP ).unwrap();
	let transistor = &solution.readings[ 4 ];
	let [ base, collector, emitter ] = transistor.currents[ .. ] else { panic!() };

	assert!( ( base - ( 5.0 - transistor.potentials[ 0 ] ) / 200_000.0 ).abs() < 1e-12 );
	assert!( transistor.potentials[ 0 ] > 0.6 && transistor.potentials[ 0 ] < 0.7 );
	assert!( ( collector / base - 100.0 ).abs() < 0.01 );
	assert!( ( base + collector + emitter ).abs() < 1e-12 );
	assert!( transistor.potentials[ 1 ] > 2.5 && transistor.potentials[ 1 ] < 3.0 );
}

#[test]
fn it_saturates_an_npn_transistor_with_a_large_base_current()
{
	let solution = solve( &amplifier( Kind::Npn, 5.0, 10_000.0 ), &History::default(), STEP ).unwrap();
	let transistor = &solution.readings[ 4 ];

	assert!( transistor.potentials[ 1 ] < 0.2 );
	assert!( transistor.currents[ 1 ] > 0.0048 && transistor.currents[ 1 ] < 0.005 );
	assert!( transistor.currents[ 1 ] / transistor.currents[ 0 ] < 20.0 );
}

#[test]
fn it_cuts_off_an_npn_transistor_without_base_current()
{
	let mut elements = amplifier( Kind::Npn, 5.0, 200_000.0 );

	elements.remove( 2 );

	let solution = solve( &elements, &History::default(), STEP ).unwrap();

	assert!( solution.readings[ 3 ].currents[ 1 ].abs() < 1e-9 );
	assert!( ( solution.readings[ 3 ].potentials[ 1 ] - 5.0 ).abs() < 1e-6 );
}

#[test]
fn it_mirrors_the_currents_of_a_pnp_transistor()
{
	let npn = solve( &amplifier( Kind::Npn, 5.0, 200_000.0 ), &History::default(), STEP ).unwrap();
	let pnp = solve( &amplifier( Kind::Pnp, -5.0, 200_000.0 ), &History::default(), STEP ).unwrap();

	for terminal in 0 .. 3
	{
		assert!( ( npn.readings[ 4 ].currents[ terminal ] + pnp.readings[ 4 ].currents[ terminal ] ).abs() < 1e-12 );
		assert!( ( npn.readings[ 4 ].potentials[ terminal ] + pnp.readings[ 4 ].potentials[ terminal ] ).abs() < 1e-9 );
	}
}

#[test]
fn it_carries_the_transistor_currents_through_the_wires()
{
	let solution = solve( &amplifier( Kind::Npn, 5.0, 200_000.0 ), &History::default(), STEP ).unwrap();
	let emitter = solution.readings[ 4 ].currents[ 2 ];
	let branch = solution.branch_at( Point { x : 7, y : 5 } ).unwrap();

	assert!( ( solution.branches[ branch ].current.abs() - emitter.abs() ).abs() < 1e-12 );
	assert!( ( solution.readings[ 0 ].currents[ 0 ] + emitter ).abs() < 1e-12 );
}

fn switch( kind : Kind, gate : f64 ) -> Vec<Element>
{
	vec![
		element( Kind::Battery { volts : 5.0 }, ( 0, 6 ), ( 0, 0 ) ),
		element( Kind::Wire, ( 0, 0 ), ( 8, 0 ) ),
		element( Kind::Battery { volts : gate }, ( 4, 6 ), ( 4, 3 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 7, 0 ), ( 7, 2 ) ),
		element( kind, ( 4, 3 ), ( 6, 3 ) ),
		element( Kind::Wire, ( 7, 4 ), ( 7, 6 ) ),
		element( Kind::Wire, ( 0, 6 ), ( 7, 6 ) ),
		element( Kind::Ground, ( 0, 6 ), ( 0, 6 ) ),
	]
}

fn inverter( input : f64 ) -> Vec<Element>
{
	vec![
		element( Kind::Battery { volts : 5.0 }, ( 0, 10 ), ( 0, 0 ) ),
		element( Kind::Wire, ( 0, 0 ), ( 7, 0 ) ),
		element( Kind::Wire, ( 7, 0 ), ( 7, 2 ) ),
		element( Kind::Pmos, ( 10, 3 ), ( 8, 3 ) ),
		element( Kind::Wire, ( 7, 4 ), ( 7, 6 ) ),
		element( Kind::Nmos, ( 4, 7 ), ( 6, 7 ) ),
		element( Kind::Wire, ( 7, 8 ), ( 7, 10 ) ),
		element( Kind::Wire, ( 0, 10 ), ( 12, 10 ) ),
		element( Kind::Ground, ( 0, 10 ), ( 0, 10 ) ),
		element( Kind::Battery { volts : input }, ( 12, 10 ), ( 12, 1 ) ),
		element( Kind::Wire, ( 12, 1 ), ( 2, 1 ) ),
		element( Kind::Wire, ( 10, 1 ), ( 10, 3 ) ),
		element( Kind::Wire, ( 2, 1 ), ( 2, 7 ) ),
		element( Kind::Wire, ( 2, 7 ), ( 4, 7 ) ),
	]
}

#[test]
fn it_follows_the_square_law_in_a_saturated_nmos_transistor()
{
	let solution = solve( &switch( Kind::Nmos, 2.0 ), &History::default(), STEP ).unwrap();
	let transistor = &solution.readings[ 4 ];
	let [ gate, drain, source ] = transistor.currents[ .. ] else { panic!() };

	assert!( ( drain - 0.02 / 2.0 * 0.5_f64.powi( 2 ) ).abs() < 1e-9 );
	assert!( ( transistor.potentials[ 1 ] - 2.5 ).abs() < 1e-6 );
	assert!( gate.abs() < 1e-9 );
	assert!( ( gate + drain + source ).abs() < 1e-15 );
}

#[test]
fn it_turns_off_an_nmos_transistor_below_its_threshold()
{
	let solution = solve( &switch( Kind::Nmos, 1.0 ), &History::default(), STEP ).unwrap();

	assert!( solution.readings[ 4 ].currents[ 1 ].abs() < 1e-9 );
	assert!( ( solution.readings[ 4 ].potentials[ 1 ] - 5.0 ).abs() < 1e-6 );
}

#[test]
fn it_solves_a_mosfet_with_a_floating_gate()
{
	let mut elements = switch( Kind::Nmos, 5.0 );

	elements.remove( 2 );

	let solution = solve( &elements, &History::default(), STEP ).unwrap();

	assert!( solution.readings[ 3 ].currents[ 1 ].abs() < 1e-9 );
}

#[test]
fn it_inverts_the_input_of_a_cmos_inverter()
{
	for ( input, output ) in [ ( 0.0, 5.0 ), ( 5.0, 0.0 ) ]
	{
		let solution = solve( &inverter( input ), &History::default(), STEP ).unwrap();

		assert!( ( solution.readings[ 5 ].potentials[ 1 ] - output ).abs() < 1e-6 );
		assert!( ( solution.readings[ 3 ].potentials[ 1 ] - output ).abs() < 1e-6 );
		assert!( solution.readings[ 5 ].currents[ 1 ].abs() < 1e-9 );
	}
}

#[test]
fn it_turns_on_both_halves_of_a_cmos_inverter_at_half_the_supply()
{
	let solution = solve( &inverter( 2.5 ), &History::default(), STEP ).unwrap();
	let current = 0.02 / 2.0 * 1.0_f64.powi( 2 );

	assert!( ( solution.readings[ 5 ].currents[ 1 ] - current ).abs() < 1e-6 );
	assert!( ( solution.readings[ 3 ].currents[ 1 ] + current ).abs() < 1e-6 );
}

#[test]
fn it_feeds_a_circuit_from_a_vcc_back_to_the_ground()
{
	let solution = solve( &[
		element( Kind::Supply { volts : 5.0 }, ( 0, 0 ), ( 0, 0 ) ),
		element( Kind::Wire, ( 0, 0 ), ( 2, 0 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 2, 0 ), ( 2, 4 ) ),
		element( Kind::Wire, ( 2, 4 ), ( 4, 4 ) ),
		element( Kind::Ground, ( 4, 4 ), ( 4, 4 ) ),
	], &History::default(), STEP ).unwrap();

	assert_eq!( solution.readings[ 0 ].potentials, vec![ 0.0, 5.0 ] );
	assert!( near( solution.readings[ 0 ].currents[ 0 ], 0.005 ) && near( solution.readings[ 0 ].currents[ 1 ], -0.005 ) );
	assert!( near( solution.readings[ 2 ].currents[ 0 ], 0.005 ) );
	assert!( near( solution.readings[ 4 ].currents[ 0 ], 0.005 ) );
	assert!( solution.branches.iter().all( | branch | near( branch.current.abs(), 0.005 ) ) );
}

#[test]
fn it_holds_a_vcc_without_a_ground_at_its_voltage()
{
	let solution = solve( &[
		element( Kind::Supply { volts : 5.0 }, ( 0, 0 ), ( 0, 0 ) ),
		element( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) ),
	], &History::default(), STEP ).unwrap();

	assert_eq!( solution.readings[ 1 ].potentials, vec![ 5.0, 5.0 ] );
	assert_eq!( solution.readings[ 0 ].currents, vec![ 0.0, -0.0 ] );
}

#[test]
fn it_solves_a_circuit_next_to_a_lone_transistor()
{
	for kind in [ Kind::Npn, Kind::Pnp, Kind::Nmos, Kind::Pmos ]
	{
		let solution = solve( &[
			element( Kind::Battery { volts : 5.0 }, ( 0, 4 ), ( 0, 0 ) ),
			element( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 0, 4 ) ),
			element( Kind::Ground, ( 0, 4 ), ( 0, 4 ) ),
			element( kind, ( 10, 0 ), ( 12, 0 ) ),
		], &History::default(), STEP ).unwrap();

		assert!( near( solution.readings[ 1 ].currents[ 0 ], 0.005 ) );
		assert!( solution.readings[ 3 ].currents.iter().all( | current | current.abs() < 1e-9 ) );
	}
}
