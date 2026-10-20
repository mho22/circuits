use simulator::{ companion, linearize, Element, History, Kind, Point, Reading, Segment, Solution };


const STEP : f64 = 1e-3;


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
	assert_eq!( companion( &element( Kind::Resistor { ohms : 200.0 } ), &History::default(), STEP, 0.0 ), Some( ( 0.005, 0.0 ) ) );
}

#[test]
fn it_turns_a_capacitor_into_a_conductance_and_its_stored_voltage()
{
	let kind = Kind::Capacitor { farads : 1e-6 };
	let history = stored( kind, Reading { from : 3.0, to : 1.0, current : 0.0 } );

	assert_eq!( companion( &element( kind ), &history, STEP, 0.0 ), Some( ( 1e-3, 2e-3 ) ) );
}

#[test]
fn it_turns_an_inductor_into_a_conductance_and_its_stored_current()
{
	let kind = Kind::Inductor { henries : 2.0 };
	let history = stored( kind, Reading { from : 0.0, to : 0.0, current : 0.5 } );

	assert_eq!( companion( &element( kind ), &history, STEP, 0.0 ), Some( ( 5e-4, -0.5 ) ) );
}

#[test]
fn it_turns_a_diode_into_its_junction_at_the_guess()
{
	assert_eq!( companion( &element( Kind::Diode ), &History::default(), STEP, 0.6 ), Some( linearize( 0.6 ) ) );
}

#[test]
fn it_gives_no_conductance_to_a_part_the_solver_handles_apart()
{
	for kind in [ Kind::Wire, Kind::Ground, Kind::Battery { volts : 5.0 }, Kind::Switch { closed : true } ]
	{
		assert_eq!( companion( &element( kind ), &History::default(), STEP, 0.0 ), None );
	}
}
