use crate::{ linearize, Element, History, Kind };


pub fn companion( element : &Element, history : &History, step : f64, guess : f64 ) -> Option<( f64, f64 )>
{
	match element.kind
	{
		Kind::Resistor { ohms } => Some( ( 1.0 / ohms, 0.0 ) ),
		Kind::Capacitor { farads } => Some( ( farads / step, farads / step * history.get( element.id ) ) ),
		Kind::Inductor { henries } => Some( ( step / henries, - history.get( element.id ) ) ),
		Kind::Diode => Some( linearize( guess ) ),
		_ => None,
	}
}
