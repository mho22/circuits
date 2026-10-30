use simulator::{ Device, Mode, Plan };


fn device() -> Device
{
	Device { layer : 1, inputs : Vec::new(), outputs : Vec::new(), scopes : Vec::new() }
}


#[test]
fn it_names_a_circuit_by_the_engine_that_runs_it()
{
	let analog = Plan { analog : vec![ 0 ], ..Plan::default() };

	assert_eq!( Mode::of( &[], &analog ), Mode::Analog );
	assert_eq!( Mode::of( &[ device() ], &analog ), Mode::Mixed );
	assert_eq!( Mode::of( &[ device() ], &Plan::default() ), Mode::Digital );
	assert_eq!( [ Mode::Analog, Mode::Mixed, Mode::Digital ].map( | mode | mode.to_string() ), [ "Analog", "Mixed", "Digital" ] );
}
