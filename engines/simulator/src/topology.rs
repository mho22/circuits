use std::collections::HashMap;

use crate::{ Element, Kind, Netlist, UnionFind };


#[derive( Clone, Debug, Default, PartialEq )]
pub struct Topology
{
	pub netlist : Netlist,
	pub terminals : Vec<Vec<usize>>,
	pub unknowns : Vec<Option<usize>>,
	pub nodes : usize,
	pub batteries : Vec<usize>,
}

impl Topology
{
	pub fn build( elements : &[ Element ] ) -> Topology
	{
		let netlist = Netlist::build( elements );
		let terminals : Vec<Vec<usize>> = elements.iter().map( | element | match element.kind
		{
			Kind::Supply { .. } => vec![ netlist.earth.unwrap(), netlist.net( element.segment.from ) ],
			_ => element.terminals().into_iter().map( | point | netlist.net( point ) ).collect(),
		} ).collect();
		let mut islands = UnionFind::new( netlist.count );

		for ( element, nets ) in elements.iter().zip( &terminals )
		{
			if matches!( element.kind, Kind::Wire | Kind::Ground | Kind::Input { .. } | Kind::Output | Kind::Switch { .. } ) { continue; }

			for &net in &nets[ 1 .. ] { islands.union( nets[ 0 ], net ); }
		}

		let mut references = HashMap::new();

		if let Some( earth ) = netlist.earth { references.insert( islands.find( earth ), earth ); }

		for net in 0 .. netlist.count { references.entry( islands.find( net ) ).or_insert( net ); }

		let mut unknowns = vec![ None; netlist.count ];
		let mut nodes = 0;

		for net in 0 .. netlist.count
		{
			if references[ &islands.find( net ) ] == net { continue; }

			unknowns[ net ] = Some( nodes );

			nodes += 1;
		}

		let batteries : Vec<usize> = ( 0 .. elements.len() ).filter( | &index | matches!( elements[ index ].kind, Kind::Battery { .. } | Kind::Supply { .. } ) ).collect();

		Topology { netlist, terminals, unknowns, nodes, batteries }
	}
}
