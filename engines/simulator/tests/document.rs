use simulator::{ Document, Error, Operation, Point, Wire };


const FOO : Point = Point { x : 0, y : 0 };
const BAR : Point = Point { x : 4, y : 0 };
const BAZ : Point = Point { x : 4, y : 4 };


#[test]
fn it_adds_a_wire_with_a_new_id()
{
	let mut document = Document::default();

	assert_eq!( document.apply( Operation::AddWire { from : FOO, to : BAR } ), Ok( 0 ) );
	assert_eq!( document.apply( Operation::AddWire { from : BAR, to : BAZ } ), Ok( 1 ) );

	assert_eq!( document.wires(), &[ Wire { id : 0, from : FOO, to : BAR }, Wire { id : 1, from : BAR, to : BAZ } ] );
}

#[test]
fn it_rejects_a_wire_without_length()
{
	let mut document = Document::default();

	assert_eq!( document.apply( Operation::AddWire { from : FOO, to : FOO } ), Err( Error::Empty ) );
	assert!( document.wires().is_empty() );
}

#[test]
fn it_rejects_a_crooked_wire()
{
	let mut document = Document::default();

	assert_eq!( document.apply( Operation::AddWire { from : FOO, to : Point { x : 2, y : 1 } } ), Err( Error::Crooked ) );
	assert!( document.wires().is_empty() );
}

#[test]
fn it_deletes_a_wire()
{
	let mut document = Document::default();

	document.apply( Operation::AddWire { from : FOO, to : BAR } ).unwrap();
	document.apply( Operation::AddWire { from : BAR, to : BAZ } ).unwrap();

	assert_eq!( document.apply( Operation::DeleteWire { id : 0 } ), Ok( 0 ) );
	assert_eq!( document.wires(), &[ Wire { id : 1, from : BAR, to : BAZ } ] );
}

#[test]
fn it_never_reuses_a_deleted_id()
{
	let mut document = Document::default();

	document.apply( Operation::AddWire { from : FOO, to : BAR } ).unwrap();
	document.apply( Operation::DeleteWire { id : 0 } ).unwrap();

	assert_eq!( document.apply( Operation::AddWire { from : FOO, to : BAR } ), Ok( 1 ) );
}

#[test]
fn it_rejects_deleting_a_missing_wire()
{
	let mut document = Document::default();

	assert_eq!( document.apply( Operation::DeleteWire { id : 7 } ), Err( Error::Missing ) );
}

#[test]
fn it_finds_the_topmost_wire_at_a_point()
{
	let mut document = Document::default();

	document.apply( Operation::AddWire { from : FOO, to : BAR } ).unwrap();
	document.apply( Operation::AddWire { from : BAR, to : BAZ } ).unwrap();

	assert_eq!( document.wire_at( Point { x : 2, y : 0 } ), Some( 0 ) );
	assert_eq!( document.wire_at( BAR ), Some( 1 ) );
	assert_eq!( document.wire_at( Point { x : 9, y : 9 } ), None );
}
