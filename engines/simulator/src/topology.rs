use std::collections::HashMap;
use std::sync::OnceLock;

use crate::{ Element, Flow, Kind, Netlist, UnionFind };


#[derive( Clone, Debug, Default, PartialEq )]
pub struct Topology
{
	pub netlist : Netlist,
	pub terminals : Vec<Vec<usize>>,
	pub unknowns : Vec<Option<usize>>,
	pub nodes : usize,
	pub batteries : Vec<usize>,
	pub taps : Vec<Vec<usize>>,
	pub joints : Vec<usize>,
	pub flow : Flow,
	pub order : OnceLock<Vec<usize>>,
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

		let earth = netlist.points.len();
		let taps = elements.iter().map( | element | match element.kind
		{
			Kind::Wire | Kind::Ground | Kind::Input { .. } | Kind::Output | Kind::Switch { .. } => Vec::new(),
			Kind::Supply { .. } => vec![ earth, netlist.index( element.segment.from ) ],
			_ => element.terminals().into_iter().map( | point | netlist.index( point ) ).collect(),
		} ).collect();
		let switches = ( 0 .. elements.len() ).filter( | &index | elements[ index ].kind == Kind::Switch { closed : true } );
		let grounds : Vec<usize> = ( 0 .. elements.len() ).filter( | &index | elements[ index ].kind == Kind::Ground ).collect();
		let edges : Vec<( usize, usize )> = netlist.edges.iter().chain( &netlist.links ).copied().chain( grounds.iter().map( | &index | ( netlist.index( elements[ index ].segment.from ), earth ) ) ).collect();
		let joints = switches.chain( grounds ).collect();
		let flow = Flow::build( earth + 1, &edges );

		Topology { netlist, terminals, unknowns, nodes, batteries, taps, joints, flow, order : OnceLock::new() }
	}
}
