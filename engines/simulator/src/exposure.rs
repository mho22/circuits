use std::collections::HashMap;
use std::hash::{ BuildHasherDefault, Hasher };

use crate::{ Element, Junction, Plan };


const HALVINGS : usize = 60;
const SPREAD : u64 = 0x517c_c1b7_2722_0a95;


fn shine( junction : Junction, volts : f64, ohms : f64 ) -> f64
{
	if volts <= 0.0 || ! ohms.is_finite() { return 0.0; }

	let current = | drop : f64 | junction.saturation * ( ( drop / junction.thermal ).exp() - 1.0 );
	let ( mut low, mut high ) = ( 0.0, volts );

	for _ in 0 .. HALVINGS
	{
		let middle = 0.5 * ( low + high );

		if middle + current( middle ) * ohms > volts { high = middle; } else { low = middle; }
	}

	current( low )
}


#[derive( Clone, Copy, Default )]
struct Fold( u64 );

impl Hasher for Fold
{
	fn finish( &self ) -> u64
	{
		self.0
	}

	fn write( &mut self, bytes : &[ u8 ] )
	{
		for &byte in bytes { self.write_u64( u64::from( byte ) ); }
	}

	fn write_u64( &mut self, value : u64 )
	{
		self.0 = ( self.0.rotate_left( 5 ) ^ value ).wrapping_mul( SPREAD );
	}
}


#[derive( Clone, Debug, Default )]
pub struct Exposure
{
	ohms : Vec<f64>,
	shares : Vec<usize>,
	lit : Vec<bool>,
	currents : Vec<f64>,
	peaks : Vec<f64>,
	dirty : Vec<usize>,
	marked : Vec<bool>,
	moved : Vec<bool>,
	starts : Vec<usize>,
	links : Vec<( usize, usize, bool )>,
	lits : Vec<Vec<( usize, usize, usize )>>,
	slots : Vec<[ usize; 2 ]>,
	junctions : Vec<Junction>,
	recent : Option<( ( u64, u64, u64 ), f64 )>,
	cache : HashMap<( u64, u64, u64 ), f64, BuildHasherDefault<Fold>>,
}

impl Exposure
{
	pub fn new( plan : &Plan, elements : &[ Element ], levels : &[ f64 ], ohms : Vec<f64> ) -> Exposure
	{
		let count = plan.lights.len();
		let nets = plan.nets.len();
		let starts = [ 0 ].into_iter().chain( plan.nets.iter().scan( 0, | end, net | { *end += net.lights.len(); Some( *end ) } ) ).collect();
		let links = plan.nets.iter().enumerate().flat_map( | ( index, net ) | net.lights.iter().map( move | &light | { let ( _, anode, cathode ) = plan.lights[ light ]; if anode == index { ( light, cathode, true ) } else { ( light, anode, false ) } } ) ).collect();
		let mut exposure = Exposure { ohms, shares : vec![ 0; nets ], lit : vec![ false; count ], currents : vec![ 0.0; count ], peaks : vec![ 0.0; count ], dirty : Vec::new(), marked : vec![ false; nets ], moved : vec![ false; nets ], starts, links, lits : vec![ Vec::new(); nets ], slots : vec![ [ 0; 2 ]; count ], junctions : plan.lights.iter().map( | &( index, _, _ ) | elements[ index ].kind.junction() ).collect(), recent : None, cache : HashMap::default() };

		for ( light, &( _, anode, cathode ) ) in plan.lights.iter().enumerate()
		{
			if levels[ anode ] <= levels[ cathode ] { continue; }

			exposure.lit[ light ] = true;
			exposure.shares[ anode ] += 1;
			exposure.shares[ cathode ] += 1;
			exposure.light( anode, 0, light, cathode );
			exposure.light( cathode, 1, light, anode );
		}

		for light in 0 .. count { exposure.refresh( plan, levels, light ); }

		exposure
	}

	pub fn change( &mut self, plan : &Plan, levels : &[ f64 ], net : usize, volts : f64, ohms : f64 )
	{
		if plan.nets[ net ].lights.is_empty() || ( volts == levels[ net ] && ohms == self.ohms[ net ] ) { return; }

		self.moved[ net ] |= volts != levels[ net ];
		self.ohms[ net ] = ohms;
		self.mark( net );
	}

	pub fn expose( &mut self ) -> Vec<f64>
	{
		std::mem::replace( &mut self.peaks, self.currents.clone() )
	}

	pub fn reset( &mut self )
	{
		self.peaks.clone_from( &self.currents );
	}

	pub fn flush( &mut self, plan : &Plan, levels : &[ f64 ] )
	{
		let changed = self.dirty.len();

		for position in 0 .. changed
		{
			let net = self.dirty[ position ];

			if ! self.moved[ net ] { continue; }

			self.moved[ net ] = false;

			for link in self.starts[ net ] .. self.starts[ net + 1 ]
			{
				let ( light, other, anode ) = self.links[ link ];
				let lit = if anode { levels[ net ] > levels[ other ] } else { levels[ other ] > levels[ net ] };

				if lit == self.lit[ light ] { continue; }

				self.lit[ light ] = lit;

				let side = usize::from( ! anode );

				for ( end, far, side ) in [ ( net, other, side ), ( other, net, 1 - side ) ]
				{
					if lit { self.shares[ end ] += 1; self.light( end, side, light, far ); continue; }

					self.shares[ end ] -= 1;
					self.darken( end, side, light );
				}

				if ! lit { self.currents[ light ] = 0.0; }

				self.mark( other );
			}
		}

		let dirty = std::mem::take( &mut self.dirty );

		for &net in &dirty
		{
			self.marked[ net ] = false;

			for index in 0 .. self.lits[ net ].len()
			{
				let ( light, other, _ ) = self.lits[ net ][ index ];

				if self.marked[ other ] { continue; }

				self.refresh( plan, levels, light );
			}
		}

		self.dirty = dirty;
		self.dirty.clear();
	}

	fn light( &mut self, net : usize, side : usize, light : usize, other : usize )
	{
		self.slots[ light ][ side ] = self.lits[ net ].len();
		self.lits[ net ].push( ( light, other, side ) );
	}

	fn darken( &mut self, net : usize, side : usize, light : usize )
	{
		let place = self.slots[ light ][ side ];

		self.lits[ net ].swap_remove( place );

		let Some( &( moved, _, side ) ) = self.lits[ net ].get( place ) else { return; };

		self.slots[ moved ][ side ] = place;
	}

	fn mark( &mut self, net : usize )
	{
		if self.marked[ net ] { return; }

		self.marked[ net ] = true;
		self.dirty.push( net );
	}

	fn refresh( &mut self, plan : &Plan, levels : &[ f64 ], light : usize )
	{
		let ( _, anode, cathode ) = plan.lights[ light ];
		let junction = self.junctions[ light ];
		let volts = levels[ anode ] - levels[ cathode ];
		let ohms = self.shares[ anode ] as f64 * self.ohms[ anode ] + self.shares[ cathode ] as f64 * self.ohms[ cathode ];
		let key = ( junction.saturation.to_bits(), volts.to_bits(), ohms.to_bits() );
		let current = match self.recent
		{
			Some( ( known, current ) ) if known == key => current,
			_ => *self.cache.entry( key ).or_insert_with( || shine( junction, volts, ohms ) ),
		};

		self.recent = Some( ( key, current ) );
		self.currents[ light ] = current;
		self.peaks[ light ] = self.peaks[ light ].max( current );
	}
}
