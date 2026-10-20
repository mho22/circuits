use crate::{ linearize, Element, History, Kind, Stamp };


const FORWARD_GAIN : f64 = 100.0;
const REVERSE_GAIN : f64 = 1.0;
const THROUGH : [ f64; 3 ] = [ 1.0, -1.0, 0.0 ];
const FORWARD : [ f64; 3 ] = [ 1.0 / FORWARD_GAIN, 1.0, -1.0 - 1.0 / FORWARD_GAIN ];
const REVERSE : [ f64; 3 ] = [ 1.0 / REVERSE_GAIN, -1.0 - 1.0 / REVERSE_GAIN, 1.0 ];


pub fn companion( element : &Element, history : &History, step : f64, guesses : [ f64; 2 ] ) -> Vec<Stamp>
{
	let [ stored, _ ] = history.get( element.id );

	let through = | conductance : f64, source : f64 | Stamp { across : ( 0, 1 ), conductance, source, weights : THROUGH };

	let junction = | index : usize, weights : [ f64; 3 ] |
	{
		let ( conductance, source ) = linearize( guesses[ index ] );

		Stamp { across : element.kind.junctions()[ index ], conductance, source, weights }
	};

	match element.kind
	{
		Kind::Resistor { ohms } => vec![ through( 1.0 / ohms, 0.0 ) ],
		Kind::Capacitor { farads } => vec![ through( farads / step, farads / step * stored ) ],
		Kind::Inductor { henries } => vec![ through( step / henries, - stored ) ],
		Kind::Diode => vec![ junction( 0, THROUGH ) ],
		Kind::Npn => vec![ junction( 0, FORWARD ), junction( 1, REVERSE ) ],
		Kind::Pnp => vec![ junction( 0, FORWARD.map( | weight | - weight ) ), junction( 1, REVERSE.map( | weight | - weight ) ) ],
		_ => vec![],
	}
}
