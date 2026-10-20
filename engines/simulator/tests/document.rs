use simulator::{ Document, Element, Error, Kind, Operation, Point, Segment };


const FOO : Point = Point { x : 0, y : 0 };
const BAR : Point = Point { x : 4, y : 0 };
const BAZ : Point = Point { x : 4, y : 4 };


fn add( kind : Kind, from : Point, to : Point ) -> Operation
{
	Operation::Add { kind, segment : Segment { from, to }, mirrored : false }
}


#[test]
fn it_adds_parts_with_new_ids()
{
	let mut document = Document::default();

	assert_eq!( document.apply( add( Kind::Wire, FOO, BAR ) ), Ok( 0 ) );
	assert_eq!( document.apply( add( Kind::Resistor { ohms : 1000.0 }, BAR, BAZ ) ), Ok( 1 ) );
	assert_eq!( document.apply( add( Kind::Ground, FOO, FOO ) ), Ok( 2 ) );

	assert_eq!( document.elements(), &[
		Element { id : 0, kind : Kind::Wire, segment : Segment { from : FOO, to : BAR }, mirrored : false },
		Element { id : 1, kind : Kind::Resistor { ohms : 1000.0 }, segment : Segment { from : BAR, to : BAZ }, mirrored : false },
		Element { id : 2, kind : Kind::Ground, segment : Segment { from : FOO, to : FOO }, mirrored : false },
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
fn it_puts_a_vcc_on_one_point()
{
	let mut document = Document::default();

	assert_eq!( document.apply( add( Kind::Supply { volts : 5.0 }, FOO, FOO ) ), Ok( 0 ) );
	assert_eq!( document.apply( add( Kind::Supply { volts : 5.0 }, FOO, BAR ) ), Err( Error::Misplaced ) );
}

#[test]
fn it_puts_a_pin_and_a_symbol_on_one_point()
{
	let mut document = Document::default();

	assert_eq!( document.apply( add( Kind::Pin, FOO, FOO ) ), Ok( 0 ) );
	assert_eq!( document.apply( add( Kind::Symbol { layer : 1, rotation : 0 }, BAR, BAR ) ), Ok( 1 ) );
	assert_eq!( document.apply( add( Kind::Pin, FOO, BAR ) ), Err( Error::Misplaced ) );
	assert_eq!( document.apply( add( Kind::Symbol { layer : 1, rotation : 0 }, FOO, BAR ) ), Err( Error::Misplaced ) );
	assert_eq!( document.apply( add( Kind::Symbol { layer : 1, rotation : 4 }, BAZ, BAZ ) ), Err( Error::Rotation ) );
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
	assert_eq!( document.elements(), &[ Element { id : 1, kind : Kind::Wire, segment : Segment { from : BAR, to : BAZ }, mirrored : false } ] );
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
	assert_eq!( document.apply( Operation::SetClosed { id : 7, closed : true } ), Err( Error::Missing ) );
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
fn it_closes_a_switch()
{
	let mut document = Document::default();

	document.apply( add( Kind::Switch { closed : false }, FOO, BAR ) ).unwrap();
	document.apply( add( Kind::Wire, BAR, BAZ ) ).unwrap();

	assert_eq!( document.apply( Operation::SetClosed { id : 0, closed : true } ), Ok( 0 ) );
	assert_eq!( document.apply( Operation::SetClosed { id : 1, closed : true } ), Err( Error::Switchless ) );
	assert_eq!( document.elements()[ 0 ].kind, Kind::Switch { closed : true } );
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

#[test]
fn it_finds_a_transistor_at_its_collector_and_its_emitter()
{
	let mut document = Document::default();

	document.apply( add( Kind::Npn, FOO, Point { x : 2, y : 0 } ) ).unwrap();

	assert_eq!( document.elements_at( Point { x : 1, y : 0 } ), vec![ 0 ] );
	assert_eq!( document.elements_at( Point { x : 3, y : -1 } ), vec![ 0 ] );
	assert_eq!( document.elements_at( Point { x : 3, y : 1 } ), vec![ 0 ] );
	assert!( document.elements_at( Point { x : 3, y : 0 } ).is_empty() );
}

#[test]
fn it_inserts_a_part_back_at_its_place_with_its_id()
{
	let mut document = Document::default();

	document.apply( add( Kind::Wire, FOO, BAR ) ).unwrap();
	document.apply( add( Kind::Wire, BAR, BAZ ) ).unwrap();
	document.apply( Operation::Delete { id : 0 } ).unwrap();

	let foo = Element { id : 0, kind : Kind::Wire, segment : Segment { from : FOO, to : BAR }, mirrored : false };

	assert_eq!( document.apply( Operation::Insert { element : foo, index : 0 } ), Ok( 0 ) );
	assert_eq!( document.elements()[ 0 ], foo );
	assert_eq!( document.apply( add( Kind::Wire, BAZ, Point { x : 0, y : 4 } ) ), Ok( 2 ) );
}

#[test]
fn it_continues_the_ids_after_an_inserted_part()
{
	let mut document = Document::default();

	document.apply( Operation::Insert { element : Element { id : 7, kind : Kind::Wire, segment : Segment { from : FOO, to : BAR }, mirrored : false }, index : 9 } ).unwrap();

	assert_eq!( document.apply( add( Kind::Wire, BAR, BAZ ) ), Ok( 8 ) );
}

#[test]
fn it_rejects_an_insert_with_a_taken_id_or_an_invalid_part()
{
	let mut document = Document::default();

	document.apply( add( Kind::Wire, FOO, BAR ) ).unwrap();

	assert_eq!( document.apply( Operation::Insert { element : Element { id : 0, kind : Kind::Wire, segment : Segment { from : BAR, to : BAZ }, mirrored : false }, index : 0 } ), Err( Error::Taken ) );
	assert_eq!( document.apply( Operation::Insert { element : Element { id : 1, kind : Kind::Wire, segment : Segment { from : BAR, to : BAR }, mirrored : false }, index : 0 } ), Err( Error::Empty ) );
	assert_eq!( document.elements().len(), 1 );
}

#[test]
fn it_gives_the_operation_that_cancels_each_operation()
{
	let mut document = Document::default();

	document.apply( add( Kind::Resistor { ohms : 1000.0 }, FOO, BAR ) ).unwrap();
	document.apply( add( Kind::Switch { closed : false }, BAR, BAZ ) ).unwrap();

	let resistor = document.elements()[ 0 ];

	assert_eq!( document.inverse( add( Kind::Wire, FOO, BAZ ) ), Ok( Operation::Delete { id : 2 } ) );
	assert_eq!( document.inverse( Operation::Insert { element : resistor, index : 0 } ), Ok( Operation::Delete { id : 0 } ) );
	assert_eq!( document.inverse( Operation::Delete { id : 0 } ), Ok( Operation::Insert { element : resistor, index : 0 } ) );
	assert_eq!( document.inverse( Operation::SetValue { id : 0, value : 220.0 } ), Ok( Operation::SetValue { id : 0, value : 1000.0 } ) );
	assert_eq!( document.inverse( Operation::SetClosed { id : 1, closed : true } ), Ok( Operation::SetClosed { id : 1, closed : false } ) );
}

#[test]
fn it_gives_no_inverse_for_an_operation_that_fails()
{
	let mut document = Document::default();

	document.apply( add( Kind::Wire, FOO, BAR ) ).unwrap();

	assert_eq!( document.inverse( Operation::Delete { id : 7 } ), Err( Error::Missing ) );
	assert_eq!( document.inverse( Operation::SetValue { id : 0, value : 1.0 } ), Err( Error::Valueless ) );
	assert_eq!( document.inverse( Operation::SetClosed { id : 0, closed : true } ), Err( Error::Switchless ) );
}

#[test]
fn it_moves_a_part_and_gives_the_move_back_as_its_inverse()
{
	let mut document = Document::default();

	document.apply( add( Kind::Npn, FOO, Point { x : 2, y : 0 } ) ).unwrap();

	let segment = Segment { from : BAR, to : Point { x : 6, y : 0 } };
	let inverse = document.inverse( Operation::Move { id : 0, kind : Kind::Npn, segment, mirrored : true } ).unwrap();

	assert_eq!( document.apply( Operation::Move { id : 0, kind : Kind::Npn, segment, mirrored : true } ), Ok( 0 ) );
	assert_eq!( document.elements()[ 0 ], Element { id : 0, kind : Kind::Npn, segment, mirrored : true } );
	assert_eq!( inverse, Operation::Move { id : 0, kind : Kind::Npn, segment : Segment { from : FOO, to : Point { x : 2, y : 0 } }, mirrored : false } );
}

#[test]
fn it_rejects_a_move_to_an_invalid_place()
{
	let mut document = Document::default();

	document.apply( add( Kind::Wire, FOO, BAR ) ).unwrap();
	document.apply( add( Kind::Ground, BAZ, BAZ ) ).unwrap();

	assert_eq!( document.apply( Operation::Move { id : 0, kind : Kind::Wire, segment : Segment { from : FOO, to : BAZ }, mirrored : false } ), Err( Error::Crooked ) );
	assert_eq!( document.apply( Operation::Move { id : 1, kind : Kind::Ground, segment : Segment { from : FOO, to : BAR }, mirrored : false } ), Err( Error::Misplaced ) );
	assert_eq!( document.apply( Operation::Move { id : 7, kind : Kind::Wire, segment : Segment { from : FOO, to : BAR }, mirrored : false } ), Err( Error::Missing ) );
	assert_eq!( document.elements()[ 0 ].segment, Segment { from : FOO, to : BAR } );
}

#[test]
fn it_changes_the_kind_of_a_moved_part_and_gives_it_back_as_its_inverse()
{
	let mut document = Document::default();

	document.apply( add( Kind::Symbol { layer : 1, rotation : 0 }, FOO, FOO ) ).unwrap();

	let turned = Operation::Move { id : 0, kind : Kind::Symbol { layer : 1, rotation : 1 }, segment : Segment { from : FOO, to : FOO }, mirrored : false };
	let inverse = document.inverse( turned ).unwrap();

	assert_eq!( document.apply( turned ), Ok( 0 ) );
	assert_eq!( document.elements()[ 0 ].kind, Kind::Symbol { layer : 1, rotation : 1 } );
	assert_eq!( inverse, Operation::Move { id : 0, kind : Kind::Symbol { layer : 1, rotation : 0 }, segment : Segment { from : FOO, to : FOO }, mirrored : false } );
}
