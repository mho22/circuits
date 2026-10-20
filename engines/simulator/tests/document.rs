use simulator::{ Document, Element, Error, Kind, Operation, Point, Segment };


const FOO : Point = Point { x : 0, y : 0 };
const BAR : Point = Point { x : 4, y : 0 };
const BAZ : Point = Point { x : 4, y : 4 };


fn add( kind : Kind, from : Point, to : Point ) -> Operation
{
	Operation::Add { kind, segment : Segment { from, to } }
}


#[test]
fn it_adds_parts_with_new_ids()
{
	let mut document = Document::default();

	assert_eq!( document.apply( add( Kind::Wire, FOO, BAR ) ), Ok( 0 ) );
	assert_eq!( document.apply( add( Kind::Resistor { ohms : 1000.0 }, BAR, BAZ ) ), Ok( 1 ) );
	assert_eq!( document.apply( add( Kind::Ground, FOO, FOO ) ), Ok( 2 ) );

	assert_eq!( document.elements(), &[
		Element { id : 0, kind : Kind::Wire, segment : Segment { from : FOO, to : BAR } },
		Element { id : 1, kind : Kind::Resistor { ohms : 1000.0 }, segment : Segment { from : BAR, to : BAZ } },
		Element { id : 2, kind : Kind::Ground, segment : Segment { from : FOO, to : FOO } },
	] );
}

#[test]
fn it_rejects_a_part_without_length()
{
	let mut document = Document::default();

	assert_eq!( document.apply( add( Kind::Wire, FOO, FOO ) ), Err( Error::Empty ) );
	assert_eq!( document.apply( add( Kind::Battery { volts : 5.0 }, FOO, FOO ) ), Err( Error::Empty ) );
	assert!( document.elements().is_empty() );
}

#[test]
fn it_rejects_a_ground_with_length()
{
	let mut document = Document::default();

	assert_eq!( document.apply( add( Kind::Ground, FOO, BAR ) ), Err( Error::Misplaced ) );
}

#[test]
fn it_rejects_a_crooked_part()
{
	let mut document = Document::default();

	assert_eq!( document.apply( add( Kind::Wire, FOO, Point { x : 2, y : 1 } ) ), Err( Error::Crooked ) );
	assert!( document.elements().is_empty() );
}

#[test]
fn it_rejects_a_part_with_an_invalid_value()
{
	let mut document = Document::default();

	assert_eq!( document.apply( add( Kind::Resistor { ohms : 0.0 }, FOO, BAR ) ), Err( Error::Resistance ) );
	assert!( document.elements().is_empty() );
}

#[test]
fn it_deletes_a_part()
{
	let mut document = Document::default();

	document.apply( add( Kind::Wire, FOO, BAR ) ).unwrap();
	document.apply( add( Kind::Wire, BAR, BAZ ) ).unwrap();

	assert_eq!( document.apply( Operation::Delete { id : 0 } ), Ok( 0 ) );
	assert_eq!( document.elements(), &[ Element { id : 1, kind : Kind::Wire, segment : Segment { from : BAR, to : BAZ } } ] );
}

#[test]
fn it_never_reuses_a_deleted_id()
{
	let mut document = Document::default();

	document.apply( add( Kind::Wire, FOO, BAR ) ).unwrap();
	document.apply( Operation::Delete { id : 0 } ).unwrap();

	assert_eq!( document.apply( add( Kind::Wire, FOO, BAR ) ), Ok( 1 ) );
}

#[test]
fn it_rejects_an_operation_on_a_missing_part()
{
	let mut document = Document::default();

	assert_eq!( document.apply( Operation::Delete { id : 7 } ), Err( Error::Missing ) );
	assert_eq!( document.apply( Operation::SetValue { id : 7, value : 1.0 } ), Err( Error::Missing ) );
}

#[test]
fn it_sets_the_value_of_a_part()
{
	let mut document = Document::default();

	document.apply( add( Kind::Battery { volts : 5.0 }, FOO, BAR ) ).unwrap();

	assert_eq!( document.apply( Operation::SetValue { id : 0, value : 9.0 } ), Ok( 0 ) );
	assert_eq!( document.elements()[ 0 ].kind, Kind::Battery { volts : 9.0 } );
}

#[test]
fn it_keeps_the_value_when_the_new_one_is_invalid()
{
	let mut document = Document::default();

	document.apply( add( Kind::Resistor { ohms : 1000.0 }, FOO, BAR ) ).unwrap();
	document.apply( add( Kind::Wire, BAR, BAZ ) ).unwrap();

	assert_eq!( document.apply( Operation::SetValue { id : 0, value : -1.0 } ), Err( Error::Resistance ) );
	assert_eq!( document.apply( Operation::SetValue { id : 1, value : 1.0 } ), Err( Error::Valueless ) );
	assert_eq!( document.elements()[ 0 ].kind, Kind::Resistor { ohms : 1000.0 } );
}

#[test]
fn it_finds_the_parts_at_a_point_topmost_first()
{
	let mut document = Document::default();

	document.apply( add( Kind::Wire, FOO, BAR ) ).unwrap();
	document.apply( add( Kind::Wire, BAR, BAZ ) ).unwrap();
	document.apply( add( Kind::Ground, BAZ, BAZ ) ).unwrap();

	assert_eq!( document.elements_at( Point { x : 2, y : 0 } ), vec![ 0 ] );
	assert_eq!( document.elements_at( BAR ), vec![ 1, 0 ] );
	assert_eq!( document.elements_at( BAZ ), vec![ 2, 1 ] );
	assert!( document.elements_at( Point { x : 9, y : 9 } ).is_empty() );
}
