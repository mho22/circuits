use simulator::{ advance, flatten, pulse, settle, Catalog, Device, Document, Drive, Element, Error, History, Kind, Layout, Logic, Machine, Model, Operation, Plan, Point, Registry, Segment, Solution };


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
		settle( Machine { elements : &self.elements, devices : &self.devices, plan : &self.plan, models : &self.models }, &History::default(), STEP, &mut self.logic, true )
	}

	fn advance( &mut self, step : f64 ) -> Result<Solution, Error>
	{
		advance( Machine { elements : &self.elements, devices : &self.devices, plan : &self.plan, models : &self.models }, &History::default(), step, &mut self.logic )
	}

	fn pulse( &mut self, step : f64, steps : u32 ) -> Result<Solution, Error>
	{
		let machine = Machine { elements : &self.elements, devices : &self.devices, plan : &self.plan, models : &self.models };

		pulse( machine, &Layout::new( machine )?, step, steps, &mut self.logic )
	}

	fn at( &self, solution : &Solution, point : ( i32, i32 ) ) -> f64
	{
		let point = Point { x : point.0, y : point.1 };
		let index = self.elements.iter().position( | element | element.kind == Kind::Wire && element.segment.contains( point ) ).unwrap();

		solution.readings[ index ].potentials[ 0 ]
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
		( Kind::Wire, ( 0, -2 ), ( 0, 0 ) ),
		( Kind::Wire, ( 0, 0 ), ( 0, 2 ) ),
		( Kind::Pmos, ( 0, -2 ), ( 2, -2 ) ),
		( Kind::Nmos, ( 0, 2 ), ( 2, 2 ) ),
		( Kind::Wire, ( 3, -3 ), ( 5, -3 ) ),
		( Kind::Wire, ( 5, -3 ), ( 5, 0 ) ),
		( Kind::Wire, ( 5, 0 ), ( 5, 1 ) ),
		( Kind::Wire, ( 5, 1 ), ( 3, 1 ) ),
		( Kind::Ground, ( 3, 3 ), ( 3, 3 ) ),
	] )
}

fn gate() -> Document
{
	layer( &[
		( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		( Kind::Input { high : false }, ( -1, 2 ), ( 0, 2 ) ),
		( Kind::Output, ( 6, 0 ), ( 5, 0 ) ),
	] )
}

fn nand() -> Model
{
	let rows = ( 0 .. 4 ).map( | row | vec![ Drive { volts : if row == 3 { 0.0 } else { 5.0 }, ohms : 1e-3 } ] ).collect();

	Model { rows, lights : vec![ Vec::new(); 4 ], next : vec![ 0; 4 ], delay : 1e-9 }
}

fn toggler() -> Model
{
	let mut rows = Vec::new();
	let mut next = Vec::new();

	for state in 0 .. 4
	{
		let ( high, clocked ) = ( state >> 1 == 1, state & 1 == 1 );

		for row in 0 .. 4
		{
			let ( clock, clear ) = ( row & 1 == 1, row >> 1 == 1 );
			let after = ! clear && high != ( clock && ! clocked );

			rows.push( vec![ Drive { volts : if after { 4.9 + 0.05 * state as f64 } else { 0.0 }, ohms : 1e-3 } ] );
			next.push( ( after as usize ) << 1 | clock as usize );
		}
	}

	let lights = vec![ Vec::new(); rows.len() ];

	Model { rows, lights, next, delay : 1e-9 }
}

fn symbol( x : i32 ) -> Part
{
	( Kind::Symbol { layer : 1, rotation : 0 }, ( x, 0 ), ( x, 0 ) )
}

fn circuit( parts : &[ Part ] ) -> Circuit
{
	let layers = [ layer( parts ), inverter() ];
	let mut catalog = Catalog::new( &layers, &[], &[], STEP );

	while catalog.advance( &layers ) {}

	let ( models, _ ) = catalog.finish();

	build( &layers, models )
}

fn build( layers : &[ Document ], models : Vec<Option<Model>> ) -> Circuit
{
	let ( elements, devices, netlist ) = flatten( layers, &mut Registry::default(), &models, 0, &[] ).unwrap();
	let plan = Plan::build( &elements, &devices, &netlist );

	Circuit { elements, devices, plan, models, logic : Logic::default() }
}

fn bits( solution : &Solution ) -> Vec<Vec<u64>>
{
	solution.readings.iter().map( | reading | reading.potentials.iter().chain( &reading.currents ).map( | value | value.to_bits() ).collect() ).collect()
}

fn wrapper() -> Document
{
	layer( &[
		( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		( Kind::Input { high : false }, ( -1, 2 ), ( 0, 2 ) ),
		( Kind::Output, ( 6, 1 ), ( 5, 1 ) ),
		( Kind::Symbol { layer : 2, rotation : 0 }, ( 2, 1 ), ( 2, 1 ) ),
		( Kind::Wire, ( 4, 1 ), ( 5, 1 ) ),
	] )
}

fn counter( hertz : f64 ) -> Circuit
{
	let layers = [ layer( &[ ( Kind::Clock { hertz, on : true }, ( 4, -1 ), ( 8, -1 ) ), ( Kind::Toggle { high : false }, ( 4, 1 ), ( 8, 1 ) ), symbol( 10 ), ( Kind::Wire, ( 12, 0 ), ( 14, 0 ) ) ] ), gate() ];

	build( &layers, vec![ None, Some( toggler() ) ] )
}


#[test]
fn it_pulses_a_clocked_circuit_to_the_solution_of_the_event_engine_one_delay_after_the_last_edge()
{
	let mut stepped = counter( 1e5 );
	let mut pulsed = counter( 1e5 );

	stepped.settle().unwrap();
	pulsed.settle().unwrap();

	let last = ( 0 .. 33 ).map( | _ | stepped.advance( STEP ).unwrap() ).last().unwrap();
	let solution = pulsed.pulse( STEP, 33 ).unwrap();

	assert!( pulsed.plan.analog.is_empty() );
	assert!( pulsed.at( &solution, ( 13, 0 ) ) > 2.5 );
	assert_eq!( ( bits( &solution ), &solution.branches, pulsed.logic.time, &pulsed.logic.states ), ( bits( &last ), &last.branches, stepped.logic.time, &stepped.logic.states ) );
}

#[test]
fn it_pulses_each_copy_of_a_symbol_in_its_own_lane()
{
	let copies = || build( &[ layer( &[
		( Kind::Clock { hertz : 1e5, on : true }, ( 4, -1 ), ( 8, -1 ) ),
		( Kind::Wire, ( 8, -1 ), ( 9, -1 ) ),
		( Kind::Wire, ( 9, -1 ), ( 9, 5 ) ),
		( Kind::Wire, ( 9, 5 ), ( 8, 5 ) ),
		( Kind::Toggle { high : false }, ( 4, 1 ), ( 8, 1 ) ),
		( Kind::Toggle { high : true }, ( 4, 7 ), ( 8, 7 ) ),
		( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 0 ), ( 10, 0 ) ),
		( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 6 ), ( 10, 6 ) ),
		( Kind::Wire, ( 12, 0 ), ( 14, 0 ) ),
		( Kind::Wire, ( 12, 6 ), ( 14, 6 ) ),
	] ), wrapper(), gate() ], vec![ None, None, Some( toggler() ) ] );
	let mut stepped = copies();
	let mut pulsed = copies();
	let machine = Machine { elements : &pulsed.elements, devices : &pulsed.devices, plan : &pulsed.plan, models : &pulsed.models };

	assert_eq!( Layout::new( machine ).unwrap().groups.iter().map( | group | group.lanes ).collect::<Vec<_>>(), vec![ 2 ] );

	stepped.settle().unwrap();
	pulsed.settle().unwrap();

	let last = ( 0 .. 33 ).map( | _ | stepped.advance( STEP ).unwrap() ).last().unwrap();
	let solution = pulsed.pulse( STEP, 33 ).unwrap();

	assert!( pulsed.plan.analog.is_empty() );
	assert!( pulsed.at( &solution, ( 13, 0 ) ) > 2.5 );
	assert!( pulsed.at( &solution, ( 13, 6 ) ) < 2.5 );
	assert_eq!( ( bits( &solution ), &pulsed.logic.states ), ( bits( &last ), &stepped.logic.states ) );
}

#[test]
fn it_pulses_one_lane_apart_from_the_lanes_that_share_its_state()
{
	let copies = || build( &[ layer( &[
		( Kind::Clock { hertz : 1e5, on : true }, ( 4, -1 ), ( 8, -1 ) ),
		( Kind::Clock { hertz : 5e4, on : true }, ( 4, 5 ), ( 8, 5 ) ),
		( Kind::Toggle { high : false }, ( 4, 1 ), ( 8, 1 ) ),
		( Kind::Toggle { high : false }, ( 4, 7 ), ( 8, 7 ) ),
		( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 0 ), ( 10, 0 ) ),
		( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 6 ), ( 10, 6 ) ),
		( Kind::Wire, ( 12, 0 ), ( 14, 0 ) ),
		( Kind::Wire, ( 12, 6 ), ( 14, 6 ) ),
	] ), wrapper(), gate() ], vec![ None, None, Some( toggler() ) ] );
	let mut stepped = copies();
	let mut pulsed = copies();

	stepped.settle().unwrap();
	pulsed.settle().unwrap();

	let last = ( 0 .. 33 ).map( | _ | stepped.advance( STEP ).unwrap() ).last().unwrap();
	let solution = pulsed.pulse( STEP, 33 ).unwrap();

	assert!( pulsed.at( &solution, ( 13, 0 ) ) > 2.5 );
	assert!( pulsed.at( &solution, ( 13, 6 ) ) < 2.5 );
	assert_eq!( ( bits( &solution ), &pulsed.logic.states ), ( bits( &last ), &stepped.logic.states ) );
}

#[test]
fn it_keeps_the_count_of_a_clock_across_pulses()
{
	let mut once = counter( 1e5 );
	let mut twice = counter( 1e5 );

	once.settle().unwrap();
	twice.settle().unwrap();
	once.pulse( STEP, 34 ).unwrap();
	twice.pulse( STEP, 17 ).unwrap();
	twice.pulse( STEP, 17 ).unwrap();

	assert_eq!( ( &once.logic.levels, &once.logic.states ), ( &twice.logic.levels, &twice.logic.states ) );
}

#[test]
fn it_gives_the_highest_current_of_a_digital_led_over_the_steps_it_pulses()
{
	let mut circuit = circuit( &[ ( Kind::Clock { hertz : 1000.0, on : true }, ( 4, 0 ), ( 8, 0 ) ), symbol( 10 ), ( Kind::Led { color : 0 }, ( 12, 0 ), ( 12, 4 ) ), ( Kind::Toggle { high : false }, ( 16, 4 ), ( 12, 4 ) ) ] );
	let led = circuit.elements.iter().position( | element | matches!( element.kind, Kind::Led { .. } ) ).unwrap();
	let lit = circuit.settle().unwrap().readings[ led ].currents[ 0 ];
	let solution = circuit.pulse( 1e-5, 75 ).unwrap();

	assert!( circuit.plan.analog.is_empty() );
	assert!( lit > 1e-3 );
	assert_eq!( solution.readings[ led ].currents[ 0 ], lit );
	assert!( circuit.pulse( 1e-5, 1 ).unwrap().readings[ led ].currents[ 0 ] < 1e-9 );
}

#[test]
fn it_redraws_from_its_last_solution_the_solution_of_a_full_redraw()
{
	let mut circuit = circuit( &[ ( Kind::Clock { hertz : 1000.0, on : true }, ( 4, 0 ), ( 8, 0 ) ), symbol( 10 ), ( Kind::Wire, ( 16, 0 ), ( 16, 2 ) ), ( Kind::Led { color : 0 }, ( 12, 0 ), ( 12, 4 ) ), ( Kind::Toggle { high : false }, ( 16, 4 ), ( 12, 4 ) ) ] );
	let mut last = circuit.settle().unwrap();

	for steps in [ 75, 1, 30, 2, 75 ]
	{
		let mut full = circuit.logic.clone();
		let machine = Machine { elements : &circuit.elements, devices : &circuit.devices, plan : &circuit.plan, models : &circuit.models };
		let expected = pulse( machine, &Layout::new( machine ).unwrap(), 1e-5, steps, &mut full ).unwrap();

		circuit.logic.spare = last;

		let solution = circuit.pulse( 1e-5, steps ).unwrap();

		assert!( ! solution.branches.is_empty() );
		assert_eq!( bits( &solution ), bits( &expected ) );
		assert_eq!( solution.branches, expected.branches );

		last = solution;
	}
}

#[test]
fn it_rejects_a_ring_of_gates_that_never_settles()
{
	let mut ring = circuit( &[ symbol( 10 ), symbol( 14 ), symbol( 18 ), ( Kind::Wire, ( 20, 0 ), ( 20, 3 ) ), ( Kind::Wire, ( 20, 3 ), ( 8, 3 ) ), ( Kind::Wire, ( 8, 3 ), ( 8, 0 ) ) ] );

	ring.settle().unwrap();

	assert!( ring.plan.analog.is_empty() );
	assert_eq!( ring.pulse( STEP, 1 ), Err( Error::Oscillating ) );
}

#[test]
fn it_rejects_a_gate_that_a_clock_edge_sets_oscillating()
{
	let layers = [ layer( &[ ( Kind::Clock { hertz : 1e5, on : true }, ( 4, -1 ), ( 8, -1 ) ), symbol( 10 ), ( Kind::Wire, ( 12, 0 ), ( 12, 3 ) ), ( Kind::Wire, ( 12, 3 ), ( 8, 3 ) ), ( Kind::Wire, ( 8, 3 ), ( 8, 1 ) ) ] ), gate() ];
	let mut circuit = build( &layers, vec![ None, Some( nand() ) ] );

	circuit.settle().unwrap();

	assert!( circuit.plan.analog.is_empty() );
	assert!( circuit.pulse( STEP, 4 ).is_ok() );
	assert_eq!( circuit.pulse( STEP, 2 ), Err( Error::Oscillating ) );
}

#[test]
fn it_rejects_a_gate_that_reads_an_analog_voltage()
{
	let mut circuit = circuit( &[ ( Kind::Battery { volts : 5.0 }, ( 6, 4 ), ( 6, 0 ) ), ( Kind::Ground, ( 6, 4 ), ( 6, 4 ) ), ( Kind::Wire, ( 6, 0 ), ( 8, 0 ) ), symbol( 10 ), ( Kind::Wire, ( 12, 0 ), ( 14, 0 ) ) ] );

	circuit.settle().unwrap();

	assert!( ! circuit.plan.analog.is_empty() );
	assert_eq!( circuit.pulse( STEP, 1 ), Err( Error::Analog ) );
}
