use simulator::{ Element, History, Kind, Point, Reading, Segment, Solution };


fn element( id : u32, kind : Kind ) -> Element
{
	Element { id, kind, segment : Segment { from : Point { x : 0, y : 0 }, to : Point { x : 2, y : 0 } } }
}


#[test]
fn it_records_the_voltage_of_a_capacitor_and_a_diode_and_the_current_of_an_inductor()
{
	let elements = [ element( 3, Kind::Capacitor { farads : 1e-6 } ), element( 5, Kind::Inductor { henries : 1.0 } ), element( 7, Kind::Resistor { ohms : 1.0 } ), element( 9, Kind::Diode ) ];

	let solution = Solution { readings : vec![
		Reading { potentials : vec![ 5.0, 2.0 ], currents : vec![ 0.1, -0.1 ] },
		Reading { potentials : vec![ 1.0, 0.0 ], currents : vec![ 0.2, -0.2 ] },
		Reading { potentials : vec![ 1.0, 0.0 ], currents : vec![ 1.0, -1.0 ] },
		Reading { potentials : vec![ 0.7, 0.0 ], currents : vec![ 0.004, -0.004 ] },
	], branches : vec![] };

	let history = History::record( &elements, &solution );

	assert_eq!( history.get( 3 ), [ 3.0, 0.0 ] );
	assert_eq!( history.get( 5 ), [ 0.2, 0.0 ] );
	assert_eq!( history.get( 7 ), [ 0.0, 0.0 ] );
	assert_eq!( history.get( 9 ), [ 0.7, 0.0 ] );
}

#[test]
fn it_starts_every_part_at_rest()
{
	assert_eq!( History::default().get( 0 ), [ 0.0, 0.0 ] );
}

#[test]
fn it_records_both_junction_voltages_of_a_transistor()
{
	let elements = [ element( 3, Kind::Npn ), element( 5, Kind::Pnp ) ];

	let solution = Solution { readings : vec![
		Reading { potentials : vec![ 0.7, 3.0, 0.0 ], currents : vec![ 0.0; 3 ] },
		Reading { potentials : vec![ 4.3, 2.0, 5.0 ], currents : vec![ 0.0; 3 ] },
	], branches : vec![] };

	let history = History::record( &elements, &solution );

	assert!( ( history.get( 3 )[ 0 ] - 0.7 ).abs() < 1e-12 && ( history.get( 3 )[ 1 ] + 2.3 ).abs() < 1e-12 );
	assert!( ( history.get( 5 )[ 0 ] - 0.7 ).abs() < 1e-12 && ( history.get( 5 )[ 1 ] + 2.3 ).abs() < 1e-12 );
}

#[test]
fn it_records_the_gate_and_the_drain_voltages_of_a_mosfet()
{
	let elements = [ element( 3, Kind::Nmos ), element( 5, Kind::Pmos ) ];

	let solution = Solution { readings : vec![
		Reading { potentials : vec![ 2.0, 3.0, 0.5 ], currents : vec![ 0.0; 3 ] },
		Reading { potentials : vec![ 3.0, 2.0, 5.0 ], currents : vec![ 0.0; 3 ] },
	], branches : vec![] };

	let history = History::record( &elements, &solution );

	assert_eq!( history.get( 3 ), [ 1.5, 2.5 ] );
	assert_eq!( history.get( 5 ), [ 2.0, 3.0 ] );
}
