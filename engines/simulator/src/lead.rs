use crate::{ Element, Kind, Netlist };


pub fn leads( elements : &[ Element ] ) -> Vec<Option<( usize, usize )>>
{
	let wired : Vec<Element> = elements.iter().map( | element | if matches!( element.kind, Kind::Resistor { .. } ) { Element { kind : Kind::Wire, ..*element } } else { *element } ).collect();
	let netlist = Netlist::build( &wired );
	let pins : Vec<usize> = elements.iter().filter( | part | part.kind.pin() ).map( | pin | netlist.net( pin.segment.to ) ).collect();
	let pin = | point | pins.iter().position( | &net | net == netlist.net( point ) );

	elements.iter().filter( | part | matches!( part.kind, Kind::Led { .. } ) ).map( | led | pin( led.segment.from ).zip( pin( led.segment.to ) ) ).collect()
}
