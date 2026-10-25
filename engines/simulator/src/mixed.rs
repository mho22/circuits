use crate::flatten::HIGH;
use crate::{ compute, Action, Branch, Device, Element, Error, History, Kind, Logic, Model, Plan, Reading, Solution, Source, FLOOR, THRESHOLD };


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
		Kind::Clock { hertz } => level( count( hertz, time ) ),
		Kind::Toggle { high : true } => HIGH,
		Kind::Slider { volts } => volts,
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
	events : usize,
	budget : usize,
}

impl<'a> Run<'a>
{
	fn new( elements : &'a [ Element ], devices : &'a [ Device ], plan : &'a Plan, models : &'a [ Option<Model> ], logic : &'a mut Logic, instant : bool ) -> Result<Run<'a>, Error>
	{
		let tables = devices.iter().map( | device | models.get( device.layer ).and_then( Option::as_ref ) ).collect::<Option<_>>().ok_or( Error::Layerless )?;
		let mut positions = vec![ None; elements.len() ];

		for ( position, &index ) in plan.analog.iter().enumerate() { positions[ index ] = Some( position ); }

		let analog = plan.analog.iter().map( | &index | elements[ index ] ).collect();
		let budget = EVENTS * ( devices.len() + plan.generators.len() + 1 );

		Ok( Run { elements, devices, plan, tables, positions, analog, logic, instant, events : 0, budget } )
	}

	fn start( &mut self, history : &History, step : f64 ) -> Result<(), Error>
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
		self.logic.clear();

		for device in 0 .. size
		{
			let table = self.tables[ device ];
			let width = 1 << self.plan.inputs[ device ].len();
			let Some( &( entry, state ) ) = self.logic.memory.get( &self.logic.keys[ device ] ).filter( | &&( entry, state ) | entry < table.rows.len() && state * width < table.rows.len() ) else { continue; };

			self.logic.applied[ device ] = entry;
			self.logic.states[ device ] = state;
		}

		for ( device, outputs ) in self.plan.outputs.iter().enumerate()
		{
			for ( drive, &net ) in self.tables[ device ].rows[ self.logic.applied[ device ] ].iter().zip( outputs )
			{
				let Some( net ) = net else { continue; };

				self.logic.levels[ net ] = drive.volts;
			}
		}

		for ( clock, &( index, net ) ) in self.plan.generators.iter().enumerate()
		{
			let Some( net ) = net else { continue; };

			self.logic.levels[ net ] = emit( self.elements[ index ].kind, time );

			let Kind::Clock { hertz } = self.elements[ index ].kind else { continue; };
			let edges = count( hertz, time );

			self.logic.schedule( ( edges + 1 ) as f64 / ( 2.0 * hertz ), Action::Edge { clock, count : edges + 1 } );
		}

		self.solve( history, step, time )?;

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

		let delay = if self.instant { 0.0 } else { self.tables[ device ].delay };

		self.logic.rows[ device ] = row;
		self.logic.stamps[ device ] += 1;
		self.logic.schedule( time + delay, Action::Fire { device, stamp : self.logic.stamps[ device ] } );
	}

	fn set( &mut self, net : usize, volts : f64, time : f64 )
	{
		let changed = ( volts > THRESHOLD ) != ( self.logic.levels[ net ] > THRESHOLD );
		let plan = self.plan;

		self.logic.levels[ net ] = volts;

		if ! changed { return; }

		for &reader in &plan.nets[ net ].readers { self.evaluate( reader, time ); }
	}

	fn drain( &mut self, until : f64 ) -> Result<(), Error>
	{
		while let Some( event ) = self.logic.next( until )
		{
			if let Action::Fire { device, stamp } = event.action && stamp != self.logic.stamps[ device ] { continue; }

			self.events += 1;

			if self.events > self.budget
			{
				if ! self.instant { return Err( Error::Oscillating ); }

				self.instant = false;
				self.events = 0;
			}

			match event.action
			{
				Action::Fire { device, .. } => self.fire( device, event.time ),
				Action::Edge { clock, count } => self.tick( clock, count, event.time ),
			}
		}

		Ok( () )
	}

	fn fire( &mut self, device : usize, time : f64 )
	{
		let table = self.tables[ device ];
		let plan = self.plan;
		let entry = self.logic.states[ device ] * ( 1 << plan.inputs[ device ].len() ) + self.logic.rows[ device ];

		self.logic.applied[ device ] = entry;
		self.logic.states[ device ] = table.next[ entry ];

		for ( drive, &net ) in table.rows[ entry ].iter().zip( &plan.outputs[ device ] )
		{
			let Some( net ) = net else { continue; };

			self.set( net, drive.volts, time );
		}
	}

	fn tick( &mut self, clock : usize, count : u64, time : f64 )
	{
		let ( index, net ) = self.plan.generators[ clock ];
		let ( Kind::Clock { hertz }, Some( net ) ) = ( self.elements[ index ].kind, net ) else { return; };

		self.set( net, level( count ), time );
		self.logic.schedule( ( count + 1 ) as f64 / ( 2.0 * hertz ), Action::Edge { clock, count : count + 1 } );
	}

	fn solve( &mut self, history : &History, step : f64, time : f64 ) -> Result<Solution, Error>
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

		let solution = compute( &self.analog, &plan.topology, history, step )?;

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

	fn assemble( &self, solution : Solution ) -> Solution
	{
		let mut analog : Vec<Option<Reading>> = solution.readings.into_iter().map( Some ).collect();
		let mut readings : Vec<Option<Reading>> = self.positions.iter().map( | position | position.and_then( | position | analog[ position ].take() ) ).collect();
		let mut branches = solution.branches;

		for ( net, &volts ) in self.plan.nets.iter().zip( &self.logic.levels )
		{
			for &index in &net.elements
			{
				readings[ index ] = Some( match self.elements[ index ].kind
				{
					Kind::Wire | Kind::Output => Reading { potentials : vec![ volts; 2 ], currents : vec![ f64::NAN; 2 ] },
					Kind::Supply { .. } | Kind::Clock { .. } | Kind::Toggle { .. } | Kind::Slider { .. } => Reading { potentials : vec![ 0.0, volts ], currents : vec![ 0.0; 2 ] },
					_ => Reading { potentials : vec![ volts; 2 ], currents : vec![ 0.0; 2 ] },
				} );
			}

			branches.extend( net.pieces.iter().map( | &segment | Branch { segment, voltage : volts, current : 0.0 } ) );
		}

		Solution { readings : readings.into_iter().flatten().collect(), branches }
	}
}


pub fn settle( elements : &[ Element ], devices : &[ Device ], plan : &Plan, models : &[ Option<Model> ], history : &History, step : f64, logic : &mut Logic ) -> Result<Solution, Error>
{
	let time = logic.time;
	let mut run = Run::new( elements, devices, plan, models, logic, true )?;
	let mut round = 0;

	run.start( history, step )?;

	loop
	{
		if round == ROUNDS { run.instant = false; }

		run.drain( time )?;

		let solution = run.solve( history, step, time )?;

		if ! run.logic.pending( time ) { return Ok( run.assemble( solution ) ); }

		round += 1;
	}
}

pub fn advance( elements : &[ Element ], devices : &[ Device ], plan : &Plan, models : &[ Option<Model> ], history : &History, step : f64, logic : &mut Logic ) -> Result<Solution, Error>
{
	let time = logic.time + step;
	let mut run = Run::new( elements, devices, plan, models, logic, false )?;

	run.drain( time )?;

	let solution = run.solve( history, step, time )?;

	run.logic.time = time;

	Ok( run.assemble( solution ) )
}
