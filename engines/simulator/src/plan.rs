use std::collections::HashMap;

use crate::{ Device, Element, Hashing, Kind, Netlist, Segment, Topology };


#[derive( Clone, Copy, Debug, PartialEq )]
pub enum Source
{
	Net( usize ),
	Analog( usize ),
}


#[derive( Clone, Debug, Default, PartialEq )]
pub struct Net
{
	pub elements : Vec<usize>,
	pub readers : Vec<usize>,
	pub pieces : Vec<Segment>,
	pub lights : Vec<usize>,
}


#[derive( Clone, Debug, Default, PartialEq )]
pub struct Plan
{
	pub analog : Vec<usize>,
	pub nets : Vec<Net>,
	pub inputs : Vec<Vec<Source>>,
	pub outputs : Vec<Vec<Option<usize>>>,
	pub generators : Vec<( usize, Option<usize> )>,
	pub topology : Topology,
	pub reactive : bool,
	pub lights : Vec<( usize, usize, usize )>,
	pub starts : Vec<usize>,
}

impl Plan
{
	pub fn build( elements : &[ Element ], devices : &[ Device ], netlist : &Netlist ) -> Plan
	{
		let mut joined = vec![ false; elements.len() ];
		let mut drivers : HashMap<usize, usize, Hashing> = HashMap::default();
		let mut touched = vec![ false; netlist.count ];

		let supplies : Vec<usize> = devices.iter().flat_map( | device | device.outputs.iter().map( | &( supply, _ ) | supply ) ).collect();
		let generators : Vec<usize> = ( 0 .. elements.len() ).filter( | &index | match elements[ index ].kind
		{
			Kind::Clock { .. } | Kind::Toggle { .. } | Kind::Slider { .. } => true,
			Kind::Supply { .. } => ! supplies.contains( &index ),
			_ => false,
		} ).collect();

		for index in devices.iter().flat_map( | device | device.inputs.iter().copied().chain( device.outputs.iter().flat_map( | &( supply, resistor ) | [ supply, resistor ] ) ) ) { joined[ index ] = true; }

		for &index in &generators
		{
			joined[ index ] = true;
			*drivers.entry( netlist.net( elements[ index ].segment.to ) ).or_insert( 0 ) += 1;
		}

		for ( index, element ) in elements.iter().enumerate()
		{
			if matches!( element.kind, Kind::Wire | Kind::Output | Kind::Switch { closed : true } | Kind::Led { .. } ) || joined[ index ] { continue; }

			for point in element.terminals() { touched[ netlist.net( point ) ] = true; }
		}

		for device in devices
		{
			for &( _, resistor ) in &device.outputs { *drivers.entry( netlist.net( elements[ resistor ].segment.to ) ).or_insert( 0 ) += 1; }

			for &input in &device.inputs
			{
				if matches!( elements[ input ].kind, Kind::Switch { .. } | Kind::Resistor { .. } ) { continue; }

				touched[ netlist.net( elements[ input ].segment.to ) ] = true;
			}
		}

		let ends = | index : usize | { let terminals = elements[ index ].terminals(); ( netlist.net( terminals[ 0 ] ), netlist.net( terminals[ 1 ] ) ) };
		let mut leds : Vec<usize> = ( 0 .. elements.len() ).filter( | &index | matches!( elements[ index ].kind, Kind::Led { .. } ) ).collect();

		loop
		{
			let open = | net : usize | ! touched[ net ] && drivers.get( &net ) == Some( &1 );
			let ( kept, refused ) : ( Vec<usize>, Vec<usize> ) = leds.iter().partition( | &&index | { let ( anode, cathode ) = ends( index ); anode != cathode && open( anode ) && open( cathode ) } );

			if refused.is_empty() { break; }

			for index in refused { let ( anode, cathode ) = ends( index ); touched[ anode ] = true; touched[ cathode ] = true; }

			leds = kept;
		}

		let mut numbers : HashMap<usize, usize, Hashing> = HashMap::default();
		let mut nets : Vec<Net> = Vec::new();
		let mut number = | net : usize | -> Option<usize>
		{
			if touched[ net ] || drivers.get( &net ).is_some_and( | &count | count != 1 ) { return None; }

			Some( *numbers.entry( net ).or_insert_with( || { nets.push( Net::default() ); nets.len() - 1 } ) )
		};

		let outputs : Vec<Vec<Option<usize>>> = devices.iter().map( | device | device.outputs.iter().map( | &( _, resistor ) | number( netlist.net( elements[ resistor ].segment.to ) ) ).collect() ).collect();
		let generators : Vec<( usize, Option<usize> )> = generators.iter().map( | &index | ( index, number( netlist.net( elements[ index ].segment.to ) ) ) ).collect();
		let sensed = | input : usize | if matches!( elements[ input ].kind, Kind::Resistor { .. } ) { elements[ input ].segment.from } else { elements[ input ].segment.to };
		let inputs : Vec<Vec<Source>> = devices.iter().map( | device | device.inputs.iter().map( | &input | number( netlist.net( sensed( input ) ) ).map_or( Source::Analog( input ), Source::Net ) ).collect() ).collect();
		let inert : Vec<( usize, Option<usize> )> = elements.iter().enumerate().filter( | ( _, element ) | matches!( element.kind, Kind::Wire | Kind::Output | Kind::Switch { closed : true } ) ).map( | ( index, element ) | ( index, number( netlist.net( element.terminals()[ 0 ] ) ) ) ).collect();
		let pieces : Vec<( Segment, Option<usize> )> = netlist.edges.iter().map( | &( a, b ) | ( Segment { from : netlist.points[ a ], to : netlist.points[ b ] }, number( netlist.nets[ a ] ) ) ).collect();

		let lights : Vec<( usize, usize, usize )> = leds.iter().filter_map( | &index | { let ( anode, cathode ) = ends( index ); Some( ( index, number( anode )?, number( cathode )? ) ) } ).collect();
		let mut digital = vec![ false; elements.len() ];

		for ( light, &( index, anode, cathode ) ) in lights.iter().enumerate()
		{
			digital[ index ] = true;
			nets[ anode ].lights.push( light );
			nets[ cathode ].lights.push( light );
		}

		let mut claim = | nets : &mut Vec<Net>, net : usize, index : usize | { nets[ net ].elements.push( index ); digital[ index ] = true; };

		for ( device, outputs ) in devices.iter().zip( &outputs )
		{
			for ( &( supply, resistor ), &net ) in device.outputs.iter().zip( outputs )
			{
				let Some( net ) = net else { continue; };

				claim( &mut nets, net, supply );
				claim( &mut nets, net, resistor );
			}
		}

		for &( index, net ) in &generators
		{
			let Some( net ) = net else { continue; };

			claim( &mut nets, net, index );
		}

		for ( index, ( device, sources ) ) in devices.iter().zip( &inputs ).enumerate()
		{
			for ( &input, &source ) in device.inputs.iter().zip( sources )
			{
				let Source::Net( net ) = source else { continue; };

				claim( &mut nets, net, input );

				if ! nets[ net ].readers.contains( &index ) { nets[ net ].readers.push( index ); }
			}
		}

		for ( index, net ) in inert
		{
			let Some( net ) = net else { continue; };

			claim( &mut nets, net, index );
		}

		for ( segment, net ) in pieces
		{
			let Some( net ) = net else { continue; };

			nets[ net ].pieces.push( segment );
		}

		let analog : Vec<usize> = ( 0 .. elements.len() ).filter( | &index | ! digital[ index ] ).collect();
		let circuit : Vec<Element> = analog.iter().map( | &index | match elements[ index ].kind
		{
			Kind::Clock { .. } | Kind::Toggle { .. } | Kind::Slider { .. } => Element { kind : Kind::Supply { volts : 0.0 }, ..elements[ index ] },
			_ => elements[ index ],
		} ).collect();
		let topology = Topology::build( &circuit );
		let reactive = circuit.iter().any( | element | matches!( element.kind, Kind::Capacitor { .. } | Kind::Inductor { .. } ) );
		let starts = [ 0 ].into_iter().chain( nets.iter().scan( 0, | end, net | { *end += net.pieces.len(); Some( *end ) } ) ).collect();

		Plan { analog, nets, inputs, outputs, generators, topology, reactive, lights, starts }
	}
}
