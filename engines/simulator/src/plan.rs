use std::collections::HashMap;

use crate::{ Device, Element, Kind, Netlist, Segment, Topology };


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
}

impl Plan
{
	pub fn build( elements : &[ Element ], devices : &[ Device ] ) -> Plan
	{
		let netlist = Netlist::build( elements );
		let mut joined = vec![ false; elements.len() ];
		let mut drivers : HashMap<usize, usize> = HashMap::new();
		let mut touched = vec![ false; netlist.count ];

		let generators : Vec<usize> = ( 0 .. elements.len() ).filter( | &index | matches!( elements[ index ].kind, Kind::Clock { .. } | Kind::Toggle { .. } ) ).collect();

		for index in devices.iter().flat_map( | device | device.inputs.iter().copied().chain( device.outputs.iter().flat_map( | &( supply, resistor ) | [ supply, resistor ] ) ) ) { joined[ index ] = true; }

		for &index in &generators
		{
			joined[ index ] = true;
			*drivers.entry( netlist.net( elements[ index ].segment.to ) ).or_insert( 0 ) += 1;
		}

		for ( index, element ) in elements.iter().enumerate()
		{
			if element.kind == Kind::Wire || element.kind == Kind::Output || joined[ index ] { continue; }

			for point in element.terminals() { touched[ netlist.net( point ) ] = true; }
		}

		for device in devices
		{
			for &( _, resistor ) in &device.outputs { *drivers.entry( netlist.net( elements[ resistor ].segment.to ) ).or_insert( 0 ) += 1; }

			for &input in &device.inputs
			{
				if matches!( elements[ input ].kind, Kind::Switch { .. } ) { continue; }

				touched[ netlist.net( elements[ input ].segment.to ) ] = true;
			}
		}

		let mut numbers : HashMap<usize, usize> = HashMap::new();
		let mut nets : Vec<Net> = Vec::new();
		let mut number = | net : usize | -> Option<usize>
		{
			if touched[ net ] || drivers.get( &net ) != Some( &1 ) { return None; }

			Some( *numbers.entry( net ).or_insert_with( || { nets.push( Net::default() ); nets.len() - 1 } ) )
		};

		let outputs : Vec<Vec<Option<usize>>> = devices.iter().map( | device | device.outputs.iter().map( | &( _, resistor ) | number( netlist.net( elements[ resistor ].segment.to ) ) ).collect() ).collect();
		let generators : Vec<( usize, Option<usize> )> = generators.iter().map( | &index | ( index, number( netlist.net( elements[ index ].segment.to ) ) ) ).collect();
		let inputs : Vec<Vec<Source>> = devices.iter().map( | device | device.inputs.iter().map( | &input | number( netlist.net( elements[ input ].segment.to ) ).map_or( Source::Analog( input ), Source::Net ) ).collect() ).collect();
		let inert : Vec<( usize, Option<usize> )> = elements.iter().enumerate().filter( | ( _, element ) | matches!( element.kind, Kind::Wire | Kind::Output ) ).map( | ( index, element ) | ( index, number( netlist.net( element.terminals()[ 0 ] ) ) ) ).collect();
		let pieces : Vec<( Segment, Option<usize> )> = netlist.edges.iter().map( | &( a, b ) | ( Segment { from : netlist.points[ a ], to : netlist.points[ b ] }, number( netlist.nets[ a ] ) ) ).collect();

		let mut digital = vec![ false; elements.len() ];

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
			Kind::Clock { .. } | Kind::Toggle { .. } => Element { kind : Kind::Supply { volts : 0.0 }, ..elements[ index ] },
			_ => elements[ index ],
		} ).collect();
		let topology = Topology::build( &circuit );

		Plan { analog, nets, inputs, outputs, generators, topology }
	}
}
