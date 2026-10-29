use std::collections::VecDeque;

use crate::flatten::HIGH;
use crate::{ compute, Action, Branch, Device, Drive, Element, Error, Exposure, History, Kind, Logic, Machine, Model, Plan, Reading, Solution, Source, FLOOR, THRESHOLD };


const ROUNDS : usize = 100;
const EVENTS : usize = 64;


fn count( hertz : f64, time : f64 ) -> u64
{
	( time * hertz * 2.0 ).floor() as u64
}

fn level( count : u64 ) -> f64
{
	if count % 2 == 1 { HIGH } else { 0.0 }
}

fn emit( kind : Kind, time : f64 ) -> f64
{
	match kind
	{
		Kind::Clock { hertz, on : true } => level( count( hertz, time ) ),
		Kind::Toggle { high : true } => HIGH,
		Kind::Slider { volts } | Kind::Supply { volts } => volts,
		_ => 0.0,
	}
}


struct Run<'a>
{
	elements : &'a [ Element ],
	devices : &'a [ Device ],
	plan : &'a Plan,
	tables : Vec<&'a Model>,
	positions : Vec<Option<usize>>,
	analog : Vec<Element>,
	logic : &'a mut Logic,
	instant : bool,
	resting : Vec<bool>,
	events : usize,
	budget : usize,
}

impl<'a> Run<'a>
{
	fn new( machine : Machine<'a>, logic : &'a mut Logic, instant : bool ) -> Result<Run<'a>, Error>
	{
		let Machine { elements, devices, plan, models } = machine;
		let tables = devices.iter().map( | device | models.get( device.layer ).and_then( Option::as_ref ) ).collect::<Option<_>>().ok_or( Error::Layerless )?;
		let mut positions = vec![ None; elements.len() ];

		for ( position, &index ) in plan.analog.iter().enumerate() { positions[ index ] = Some( position ); }

		let analog = plan.analog.iter().map( | &index | elements[ index ] ).collect();
		let budget = EVENTS * ( devices.len() + plan.generators.len() + 1 );

		Ok( Run { elements, devices, plan, tables, positions, analog, logic, instant, resting : vec![ false; devices.len() ], events : 0, budget } )
	}

	fn start( &mut self, history : &History, step : f64, rest : bool ) -> Result<(), Error>
	{
		let time = self.logic.time;
		let size = self.devices.len();

		self.logic.memory.extend( self.logic.keys.iter().copied().zip( self.logic.applied.iter().copied().zip( self.logic.states.iter().copied() ) ) );

		self.logic.keys = self.devices.iter().map( | device | self.elements[ device.outputs[ 0 ].1 ].id ).collect();
		self.logic.levels = vec![ 0.0; self.plan.nets.len() ];
		self.logic.rows = vec![ usize::MAX; size ];
		self.logic.applied = vec![ 0; size ];
		self.logic.states = vec![ 0; size ];
		self.logic.sensed = self.plan.inputs.iter().map( | sources | vec![ false; sources.len() ] ).collect();
		self.logic.stamps = vec![ 0; size ];
		self.logic.since = vec![ None; size ];
		self.logic.queued = vec![ VecDeque::new(); size ];
		self.logic.clear();
		self.resting = vec![ rest; size ];

		let mut ohms = vec![ f64::INFINITY; self.plan.nets.len() ];

		for device in 0 .. size
		{
			let table = self.tables[ device ];
			let width = 1 << self.plan.inputs[ device ].len();
			let Some( &( entry, state ) ) = self.logic.memory.get( &self.logic.keys[ device ] ).filter( | &&( entry, state ) | entry < table.rows.len() && state * width < table.rows.len() ) else { continue; };

			self.logic.applied[ device ] = entry;
			self.logic.states[ device ] = state;
			self.resting[ device ] = false;
		}

		for ( device, outputs ) in self.plan.outputs.iter().enumerate()
		{
			for ( drive, &net ) in self.tables[ device ].rows[ self.logic.applied[ device ] ].iter().zip( outputs )
			{
				let Some( net ) = net else { continue; };

				self.logic.levels[ net ] = drive.volts;
				ohms[ net ] = drive.ohms.max( FLOOR );
			}
		}

		for ( clock, &( index, net ) ) in self.plan.generators.iter().enumerate()
		{
			let Some( net ) = net else { continue; };

			self.logic.levels[ net ] = emit( self.elements[ index ].kind, time );
			ohms[ net ] = FLOOR;

			let Kind::Clock { hertz, on : true } = self.elements[ index ].kind else { continue; };
			let edges = count( hertz, time );

			self.logic.schedule( ( edges + 1 ) as f64 / ( 2.0 * hertz ), Action::Edge { clock, count : edges + 1 } );
		}

		self.logic.exposure = Exposure::new( self.plan, self.elements, &self.logic.levels, ohms );
		self.solve( history, step, time, Solution::default() )?;

		for device in 0 .. size { self.evaluate( device, time ); }

		Ok( () )
	}

	fn row( &self, device : usize ) -> usize
	{
		self.plan.inputs[ device ].iter().zip( &self.logic.sensed[ device ] ).zip( 0 .. ).fold( 0, | row, ( ( &source, &sense ), bit ) |
		{
			let high = match source
			{
				Source::Net( net ) => self.logic.levels[ net ] > THRESHOLD,
				Source::Analog( _ ) => sense,
			};

			row | ( high as usize ) << bit
		} )
	}

	fn evaluate( &mut self, device : usize, time : f64 )
	{
		let row = self.row( device );

		if row == self.logic.rows[ device ] { return; }

		if self.logic.since[ device ].is_some_and( | since | since < time ) && self.remembers( device )
		{
			let entry = self.commit( device );

			self.logic.queued[ device ].push_back( ( self.logic.stamps[ device ], entry ) );
		}

		let delay = if self.instant { 0.0 } else { self.tables[ device ].delay };

		self.logic.rows[ device ] = row;
		self.logic.since[ device ] = Some( time );
		self.logic.stamps[ device ] += 1;
		self.logic.schedule( time + delay, Action::Fire { device, stamp : self.logic.stamps[ device ] } );
	}

	fn set( &mut self, net : usize, volts : f64, ohms : f64, time : f64 )
	{
		let changed = ( volts > THRESHOLD ) != ( self.logic.levels[ net ] > THRESHOLD );
		let plan = self.plan;

		self.logic.exposure.change( plan, &self.logic.levels, net, volts, ohms );
		self.logic.levels[ net ] = volts;

		if ! changed { return; }

		for &reader in &plan.nets[ net ].readers { self.evaluate( reader, time ); }
	}

	fn drain( &mut self, until : f64 ) -> Result<(), Error>
	{
		while let Some( event ) = self.logic.next( until )
		{
			if let Action::Fire { device, stamp } = event.action && ! self.live( device, stamp ) { continue; }

			self.events += 1;

			if self.events > self.budget
			{
				if ! self.instant { return Err( Error::Oscillating ); }

				self.instant = false;
				self.events = 0;
			}

			match event.action
			{
				Action::Fire { device, stamp } => self.fire( device, stamp, event.time ),
				Action::Edge { clock, count } => self.tick( clock, count, event.time ),
			}
		}

		self.logic.exposure.flush( self.plan, self.elements, &self.logic.levels );

		Ok( () )
	}

	fn remembers( &self, device : usize ) -> bool
	{
		self.tables[ device ].rows.len() > 1 << self.plan.inputs[ device ].len()
	}

	fn live( &self, device : usize, stamp : u64 ) -> bool
	{
		stamp == self.logic.stamps[ device ] || self.logic.queued[ device ].front().is_some_and( | &( first, _ ) | first == stamp )
	}

	fn commit( &mut self, device : usize ) -> usize
	{
		let width = 1 << self.plan.inputs[ device ].len();
		let row = self.logic.rows[ device ];

		if self.resting[ device ] { self.logic.states[ device ] = self.rest( device, width, row ); }

		let entry = self.logic.states[ device ] * width + row;

		self.logic.states[ device ] = self.tables[ device ].next[ entry ];
		self.logic.since[ device ] = None;

		entry
	}

	fn rest( &self, device : usize, width : usize, row : usize ) -> usize
	{
		let table = self.tables[ device ];
		let stable = ( 0 .. table.next.len() / width ).filter( | &state | table.next[ state * width + row ] == state );

		stable.min_by_key( | &state | table.rows[ state * width + row ].iter().map( Drive::high ).collect::<Vec<_>>() ).unwrap_or( self.logic.states[ device ] )
	}

	fn fire( &mut self, device : usize, stamp : u64, time : f64 )
	{
		let table = self.tables[ device ];
		let plan = self.plan;
		let entry = match self.logic.queued[ device ].front()
		{
			Some( &( first, entry ) ) if first == stamp => { self.logic.queued[ device ].pop_front(); entry },
			_ => self.commit( device ),
		};

		self.logic.applied[ device ] = entry;

		for ( drive, &net ) in table.rows[ entry ].iter().zip( &plan.outputs[ device ] )
		{
			let Some( net ) = net else { continue; };

			self.set( net, drive.volts, drive.ohms.max( FLOOR ), time );
		}
	}

	fn tick( &mut self, clock : usize, count : u64, time : f64 )
	{
		let ( index, net ) = self.plan.generators[ clock ];
		let ( Kind::Clock { hertz, .. }, Some( net ) ) = ( self.elements[ index ].kind, net ) else { return; };

		self.set( net, level( count ), FLOOR, time );
		self.logic.schedule( ( count + 1 ) as f64 / ( 2.0 * hertz ), Action::Edge { clock, count : count + 1 } );
	}

	fn solve( &mut self, history : &History, step : f64, time : f64, spare : Solution ) -> Result<Solution, Error>
	{
		let plan = self.plan;

		for ( index, device ) in self.devices.iter().enumerate()
		{
			for ( ( &( supply, resistor ), drive ), net ) in device.outputs.iter().zip( &self.tables[ index ].rows[ self.logic.applied[ index ] ] ).zip( &plan.outputs[ index ] )
			{
				if net.is_some() { continue; }

				let ( Some( supply ), Some( resistor ) ) = ( self.positions[ supply ], self.positions[ resistor ] ) else { continue; };

				self.analog[ supply ].kind = Kind::Supply { volts : drive.volts };
				self.analog[ resistor ].kind = Kind::Resistor { ohms : drive.ohms.max( FLOOR ) };
			}
		}

		for &( index, net ) in &plan.generators
		{
			let ( None, Some( position ) ) = ( net, self.positions[ index ] ) else { continue; };

			self.analog[ position ].kind = Kind::Supply { volts : emit( self.elements[ index ].kind, time ) };
		}

		let solution = compute( &self.analog, &plan.topology, history, step, spare )?;

		for ( index, sources ) in plan.inputs.iter().enumerate()
		{
			for ( bit, &source ) in sources.iter().enumerate()
			{
				let Source::Analog( input ) = source else { continue; };
				let Some( position ) = self.positions[ input ] else { continue; };
				let high = solution.readings[ position ].potentials[ 1 ] > THRESHOLD;

				if high == self.logic.sensed[ index ][ bit ] { continue; }

				self.logic.sensed[ index ][ bit ] = high;

				self.evaluate( index, time );
			}
		}

		Ok( solution )
	}

	fn assemble( &mut self, solution : Solution, mut readings : Vec<Reading> ) -> Solution
	{
		let mut branches = solution.branches;

		readings.resize_with( self.elements.len(), Reading::default );

		for ( reading, &index ) in solution.readings.into_iter().zip( &self.plan.analog ) { readings[ index ] = reading; }

		for ( net, &volts ) in self.plan.nets.iter().zip( &self.logic.levels )
		{
			for &index in &net.elements
			{
				let Reading { potentials, currents } = &mut readings[ index ];

				potentials.clear();
				currents.clear();

				match self.elements[ index ].kind
				{
					Kind::Wire | Kind::Output => { potentials.extend( [ volts; 2 ] ); currents.extend( [ f64::NAN; 2 ] ); },
					Kind::Supply { .. } | Kind::Clock { .. } | Kind::Toggle { .. } | Kind::Slider { .. } => { potentials.extend( [ 0.0, volts ] ); currents.extend( [ 0.0; 2 ] ); },
					_ => { potentials.extend( [ volts; 2 ] ); currents.extend( [ 0.0; 2 ] ); },
				}
			}

			branches.extend( net.pieces.iter().map( | &segment | Branch { segment, voltage : volts, current : 0.0 } ) );
		}

		let peaks = self.logic.exposure.expose();

		for ( &( index, anode, cathode ), current ) in self.plan.lights.iter().zip( peaks )
		{
			let Reading { potentials, currents } = &mut readings[ index ];

			potentials.clear();
			currents.clear();
			potentials.extend( [ self.logic.levels[ anode ], self.logic.levels[ cathode ] ] );
			currents.extend( [ current, - current ] );
		}

		Solution { readings, branches }
	}
}


pub fn settle( machine : Machine, history : &History, step : f64, logic : &mut Logic, rest : bool ) -> Result<Solution, Error>
{
	let time = logic.time;
	let mut run = Run::new( machine, logic, true )?;
	let mut round = 0;
	let mut spare = Solution::default();

	run.start( history, step, rest )?;

	loop
	{
		if round == ROUNDS { run.instant = false; }

		run.drain( time )?;

		let solution = run.solve( history, step, time, spare )?;

		if ! run.logic.pending( time )
		{
			run.logic.exposure.reset();

			return Ok( run.assemble( solution, Vec::new() ) );
		}

		spare = solution;
		round += 1;
	}
}

pub fn advance( machine : Machine, history : &History, step : f64, logic : &mut Logic ) -> Result<Solution, Error>
{
	let time = logic.time + step;
	let Solution { mut readings, branches } = std::mem::take( &mut logic.spare );
	let mut run = Run::new( machine, logic, false )?;

	run.drain( time )?;

	let analog = machine.plan.analog.iter().map( | &index | readings.get_mut( index ).map( std::mem::take ).unwrap_or_default() ).collect();
	let solution = run.solve( history, step, time, Solution { readings : analog, branches } )?;

	run.logic.time = time;

	Ok( run.assemble( solution, readings ) )
}

pub fn rush( machine : Machine, step : f64, count : u32, logic : &mut Logic ) -> Result<Solution, Error>
{
	let Solution { readings, mut branches } = std::mem::take( &mut logic.spare );
	let mut run = Run::new( machine, logic, false )?;

	branches.clear();

	for _ in 0 .. count
	{
		let time = run.logic.time + step;

		run.events = 0;
		run.drain( time )?;
		run.logic.time = time;
	}

	Ok( run.assemble( Solution { readings : Vec::new(), branches }, readings ) )
}

pub fn idle( elements : &[ Element ], plan : &Plan, step : f64, logic : &Logic ) -> bool
{
	let time = logic.time + step;
	let constant = | &( index, net ) : &( usize, Option<usize> ) | net.is_some() || emit( elements[ index ].kind, time ) == emit( elements[ index ].kind, logic.time );

	! plan.reactive && ! logic.pending( time ) && plan.generators.iter().all( constant )
}
