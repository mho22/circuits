use simulator::{ advance, characterize, flatten, settle, Device, Document, Element, Error, History, Kind, Logic, Model, Operation, Plan, Point, Registry, Segment, Solution };


const STEP : f64 = 1e-6;


type Part = ( Kind, ( i32, i32 ), ( i32, i32 ) );


struct Circuit
{
	elements : Vec<Element>,
	devices : Vec<Device>,
	plan : Plan,
	models : Vec<Option<Model>>,
	logic : Logic,
}

impl Circuit
{
	fn settle( &mut self ) -> Result<Solution, Error>
	{
		settle( &self.elements, &self.devices, &self.plan, &self.models, &History::default(), STEP, &mut self.logic )
	}

	fn advance( &mut self, step : f64 ) -> Result<Solution, Error>
	{
		advance( &self.elements, &self.devices, &self.plan, &self.models, &History::default(), step, &mut self.logic )
	}

	fn at( &self, solution : &Solution, point : ( i32, i32 ) ) -> f64
	{
		let point = Point { x : point.0, y : point.1 };
		let index = self.elements.iter().position( | element | element.kind == Kind::Wire && element.segment.contains( point ) ).unwrap();

		solution.readings[ index ].potentials[ 0 ]
	}

	fn rises( &mut self, point : ( i32, i32 ), step : f64, steps : usize ) -> usize
	{
		let solution = self.settle().unwrap();
		let mut high = self.at( &solution, point ) > 2.5;
		let mut rises = 0;

		for _ in 0 .. steps
		{
			let solution = self.advance( step ).unwrap();
			let now = self.at( &solution, point ) > 2.5;

			if now && ! high { rises += 1; }

			high = now;
		}

		rises
	}
}


fn layer( parts : &[ Part ] ) -> Document
{
	let mut document = Document::default();

	for &( kind, from, to ) in parts { document.apply( Operation::Add { kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }, mirrored : false } ).unwrap(); }

	document
}

fn inverter() -> Document
{
	layer( &[
		( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		( Kind::Output, ( 6, 0 ), ( 5, 0 ) ),
		( Kind::Supply { volts : 5.0 }, ( 3, -1 ), ( 3, -1 ) ),
		( Kind::Wire, ( 0, -2 ), ( 0, 2 ) ),
		( Kind::Pmos, ( 0, -2 ), ( 2, -2 ) ),
		( Kind::Nmos, ( 0, 2 ), ( 2, 2 ) ),
		( Kind::Wire, ( 3, -3 ), ( 5, -3 ) ),
		( Kind::Wire, ( 5, -3 ), ( 5, 1 ) ),
		( Kind::Wire, ( 5, 1 ), ( 3, 1 ) ),
		( Kind::Ground, ( 3, 3 ), ( 3, 3 ) ),
	] )
}

fn symbol( x : i32 ) -> Part
{
	( Kind::Symbol { layer : 1, rotation : 0 }, ( x, 0 ), ( x, 0 ) )
}

fn circuit( parts : &[ Part ] ) -> Circuit
{
	let layers = [ layer( parts ), inverter() ];
	let models = vec![ None, characterize( &layers, &[], 1, STEP ) ];
	let ( elements, devices ) = flatten( &layers, &mut Registry::default(), &models, 0, &[] ).unwrap();
	let plan = Plan::build( &elements, &devices );

	Circuit { elements, devices, plan, models, logic : Logic::default() }
}

fn ring() -> Circuit
{
	circuit( &[
		symbol( 10 ),
		symbol( 14 ),
		symbol( 18 ),
		( Kind::Wire, ( 20, 0 ), ( 20, 3 ) ),
		( Kind::Wire, ( 20, 3 ), ( 8, 3 ) ),
		( Kind::Wire, ( 8, 3 ), ( 8, 0 ) ),
	] )
}


#[test]
fn it_runs_a_chain_of_gates_without_the_analog_solver()
{
	let mut circuit = circuit( &[ symbol( 10 ), symbol( 16 ), symbol( 22 ), ( Kind::Wire, ( 12, 0 ), ( 14, 0 ) ), ( Kind::Wire, ( 18, 0 ), ( 20, 0 ) ), ( Kind::Wire, ( 24, 0 ), ( 26, 0 ) ) ] );
	let solution = circuit.settle().unwrap();

	assert!( ( circuit.at( &solution, ( 13, 0 ) ) - 5.0 ).abs() < 1e-3 );
	assert!( circuit.at( &solution, ( 19, 0 ) ).abs() < 1e-3 );
	assert!( ( circuit.at( &solution, ( 25, 0 ) ) - 5.0 ).abs() < 1e-3 );
	assert!( solution.branches.iter().any( | branch | branch.segment.contains( Point { x : 13, y : 0 } ) && ( branch.voltage - 5.0 ).abs() < 1e-3 ) );
}

#[test]
fn it_reads_an_analog_input_and_drives_an_analog_load()
{
	let mut circuit = circuit( &[
		( Kind::Battery { volts : 5.0 }, ( 6, 4 ), ( 6, 0 ) ),
		( Kind::Ground, ( 6, 4 ), ( 6, 4 ) ),
		( Kind::Wire, ( 6, 0 ), ( 8, 0 ) ),
		symbol( 10 ),
		symbol( 16 ),
		( Kind::Wire, ( 12, 0 ), ( 14, 0 ) ),
		( Kind::Resistor { ohms : 1000.0 }, ( 18, 0 ), ( 18, 4 ) ),
		( Kind::Ground, ( 18, 4 ), ( 18, 4 ) ),
	] );
	let solution = circuit.settle().unwrap();
	let resistor = circuit.elements.iter().position( | element | element.kind == Kind::Resistor { ohms : 1000.0 } ).unwrap();

	assert!( circuit.at( &solution, ( 13, 0 ) ).abs() < 1e-3 );
	assert!( solution.readings[ resistor ].potentials[ 0 ] > 4.9 );
	assert!( solution.readings[ resistor ].currents[ 0 ] > 4.9e-3 );
}

#[test]
fn it_runs_a_ring_of_three_gates_with_a_period_of_six_delays()
{
	assert!( ( 99 ..= 101 ).contains( &ring().rises( ( 20, 1 ), 1e-10, 6000 ) ) );
}

#[test]
fn it_gives_its_delay_to_a_gate_that_feeds_itself_through_a_resistor_and_lets_it_switch_once_a_step()
{
	let mut circuit = circuit( &[
		symbol( 10 ),
		( Kind::Resistor { ohms : 1000.0 }, ( 12, 0 ), ( 12, 3 ) ),
		( Kind::Wire, ( 12, 3 ), ( 8, 3 ) ),
		( Kind::Wire, ( 8, 3 ), ( 8, 0 ) ),
	] );

	circuit.settle().unwrap();

	let levels : Vec<bool> = ( 0 .. 4 ).map( | _ | { let solution = circuit.advance( STEP ).unwrap(); circuit.at( &solution, ( 10, 3 ) ) > 2.5 } ).collect();

	assert!( levels.windows( 2 ).all( | pair | pair[ 0 ] != pair[ 1 ] ) );
}

#[test]
fn it_rejects_a_ring_of_gates_that_switches_too_often_in_one_step()
{
	let mut ring = ring();

	ring.settle().unwrap();

	assert_eq!( ring.advance( STEP ), Err( Error::Oscillating ) );
}

#[test]
fn it_switches_a_gate_one_delay_after_each_edge_of_a_clock()
{
	let mut circuit = circuit( &[ ( Kind::Clock { hertz : 1e8 }, ( 4, 0 ), ( 8, 0 ) ), symbol( 10 ), ( Kind::Wire, ( 12, 0 ), ( 14, 0 ) ) ] );
	let solutions : Vec<Solution> = [ circuit.settle() ].into_iter().chain( ( 0 .. 13 ).map( | _ | circuit.advance( 5e-10 ) ) ).map( Result::unwrap ).collect();
	let levels : Vec<f64> = solutions.iter().map( | solution | circuit.at( solution, ( 13, 0 ) ) ).collect();

	assert!( levels[ .. 12 ].iter().all( | &volts | volts > 2.5 ) );
	assert!( levels[ 13 ] < 2.5 );
	assert!( ( 99 ..= 101 ).contains( &circuit.rises( ( 13, 0 ), 5e-10, 2000 ) ) );
}

#[test]
fn it_ignores_a_pulse_shorter_than_the_delay()
{
	let mut circuit = circuit( &[ ( Kind::Clock { hertz : 1.25e9 }, ( 4, 0 ), ( 8, 0 ) ), symbol( 10 ), ( Kind::Wire, ( 12, 0 ), ( 14, 0 ) ) ] );

	circuit.settle().unwrap();

	assert!( ( 0 .. 400 ).all( | _ | { let solution = circuit.advance( 5e-11 ).unwrap(); circuit.at( &solution, ( 13, 0 ) ) > 2.5 } ) );
}

#[test]
fn it_drives_an_analog_load_with_the_level_of_a_clock()
{
	let mut circuit = circuit( &[ ( Kind::Clock { hertz : 1.0 }, ( 0, -2 ), ( 0, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 0, 4 ) ), ( Kind::Ground, ( 0, 4 ), ( 0, 4 ) ) ] );
	let resistor = circuit.elements.iter().position( | element | element.kind == Kind::Resistor { ohms : 1000.0 } ).unwrap();
	let low = circuit.settle().unwrap().readings[ resistor ].currents[ 0 ];

	circuit.logic.time = 0.75;

	let high = circuit.settle().unwrap().readings[ resistor ].currents[ 0 ];

	assert!( low.abs() < 1e-9 );
	assert!( ( high.abs() - 5e-3 ).abs() < 1e-9 );
}

#[test]
fn it_drives_an_analog_load_with_the_level_of_a_toggle()
{
	let current = | high |
	{
		let mut circuit = circuit( &[ ( Kind::Toggle { high }, ( 0, -2 ), ( 0, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 0, 4 ) ), ( Kind::Ground, ( 0, 4 ), ( 0, 4 ) ) ] );
		let resistor = circuit.elements.iter().position( | element | element.kind == Kind::Resistor { ohms : 1000.0 } ).unwrap();

		circuit.settle().unwrap().readings[ resistor ].currents[ 0 ]
	};

	assert!( current( false ).abs() < 1e-9 );
	assert!( ( current( true ).abs() - 5e-3 ).abs() < 1e-9 );
}

#[test]
fn it_keeps_the_state_of_each_gate_when_it_settles_again()
{
	let mut circuit = circuit( &[ symbol( 10 ), symbol( 14 ), ( Kind::Wire, ( 16, 0 ), ( 16, 3 ) ), ( Kind::Wire, ( 16, 3 ), ( 8, 3 ) ), ( Kind::Wire, ( 8, 3 ), ( 8, 0 ) ) ] );
	let solution = circuit.settle().unwrap();
	let first = circuit.at( &solution, ( 16, 1 ) ) > 2.5;

	for applied in &mut circuit.logic.applied { *applied = 1 - *applied; }

	let solution = circuit.settle().unwrap();

	assert_ne!( circuit.at( &solution, ( 16, 1 ) ) > 2.5, first );
}

#[test]
fn it_starts_a_gate_from_its_first_row_when_its_kept_state_is_outside_its_table()
{
	let mut circuit = circuit( &[ symbol( 10 ), ( Kind::Wire, ( 12, 0 ), ( 14, 0 ) ) ] );

	circuit.settle().unwrap();
	circuit.logic.applied = vec![ 9 ];
	circuit.logic.states = vec![ 9 ];

	let solution = circuit.settle().unwrap();

	assert!( circuit.at( &solution, ( 13, 0 ) ) > 2.5 );
	assert_eq!( circuit.logic.states, vec![ 0 ] );
}
