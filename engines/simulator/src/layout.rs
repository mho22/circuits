use std::collections::HashMap;

use crate::{ Device, Drive, Error, Kind, Machine, Source };


pub const LANES : usize = 64;
const EVENTS : usize = 64;
const HOMELESS : usize = usize::MAX;


#[derive( Clone, Debug, Default, PartialEq )]
pub struct Table
{
	pub layer : usize,
	pub width : usize,
	pub count : usize,
	pub highs : Vec<bool>,
}


#[derive( Clone, Copy, Debug, PartialEq )]
pub enum Input
{
	Bundle( usize ),
	Net( usize ),
	Lanes( usize ),
}


#[derive( Clone, Copy, Debug, PartialEq )]
pub struct Group
{
	pub table : usize,
	pub lanes : usize,
	pub first : usize,
	pub inputs : usize,
	pub outputs : usize,
	pub lit : bool,
}


#[derive( Clone, Debug, Default, PartialEq )]
pub struct Layout
{
	pub tables : Vec<Table>,
	pub groups : Vec<Group>,
	pub devices : Vec<usize>,
	pub inputs : Vec<Input>,
	pub gathered : Vec<usize>,
	pub outputs : Vec<usize>,
	pub readers : Vec<Vec<usize>>,
	pub nets : Vec<Vec<Option<usize>>>,
	pub homes : Vec<( usize, usize )>,
	pub lit : Vec<bool>,
	pub clocks : Vec<( usize, f64 )>,
	pub budget : usize,
}

impl Layout
{
	pub fn new( machine : Machine ) -> Result<Layout, Error>
	{
		let Machine { elements, devices, plan, models } = machine;
		let mut indexes : Vec<Option<usize>> = vec![ None; models.len() ];
		let mut sources : Vec<Vec<usize>> = Vec::with_capacity( devices.len() );
		let mut layout = Layout
		{
			homes : vec![ ( HOMELESS, 0 ); plan.nets.len() ],
			lit : plan.nets.iter().map( | net | ! net.lights.is_empty() ).collect(),
			budget : EVENTS * ( devices.len() + plan.generators.len() + 1 ),
			..Layout::default()
		};

		for ( device, inputs ) in devices.iter().zip( &plan.inputs )
		{
			let model = models.get( device.layer ).and_then( Option::as_ref ).ok_or( Error::Layerless )?;

			sources.push( inputs.iter().map( | &source | match source { Source::Net( net ) => Some( net ), Source::Analog( _ ) => None } ).collect::<Option<_>>().ok_or( Error::Analog )? );
			indexes[ device.layer ].get_or_insert_with( || { layout.tables.push( Table { layer : device.layer, width : inputs.len(), count : device.outputs.len(), highs : model.rows.iter().flatten().map( Drive::high ).collect() } ); layout.tables.len() - 1 } );
		}

		for members in members( devices )
		{
			let table = indexes[ devices[ members[ 0 ] ].layer ].ok_or( Error::Layerless )?;
			let lit = members.iter().any( | &device | plan.outputs[ device ].iter().flatten().any( | &net | layout.lit[ net ] ) );

			layout.groups.push( Group { table, lanes : members.len(), first : layout.devices.len(), inputs : 0, outputs : layout.outputs.len(), lit } );

			for output in 0 .. layout.tables[ table ].count
			{
				layout.outputs.push( layout.nets.len() );
				layout.bundle( members.iter().map( | &device | plan.outputs[ device ][ output ] ).collect() );
			}

			layout.devices.extend_from_slice( &members );
		}

		for index in 0 .. layout.groups.len()
		{
			for bit in 0 .. layout.tables[ layout.groups[ index ].table ].width
			{
				let nets = layout.column( &sources, index, bit );
				let homeless = nets.iter().all( | &net | layout.homes[ net ].0 == HOMELESS );
				let distinct = ( 1 .. nets.len() ).all( | lane | ! nets[ .. lane ].contains( &nets[ lane ] ) );

				if nets.len() > 1 && homeless && distinct { layout.bundle( nets.into_iter().map( Some ).collect() ); }
			}
		}

		for net in 0 .. plan.nets.len()
		{
			if layout.homes[ net ].0 == HOMELESS { layout.bundle( vec![ Some( net ) ] ); }
		}

		for index in 0 .. layout.groups.len()
		{
			layout.groups[ index ].inputs = layout.inputs.len();

			for bit in 0 .. layout.tables[ layout.groups[ index ].table ].width
			{
				let nets = layout.column( &sources, index, bit );
				let bundle = layout.homes[ nets[ 0 ] ].0;
				let input = if nets.iter().enumerate().all( | ( lane, &net ) | layout.homes[ net ] == ( bundle, lane ) ) { Input::Bundle( bundle ) }
					else if nets.iter().all( | &net | net == nets[ 0 ] ) { Input::Net( nets[ 0 ] ) }
					else { layout.gathered.extend_from_slice( &nets ); Input::Lanes( layout.gathered.len() - nets.len() ) };

				for &net in &nets { layout.readers[ layout.homes[ net ].0 ].push( index ); }

				layout.inputs.push( input );
			}
		}

		for readers in &mut layout.readers { readers.dedup(); }

		layout.clocks = plan.generators.iter().filter_map( | &( index, net ) | match ( elements[ index ].kind, net )
		{
			( Kind::Clock { hertz, on : true }, Some( net ) ) => Some( ( net, hertz ) ),
			_ => None,
		} ).collect();

		Ok( layout )
	}

	fn column( &self, sources : &[ Vec<usize> ], group : usize, bit : usize ) -> Vec<usize>
	{
		let Group { lanes, first, .. } = self.groups[ group ];

		self.devices[ first .. first + lanes ].iter().map( | &device | sources[ device ][ bit ] ).collect()
	}

	fn bundle( &mut self, nets : Vec<Option<usize>> )
	{
		let bundle = self.nets.len();

		for ( lane, net ) in nets.iter().enumerate()
		{
			let Some( net ) = *net else { continue; };

			self.homes[ net ] = ( bundle, lane );
		}

		self.nets.push( nets );
		self.readers.push( Vec::new() );
	}
}


fn members( devices : &[ Device ] ) -> Vec<Vec<usize>>
{
	let mut copies : HashMap<( usize, i32 ), ( usize, usize )> = HashMap::new();

	for ( index, device ) in devices.iter().enumerate()
	{
		for &scope in &device.scopes { copies.entry( scope ).or_insert( ( index, 0 ) ).1 += 1; }
	}

	let mut layers : HashMap<usize, Vec<( usize, usize )>> = HashMap::new();

	for ( &( layer, _ ), &copy ) in &copies { layers.entry( layer ).or_default().push( copy ); }

	let alike = | copies : &[ ( usize, usize ) ] | copies.iter().all( | &( start, size ) | size == copies[ 0 ].1 && ( 0 .. size ).all( | offset | devices[ start + offset ].layer == devices[ copies[ 0 ].0 + offset ].layer ) );
	let chosen = layers.into_values().filter( | copies | copies.len() > 1 && alike( copies ) ).max_by_key( | copies | ( copies.len() * copies[ 0 ].1, copies.iter().map( | &( start, _ ) | start ).min() ) );
	let mut taken = vec![ false; devices.len() ];
	let mut lanes = Vec::new();

	if let Some( mut copies ) = chosen
	{
		copies.sort_unstable();

		for chunk in copies.chunks( LANES )
		{
			for offset in 0 .. chunk[ 0 ].1
			{
				let members : Vec<usize> = chunk.iter().map( | &( start, _ ) | start + offset ).collect();

				for &device in &members { taken[ device ] = true; }

				lanes.push( members );
			}
		}
	}

	lanes.extend( ( 0 .. devices.len() ).filter( | &device | ! taken[ device ] ).map( | device | vec![ device ] ) );

	lanes
}
