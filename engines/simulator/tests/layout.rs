use simulator::{ flatten, Document, Drive, Kind, Layout, Machine, Model, Operation, Plan, Point, Registry, Segment, DELAY };


type Part = ( Kind, ( i32, i32 ), ( i32, i32 ) );


fn layer( parts : &[ Part ] ) -> Document
{
	let mut document = Document::default();

	for &( kind, from, to ) in parts { document.apply( Operation::Add { kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }, mirrored : false } ).unwrap(); }

	document
}

fn symbol( layer : u32, y : i32 ) -> Part
{
	( Kind::Symbol { layer, rotation : 0 }, ( 10, y ), ( 10, y ) )
}

fn gate() -> Document
{
	layer( &[
		( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		( Kind::Input { high : false }, ( -1, 2 ), ( 0, 2 ) ),
		( Kind::Output, ( 6, 0 ), ( 5, 0 ) ),
	] )
}

fn model() -> Model
{
	Model { rows : vec![ vec![ Drive { volts : 0.0, ohms : 1e-3 } ]; 4 ], lights : vec![ Vec::new(); 4 ], next : vec![ 0; 4 ], delay : DELAY }
}

fn groups( parts : &[ Part ] ) -> Vec<( usize, usize )>
{
	let layers = [ layer( parts ), layer( &[ symbol( 2, 0 ) ] ), gate() ];
	let models = [ None, None, Some( model() ) ];
	let ( elements, devices, netlist ) = flatten( &layers, &mut Registry::default(), &models, 0, &[] ).unwrap();
	let plan = Plan::build( &elements, &devices, &netlist );
	let layout = Layout::new( Machine { elements : &elements, devices : &devices, plan : &plan, models : &models } ).unwrap();

	layout.groups.iter().map( | group | ( group.first, group.lanes ) ).collect()
}


#[test]
fn it_puts_the_devices_of_copies_of_a_symbol_in_the_lanes_of_one_group()
{
	assert_eq!( groups( &[ symbol( 1, 0 ), symbol( 1, 10 ) ] ), vec![ ( 0, 2 ) ] );
}

#[test]
fn it_gives_a_device_outside_the_copies_a_group_of_its_own()
{
	assert_eq!( groups( &[ symbol( 1, 0 ), symbol( 2, 10 ), symbol( 1, 20 ) ] ), vec![ ( 0, 2 ), ( 2, 1 ) ] );
	assert_eq!( groups( &[ symbol( 1, 0 ) ] ), vec![ ( 0, 1 ) ] );
}

#[test]
fn it_splits_more_copies_than_lanes_into_groups()
{
	let parts : Vec<Part> = ( 0 .. 65 ).map( | copy | symbol( 1, 10 * copy ) ).collect();

	assert_eq!( groups( &parts ), vec![ ( 0, 64 ), ( 64, 1 ) ] );
}
