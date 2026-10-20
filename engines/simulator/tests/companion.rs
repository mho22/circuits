use simulator::{ companion, linearize, Element, History, Kind, Point, Reading, Segment, Solution, Stamp };


const STEP : f64 = 1e-3;
const THROUGH : [ f64; 3 ] = [ 1.0, -1.0, 0.0 ];


fn element( kind : Kind ) -> Element
{
	Element { id : 1, kind, segment : Segment { from : Point { x : 0, y : 0 }, to : Point { x : 2, y : 0 } } }
}

fn stored( kind : Kind, reading : Reading ) -> History
{
	History::record( &[ element( kind ) ], &Solution { readings : vec![ reading ], branches : vec![] } )
}


#[test]
fn it_turns_a_resistor_into_a_conductance()
{
	assert_eq!( companion( &element( Kind::Resistor { ohms : 200.0 } ), &History::default(), STEP, [ 0.0; 2 ] ), vec![ Stamp { across : ( 0, 1 ), conductance : 0.005, source : 0.0, weights : THROUGH } ] );
}

#[test]
fn it_turns_a_capacitor_into_a_conductance_and_its_stored_voltage()
{
	let kind = Kind::Capacitor { farads : 1e-6 };
	let history = stored( kind, Reading { potentials : vec![ 3.0, 1.0 ], currents : vec![ 0.0, 0.0 ] } );

	assert_eq!( companion( &element( kind ), &history, STEP, [ 0.0; 2 ] ), vec![ Stamp { across : ( 0, 1 ), conductance : 1e-3, source : 2e-3, weights : THROUGH } ] );
}

#[test]
fn it_turns_an_inductor_into_a_conductance_and_its_stored_current()
{
	let kind = Kind::Inductor { henries : 2.0 };
	let history = stored( kind, Reading { potentials : vec![ 0.0, 0.0 ], currents : vec![ 0.5, -0.5 ] } );

	assert_eq!( companion( &element( kind ), &history, STEP, [ 0.0; 2 ] ), vec![ Stamp { across : ( 0, 1 ), conductance : 5e-4, source : -0.5, weights : THROUGH } ] );
}

#[test]
fn it_turns_a_diode_into_its_junction_at_the_guess()
{
	let ( conductance, source ) = linearize( 0.6 );

	assert_eq!( companion( &element( Kind::Diode ), &History::default(), STEP, [ 0.6, 0.0 ] ), vec![ Stamp { across : ( 0, 1 ), conductance, source, weights : THROUGH } ] );
}

#[test]
fn it_turns_an_npn_transistor_into_two_junctions_with_gain()
{
	let stamps = companion( &element( Kind::Npn ), &History::default(), STEP, [ 0.65, -2.0 ] );

	assert_eq!( stamps.iter().map( | stamp | stamp.across ).collect::<Vec<_>>(), vec![ ( 0, 2 ), ( 0, 1 ) ] );
	assert_eq!( ( stamps[ 0 ].conductance, stamps[ 0 ].source ), linearize( 0.65 ) );
	assert_eq!( ( stamps[ 1 ].conductance, stamps[ 1 ].source ), linearize( -2.0 ) );
	assert_eq!( stamps[ 0 ].weights, [ 0.01, 1.0, -1.01 ] );
	assert_eq!( stamps[ 1 ].weights, [ 1.0, -2.0, 1.0 ] );
}

#[test]
fn it_turns_a_pnp_transistor_into_two_reversed_junctions()
{
	let stamps = companion( &element( Kind::Pnp ), &History::default(), STEP, [ 0.65, -2.0 ] );

	assert_eq!( stamps.iter().map( | stamp | stamp.across ).collect::<Vec<_>>(), vec![ ( 2, 0 ), ( 1, 0 ) ] );
	assert_eq!( stamps[ 0 ].weights, [ -0.01, -1.0, 1.01 ] );
	assert_eq!( stamps[ 1 ].weights, [ -1.0, 2.0, -1.0 ] );
}

#[test]
fn it_keeps_the_weights_of_every_junction_balanced()
{
	for kind in [ Kind::Diode, Kind::Npn, Kind::Pnp ]
	{
		for stamp in companion( &element( kind ), &History::default(), STEP, [ 0.0; 2 ] ) { assert!( stamp.weights.iter().sum::<f64>().abs() < 1e-12 ); }
	}
}

#[test]
fn it_gives_no_stamp_to_a_part_the_solver_handles_apart()
{
	for kind in [ Kind::Wire, Kind::Ground, Kind::Battery { volts : 5.0 }, Kind::Switch { closed : true } ]
	{
		assert!( companion( &element( kind ), &History::default(), STEP, [ 0.0; 2 ] ).is_empty() );
	}
}
