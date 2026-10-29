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
	sums : Vec<f64>,
	marks : Vec<f64>,
	opened : f64,
	dirty : Vec<usize>,
	marked : Vec<bool>,
	cache : HashMap<( u64, u64, u64 ), f64, BuildHasherDefault<Fold>>,
}

impl Exposure
{
	pub fn new( plan : &Plan, elements : &[ Element ], levels : &[ f64 ], ohms : Vec<f64>, time : f64 ) -> Exposure
	{
		let count = plan.lights.len();
		let nets = plan.nets.len();
		let mut exposure = Exposure { ohms, shares : vec![ 0; nets ], lit : vec![ false; count ], currents : vec![ 0.0; count ], sums : vec![ 0.0; count ], marks : vec![ time; count ], opened : time, dirty : Vec::new(), marked : vec![ false; nets ], cache : HashMap::default() };

		for ( light, &( _, anode, cathode ) ) in plan.lights.iter().enumerate()
		{
			if levels[ anode ] <= levels[ cathode ] { continue; }

			exposure.lit[ light ] = true;
			exposure.shares[ anode ] += 1;
			exposure.shares[ cathode ] += 1;
		}

		for light in 0 .. count { exposure.refresh( plan, elements, levels, light, time ); }

		exposure
	}

	pub fn change( &mut self, plan : &Plan, levels : &[ f64 ], net : usize, volts : f64, ohms : f64 )
	{
		if plan.nets[ net ].lights.is_empty() || ( volts == levels[ net ] && ohms == self.ohms[ net ] ) { return; }

		self.ohms[ net ] = ohms;
		self.mark( net );
	}

	pub fn expose( &mut self, time : f64 ) -> Vec<f64>
	{
		let span = time - self.opened;
		let means = self.currents.iter().zip( &self.sums ).zip( &self.marks ).map( | ( ( &current, &sum ), &mark ) | if span > 0.0 { ( sum + current * ( time - mark ) ) / span } else { current } ).collect();

		self.sums.fill( 0.0 );
		self.marks.fill( time );
		self.opened = time;

		means
	}

	pub fn flush( &mut self, plan : &Plan, elements : &[ Element ], levels : &[ f64 ], time : f64 )
	{
		let changed = self.dirty.len();

		for position in 0 .. changed
		{
			let net = self.dirty[ position ];

			for &light in &plan.nets[ net ].lights
			{
				let ( _, anode, cathode ) = plan.lights[ light ];
				let lit = levels[ anode ] > levels[ cathode ];

				if lit == self.lit[ light ] { continue; }

				self.lit[ light ] = lit;

				for end in [ anode, cathode ] { if lit { self.shares[ end ] += 1; } else { self.shares[ end ] -= 1; } }

				self.mark( if anode == net { cathode } else { anode } );
			}
		}

		let dirty = std::mem::take( &mut self.dirty );

		for &net in &dirty
		{
			self.marked[ net ] = false;

			for &light in &plan.nets[ net ].lights
			{
				let ( _, anode, cathode ) = plan.lights[ light ];

				if self.marked[ anode ] || self.marked[ cathode ] || ( ! self.lit[ light ] && self.currents[ light ] == 0.0 ) { continue; }

				self.refresh( plan, elements, levels, light, time );
			}
		}

		self.dirty = dirty;
		self.dirty.clear();
	}

	fn mark( &mut self, net : usize )
	{
		if self.marked[ net ] { return; }

		self.marked[ net ] = true;
		self.dirty.push( net );
	}

	fn refresh( &mut self, plan : &Plan, elements : &[ Element ], levels : &[ f64 ], light : usize, time : f64 )
	{
		let ( index, anode, cathode ) = plan.lights[ light ];
		let junction = elements[ index ].kind.junction();
		let volts = levels[ anode ] - levels[ cathode ];
		let ohms = self.shares[ anode ] as f64 * self.ohms[ anode ] + self.shares[ cathode ] as f64 * self.ohms[ cathode ];

		self.sums[ light ] += self.currents[ light ] * ( time - self.marks[ light ] );
		self.marks[ light ] = time;
		self.currents[ light ] = *self.cache.entry( ( junction.saturation.to_bits(), volts.to_bits(), ohms.to_bits() ) ).or_insert_with( || shine( junction, volts, ohms ) );
	}
}
