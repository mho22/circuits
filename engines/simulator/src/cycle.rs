use std::collections::VecDeque;

use crate::layout::{ Group, Input, Table, LANES };
use crate::mixed::{ assemble, count, level };
use crate::{ Drive, Error, Layout, Logic, Machine, Model, Solution, FLOOR, THRESHOLD };


struct Clock
{
	net : usize,
	hertz : f64,
	count : u64,
	time : f64,
}


struct Cycle<'a>
{
	machine : Machine<'a>,
	layout : &'a Layout,
	models : Vec<&'a Model>,
	states : Vec<Vec<( u64, usize )>>,
	applied : Vec<Vec<( u64, usize )>>,
	previous : Vec<u64>,
	levels : Vec<u64>,
	clocks : Vec<Clock>,
	next : f64,
	queue : VecDeque<usize>,
	queued : Vec<bool>,
	masks : Vec<u64>,
	highs : Vec<u64>,
	woken : Vec<usize>,
}

impl<'a> Cycle<'a>
{
	fn new( machine : Machine<'a>, layout : &'a Layout, logic : &Logic ) -> Result<Cycle<'a>, Error>
	{
		let models = layout.tables.iter().map( | table | machine.models.get( table.layer ).and_then( Option::as_ref ) ).collect::<Option<_>>().ok_or( Error::Layerless )?;
		let members = | group : &Group | &layout.devices[ group.first .. group.first + group.lanes ];
		let mut previous = Vec::with_capacity( layout.inputs.len() );

		for group in &layout.groups
		{
			for bit in 0 .. layout.tables[ group.table ].width { previous.push( members( group ).iter().enumerate().fold( 0, | mask, ( lane, &device ) | mask | ( ( logic.rows[ device ] >> bit & 1 ) as u64 ) << lane ) ); }
		}

		let time = logic.time;
		let clocks : Vec<Clock> = layout.clocks.iter().map( | &( net, hertz ) |
		{
			let count = ( count( hertz, time ) .. ).find( | &edges | ( edges + 1 ) as f64 / ( 2.0 * hertz ) > time ).unwrap_or( 0 );

			Clock { net, hertz, count, time : ( count + 1 ) as f64 / ( 2.0 * hertz ) }
		} ).collect();

		Ok( Cycle
		{
			machine,
			layout,
			models,
			states : layout.groups.iter().map( | group | sort( members( group ).iter().map( | &device | logic.states[ device ] ) ) ).collect(),
			applied : layout.groups.iter().map( | group | sort( members( group ).iter().map( | &device | logic.applied[ device ] ) ) ).collect(),
			previous,
			levels : layout.nets.iter().map( | nets | nets.iter().enumerate().fold( 0, | mask, ( lane, net ) | mask | ( net.is_some_and( | net | logic.levels[ net ] > THRESHOLD ) as u64 ) << lane ) ).collect(),
			next : next( &clocks ),
			clocks,
			queue : VecDeque::new(),
			queued : vec![ false; layout.groups.len() ],
			masks : Vec::new(),
			highs : Vec::new(),
			woken : Vec::new(),
		} )
	}

	fn run( &mut self, logic : &mut Logic, until : f64 ) -> Result<(), Error>
	{
		let mut waves = 0;

		loop
		{
			let time = self.next;

			if time > until { break; }

			for index in 0 .. self.clocks.len()
			{
				let Clock { net, hertz, count, time : edge } = self.clocks[ index ];

				if edge != time { continue; }

				let volts = level( count + 1 );
				let ( bundle, lane ) = self.layout.homes[ net ];

				self.clocks[ index ].count = count + 1;
				self.clocks[ index ].time = ( count + 2 ) as f64 / ( 2.0 * hertz );
				self.raise( logic, net, Drive { volts, ohms : FLOOR } );
				self.levels[ bundle ] = self.levels[ bundle ] & ! ( 1 << lane ) | ( ( volts > THRESHOLD ) as u64 ) << lane;

				if ! self.woken.contains( &bundle ) { self.woken.push( bundle ); }
			}

			for position in 0 .. self.woken.len() { self.wake( self.woken[ position ] ); }

			self.woken.clear();
			self.next = next( &self.clocks );

			self.spread( logic )?;
			waves += 1;
		}

		if waves > 0 { logic.exposure.flush( self.machine.plan, &logic.levels ); }

		Ok( () )
	}

	fn raise( &self, logic : &mut Logic, net : usize, drive : Drive )
	{
		if self.layout.lit[ net ] { logic.exposure.change( self.machine.plan, &logic.levels, net, drive.volts, drive.ohms ); }

		logic.levels[ net ] = drive.volts;
	}

	fn set( &mut self, bundle : usize, levels : u64 )
	{
		if levels == self.levels[ bundle ] { return; }

		self.levels[ bundle ] = levels;
		self.wake( bundle );
	}

	fn wake( &mut self, bundle : usize )
	{
		for &reader in &self.layout.readers[ bundle ]
		{
			if self.queued[ reader ] { continue; }

			self.queued[ reader ] = true;
			self.queue.push_back( reader );
		}
	}

	fn high( &self, net : usize ) -> bool
	{
		let ( bundle, lane ) = self.layout.homes[ net ];

		self.levels[ bundle ] >> lane & 1 == 1
	}

	fn spread( &mut self, logic : &mut Logic ) -> Result<(), Error>
	{
		let layout = self.layout;
		let mut events = 0;
		let mut masks = std::mem::take( &mut self.masks );
		let mut highs = std::mem::take( &mut self.highs );

		while let Some( index ) = self.queue.pop_front()
		{
			self.queued[ index ] = false;
			events += 1;

			if events > layout.budget { return Err( Error::Oscillating ); }

			let Group { table, lanes, inputs, outputs, lit, .. } = layout.groups[ index ];
			let Table { width, count, highs : ref pattern, .. } = layout.tables[ table ];
			let model = self.models[ table ];
			let full = u64::MAX >> ( LANES - lanes );
			let mut changed = 0;

			masks.clear();

			for bit in 0 .. width
			{
				let mask = match layout.inputs[ inputs + bit ]
				{
					Input::Bundle( bundle ) => self.levels[ bundle ],
					Input::Net( net ) => if self.high( net ) { full } else { 0 },
					Input::Lanes( start ) => ( 0 .. lanes ).fold( 0, | mask, lane | mask | ( self.high( layout.gathered[ start + lane ] ) as u64 ) << lane ),
				};

				changed |= ( mask ^ self.previous[ inputs + bit ] ) & full;
				self.previous[ inputs + bit ] = mask;
				masks.push( mask );
			}

			if changed == 0 { continue; }

			highs.clear();
			highs.resize( count, 0 );

			let mut rest = changed;

			while rest != 0
			{
				let lane = 1 << rest.trailing_zeros();
				let Some( &( mask, state ) ) = self.states[ index ].iter().find( | &&( mask, _ ) | mask & lane != 0 ) else { break; };
				let mut same = mask & rest;
				let mut row = 0;

				for ( bit, &input ) in masks.iter().enumerate()
				{
					if input & lane == 0 { same &= ! input; continue; }

					same &= input;
					row |= 1 << bit;
				}

				let entry = state << width | row;

				rest &= ! same;
				assign( &mut self.states[ index ], same, model.next[ entry ] );
				assign( &mut self.applied[ index ], same, entry );

				for ( output, high ) in highs.iter_mut().enumerate()
				{
					if pattern[ entry * count + output ] { *high |= same; }
				}

				if ! lit { continue; }

				for ( output, drive ) in model.rows[ entry ].iter().enumerate()
				{
					let nets = &layout.nets[ layout.outputs[ outputs + output ] ];

					for lane in ones( same )
					{
						let Some( net ) = nets[ lane ] else { continue; };

						if layout.lit[ net ] { self.raise( logic, net, Drive { volts : drive.volts, ohms : drive.ohms.max( FLOOR ) } ); }
					}
				}
			}

			for ( output, &high ) in highs.iter().enumerate()
			{
				let bundle = layout.outputs[ outputs + output ];

				self.set( bundle, self.levels[ bundle ] & ! changed | high );
			}
		}

		self.masks = masks;
		self.highs = highs;

		Ok( () )
	}

	fn apply( &self, logic : &mut Logic )
	{
		let layout = self.layout;

		for ( index, group ) in layout.groups.iter().enumerate()
		{
			let width = layout.tables[ group.table ].width;
			let model = self.models[ group.table ];
			let members = &layout.devices[ group.first .. group.first + group.lanes ];

			for &( mask, state ) in &self.states[ index ]
			{
				for lane in ones( mask ) { logic.states[ members[ lane ] ] = state; }
			}

			for &( mask, entry ) in &self.applied[ index ]
			{
				for lane in ones( mask )
				{
					let device = members[ lane ];

					logic.applied[ device ] = entry;
					logic.rows[ device ] = ( 0 .. width ).fold( 0, | row, bit | row | ( ( self.previous[ group.inputs + bit ] >> lane & 1 ) as usize ) << bit );

					for ( output, drive ) in model.rows[ entry ].iter().enumerate()
					{
						let Some( net ) = layout.nets[ layout.outputs[ group.outputs + output ] ][ lane ] else { continue; };

						logic.levels[ net ] = drive.volts;
					}
				}
			}
		}
	}
}


fn next( clocks : &[ Clock ] ) -> f64
{
	clocks.iter().map( | clock | clock.time ).fold( f64::INFINITY, f64::min )
}

fn ones( mask : u64 ) -> impl Iterator<Item = usize>
{
	let mut rest = mask;

	std::iter::from_fn( move || { if rest == 0 { return None; } let lane = rest.trailing_zeros() as usize; rest &= rest - 1; Some( lane ) } )
}

fn sort( values : impl Iterator<Item = usize> ) -> Vec<( u64, usize )>
{
	let mut pairs : Vec<( u64, usize )> = Vec::new();

	for ( lane, value ) in values.enumerate() { assign( &mut pairs, 1 << lane, value ); }

	pairs
}

fn assign( pairs : &mut Vec<( u64, usize )>, lanes : u64, value : usize )
{
	if let [ ( mask, known ) ] = pairs.as_mut_slice() && *mask == lanes { *known = value; return; }

	pairs.retain_mut( | ( mask, _ ) | { *mask &= ! lanes; *mask != 0 } );

	match pairs.iter_mut().find( | ( _, known ) | *known == value )
	{
		Some( ( mask, _ ) ) => *mask |= lanes,
		None => pairs.push( ( lanes, value ) ),
	}
}


pub fn pulse( machine : Machine, layout : &Layout, step : f64, steps : u32, logic : &mut Logic ) -> Result<Solution, Error>
{
	if logic.firing() { return Err( Error::Oscillating ); }

	let previous = std::mem::take( &mut logic.spare );
	let mut cycle = Cycle::new( machine, layout, logic )?;

	for _ in 0 .. steps
	{
		let time = logic.time + step;

		cycle.run( logic, time )?;
		logic.time = time;
	}

	cycle.apply( logic );

	Ok( assemble( machine.elements, machine.plan, logic, Solution::default(), previous ) )
}
