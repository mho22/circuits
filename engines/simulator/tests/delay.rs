use simulator::{ characterize, Document, Kind, Operation, Point, Segment, DELAY };


const STEP : f64 = 1e-6;


type Part = ( Kind, ( i32, i32 ), ( i32, i32 ) );


fn layer( parts : &[ Part ] ) -> Document
{
	let mut document = Document::default();

	for &( kind, from, to ) in parts { document.apply( Operation::Add { kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }, mirrored : false } ).unwrap(); }

	document
}

fn gate( x : i32 ) -> Vec<Part>
{
	vec![
		( Kind::Supply { volts : 5.0 }, ( x + 3, -1 ), ( x + 3, -1 ) ),
		( Kind::Wire, ( x, -2 ), ( x, 0 ) ),
		( Kind::Wire, ( x, 0 ), ( x, 2 ) ),
		( Kind::Pmos, ( x, -2 ), ( x + 2, -2 ) ),
		( Kind::Nmos, ( x, 2 ), ( x + 2, 2 ) ),
		( Kind::Wire, ( x + 3, -3 ), ( x + 5, -3 ) ),
		( Kind::Wire, ( x + 5, -3 ), ( x + 5, 0 ) ),
		( Kind::Wire, ( x + 5, 0 ), ( x + 5, 1 ) ),
		( Kind::Wire, ( x + 5, 1 ), ( x + 3, 1 ) ),
		( Kind::Ground, ( x + 3, 3 ), ( x + 3, 3 ) ),
	]
}

fn inverters( count : i32 ) -> Document
{
	let stages = ( 0 .. count ).flat_map( | index | gate( 10 * index ) );
	let links = ( 1 .. count ).map( | index | ( Kind::Wire, ( 10 * index - 5, 0 ), ( 10 * index, 0 ) ) );
	let pins = [ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Output, ( 10 * count - 4, 0 ), ( 10 * count - 5, 0 ) ) ];

	layer( &pins.into_iter().chain( stages ).chain( links ).collect::<Vec<Part>>() )
}

fn nand() -> Document
{
	layer( &[
		( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		( Kind::Input { high : false }, ( -3, 4 ), ( -2, 4 ) ),
		( Kind::Output, ( 12, 0 ), ( 11, 0 ) ),
		( Kind::Supply { volts : 5.0 }, ( 3, -1 ), ( 3, -1 ) ),
		( Kind::Supply { volts : 5.0 }, ( 9, -1 ), ( 9, -1 ) ),
		( Kind::Pmos, ( 0, -2 ), ( 2, -2 ) ),
		( Kind::Pmos, ( 6, -2 ), ( 8, -2 ) ),
		( Kind::Nmos, ( 0, 2 ), ( 2, 2 ) ),
		( Kind::Nmos, ( 0, 6 ), ( 2, 6 ) ),
		( Kind::Ground, ( 3, 7 ), ( 3, 7 ) ),
		( Kind::Wire, ( 0, -2 ), ( 0, 0 ) ),
		( Kind::Wire, ( 0, 0 ), ( 0, 2 ) ),
		( Kind::Wire, ( 6, -2 ), ( 6, 4 ) ),
		( Kind::Wire, ( 6, 4 ), ( -2, 4 ) ),
		( Kind::Wire, ( -2, 4 ), ( -2, 6 ) ),
		( Kind::Wire, ( -2, 6 ), ( 0, 6 ) ),
		( Kind::Wire, ( 3, -3 ), ( 9, -3 ) ),
		( Kind::Wire, ( 9, -3 ), ( 11, -3 ) ),
		( Kind::Wire, ( 11, -3 ), ( 11, 0 ) ),
		( Kind::Wire, ( 11, 0 ), ( 11, 1 ) ),
		( Kind::Wire, ( 11, 1 ), ( 3, 1 ) ),
		( Kind::Wire, ( 3, 3 ), ( 3, 5 ) ),
	] )
}

fn and() -> Document
{
	layer( &[
		( Kind::Input { high : false }, ( -3, -1 ), ( -2, -1 ) ),
		( Kind::Input { high : false }, ( -3, 1 ), ( -2, 1 ) ),
		( Kind::Symbol { layer : 1, rotation : 0 }, ( 0, 0 ), ( 0, 0 ) ),
		( Kind::Wire, ( 2, 0 ), ( 6, 0 ) ),
		( Kind::Wire, ( 6, -1 ), ( 6, 0 ) ),
		( Kind::Wire, ( 6, 0 ), ( 6, 1 ) ),
		( Kind::Wire, ( 6, -1 ), ( 8, -1 ) ),
		( Kind::Wire, ( 6, 1 ), ( 8, 1 ) ),
		( Kind::Symbol { layer : 1, rotation : 0 }, ( 10, 0 ), ( 10, 0 ) ),
		( Kind::Output, ( 13, 0 ), ( 12, 0 ) ),
	] )
}

fn delay( layers : &[ Document ], layer : usize ) -> f64
{
	let models : Vec<_> = ( 0 .. layer ).map( | inner | characterize( layers, &[], inner, STEP ) ).collect();

	characterize( layers, &models, layer, STEP ).unwrap().delay
}


#[test]
fn it_gives_one_delay_to_each_stage_of_transistors_between_an_input_and_an_output()
{
	assert_eq!( [ 1, 2 ].map( | count | delay( &[ Document::default(), inverters( count ) ], 1 ) ), [ DELAY, 2.0 * DELAY ] );
}

#[test]
fn it_counts_transistors_joined_by_their_channels_as_one_stage()
{
	assert_eq!( delay( &[ Document::default(), nand() ], 1 ), DELAY );
}

#[test]
fn it_adds_the_delays_of_the_gates_on_the_longest_path()
{
	assert_eq!( delay( &[ Document::default(), nand(), and() ], 2 ), 2.0 * DELAY );
}
