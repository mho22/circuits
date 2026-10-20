use simulator::{ Element, History, Kind, Point, Reading, Segment, Solution };


fn element( id : u32, kind : Kind ) -> Element
{
	Element { id, kind, segment : Segment { from : Point { x : 0, y : 0 }, to : Point { x : 2, y : 0 } } }
}


#[test]
fn it_records_the_voltage_of_a_capacitor_and_a_diode_and_the_current_of_an_inductor()
{
	let elements = [ element( 3, Kind::Capacitor { farads : 1e-6 } ), element( 5, Kind::Inductor { henries : 1.0 } ), element( 7, Kind::Resistor { ohms : 1.0 } ), element( 9, Kind::Diode ) ];

	let solution = Solution { readings : vec![
		Reading { from : 5.0, to : 2.0, current : 0.1 },
		Reading { from : 1.0, to : 0.0, current : 0.2 },
		Reading { from : 1.0, to : 0.0, current : 1.0 },
		Reading { from : 0.7, to : 0.0, current : 0.004 },
	], branches : vec![] };

	let history = History::record( &elements, &solution );

	assert_eq!( history.get( 3 ), 3.0 );
	assert_eq!( history.get( 5 ), 0.2 );
	assert_eq!( history.get( 7 ), 0.0 );
	assert_eq!( history.get( 9 ), 0.7 );
}

#[test]
fn it_starts_every_part_at_rest()
{
	assert_eq!( History::default().get( 0 ), 0.0 );
}
