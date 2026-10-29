use simulator::{ Characterization, Document, Kind, Model, Operation, Outline, Point, Segment, DELAY, FLOOR };


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

fn inverter( extra : &[ Part ] ) -> Document
{
	layer( &[ &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Output, ( 6, 0 ), ( 5, 0 ) ) ], gate( 0 ).as_slice(), extra ].concat() )
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

fn nand_4() -> Document
{
	let parts : [ Part; 47 ] = [
		( Kind::Pmos, ( 26, 10 ), ( 27, 10 ) ),
		( Kind::Pmos, ( 29, 10 ), ( 30, 10 ) ),
		( Kind::Nmos, ( 29, 14 ), ( 30, 14 ) ),
		( Kind::Nmos, ( 29, 17 ), ( 30, 17 ) ),
		( Kind::Wire, ( 28, 9 ), ( 28, 8 ) ),
		( Kind::Wire, ( 31, 8 ), ( 31, 9 ) ),
		( Kind::Wire, ( 31, 11 ), ( 31, 12 ) ),
		( Kind::Wire, ( 28, 11 ), ( 28, 12 ) ),
		( Kind::Wire, ( 31, 15 ), ( 31, 16 ) ),
		( Kind::Wire, ( 31, 18 ), ( 31, 19 ) ),
		( Kind::Pmos, ( 23, 10 ), ( 24, 10 ) ),
		( Kind::Wire, ( 25, 9 ), ( 25, 8 ) ),
		( Kind::Wire, ( 25, 11 ), ( 25, 12 ) ),
		( Kind::Wire, ( 25, 8 ), ( 28, 8 ) ),
		( Kind::Wire, ( 25, 12 ), ( 28, 12 ) ),
		( Kind::Wire, ( 28, 8 ), ( 31, 8 ) ),
		( Kind::Wire, ( 31, 12 ), ( 28, 12 ) ),
		( Kind::Wire, ( 30, 12 ), ( 31, 12 ) ),
		( Kind::Wire, ( 31, 12 ), ( 31, 13 ) ),
		( Kind::Wire, ( 29, 10 ), ( 29, 14 ) ),
		( Kind::Wire, ( 26, 10 ), ( 26, 17 ) ),
		( Kind::Wire, ( 29, 17 ), ( 26, 17 ) ),
		( Kind::Wire, ( 23, 10 ), ( 23, 20 ) ),
		( Kind::Wire, ( 23, 20 ), ( 29, 20 ) ),
		( Kind::Ground, ( 31, 25 ), ( 31, 25 ) ),
		( Kind::Nmos, ( 29, 20 ), ( 30, 20 ) ),
		( Kind::Input { high : false }, ( 18, 17 ), ( 19, 17 ) ),
		( Kind::Input { high : false }, ( 18, 20 ), ( 19, 20 ) ),
		( Kind::Supply { volts : 5.0 }, ( 28, 8 ), ( 28, 8 ) ),
		( Kind::Input { high : false }, ( 18, 14 ), ( 19, 14 ) ),
		( Kind::Wire, ( 31, 12 ), ( 32, 12 ) ),
		( Kind::Output, ( 33, 12 ), ( 32, 12 ) ),
		( Kind::Wire, ( 25, 8 ), ( 22, 8 ) ),
		( Kind::Wire, ( 22, 9 ), ( 22, 8 ) ),
		( Kind::Wire, ( 22, 11 ), ( 22, 12 ) ),
		( Kind::Wire, ( 22, 12 ), ( 25, 12 ) ),
		( Kind::Wire, ( 29, 14 ), ( 19, 14 ) ),
		( Kind::Wire, ( 26, 17 ), ( 19, 17 ) ),
		( Kind::Wire, ( 23, 20 ), ( 19, 20 ) ),
		( Kind::Nmos, ( 29, 23 ), ( 30, 23 ) ),
		( Kind::Pmos, ( 20, 10 ), ( 21, 10 ) ),
		( Kind::Wire, ( 20, 10 ), ( 20, 23 ) ),
		( Kind::Wire, ( 19, 23 ), ( 20, 23 ) ),
		( Kind::Wire, ( 20, 23 ), ( 29, 23 ) ),
		( Kind::Wire, ( 31, 21 ), ( 31, 22 ) ),
		( Kind::Wire, ( 31, 24 ), ( 31, 25 ) ),
		( Kind::Input { high : false }, ( 18, 23 ), ( 19, 23 ) ),
	];
	let mut document = Document::default();

	for ( kind, from, to ) in parts { document.apply( Operation::Add { kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }, mirrored : kind == Kind::Pmos } ).unwrap(); }

	document
}

fn flip_flop() -> Document
{
	let input = | gate : i32, side : i32 | ( 10 * gate - 2, side );
	let output = | gate : i32 | ( 10 * gate + 2, 0 );
	let nets = [
		vec![ ( -20, 0 ), input( 1, -1 ) ],
		vec![ ( -16, 0 ), input( 0, -1 ), input( 0, 1 ), input( 5, 1 ), input( 6, 1 ) ],
		vec![ output( 0 ), input( 1, 1 ), input( 2, 1 ) ],
		vec![ output( 1 ), input( 2, -1 ), input( 3, -1 ) ],
		vec![ output( 2 ), input( 4, -1 ) ],
		vec![ output( 3 ), input( 4, 1 ), input( 5, -1 ) ],
		vec![ output( 4 ), input( 3, 1 ) ],
		vec![ output( 5 ), input( 6, -1 ), input( 7, -1 ) ],
		vec![ output( 6 ), input( 8, -1 ) ],
		vec![ output( 7 ), input( 8, 1 ), ( 100, 0 ) ],
		vec![ output( 8 ), input( 7, 1 ), ( 104, 0 ) ],
	];
	let pins = [
		( Kind::Input { high : false }, ( -21, 0 ), ( -20, 0 ) ),
		( Kind::Input { high : false }, ( -17, 0 ), ( -16, 0 ) ),
		( Kind::Output, ( 101, 0 ), ( 100, 0 ) ),
		( Kind::Output, ( 105, 0 ), ( 104, 0 ) ),
	];
	let gates = ( 0 .. 9 ).map( | gate | ( Kind::Symbol { layer : 1, rotation : 0 }, ( 10 * gate, 0 ), ( 10 * gate, 0 ) ) );
	let wires = nets.iter().zip( 3 .. ).flat_map( | ( net, lane ) |
	{
		let stubs = net.iter().flat_map( move | &( x, side ) | match side
		{
			0 => vec![ ( Kind::Wire, ( x, - lane ), ( x, 0 ) ), ( Kind::Wire, ( x, 0 ), ( x, lane ) ) ],
			_ => vec![ ( Kind::Wire, ( x, side ), ( x, side.signum() * lane ) ) ],
		} );
		let span = | y : i32, sign : i32 |
		{
			let mut columns : Vec<i32> = net.iter().filter( | &&( _, side ) | side * sign >= 0 ).map( | &( x, _ ) | x ).collect();

			columns.sort();

			columns.windows( 2 ).map( | pair | ( Kind::Wire, ( pair[ 0 ], y ), ( pair[ 1 ], y ) ) ).collect::<Vec<_>>()
		};

		stubs.chain( span( - lane, -1 ) ).chain( span( lane, 1 ) ).collect::<Vec<Part>>()
	} );

	layer( &pins.into_iter().chain( gates ).chain( wires ).collect::<Vec<Part>>() )
}

fn wrapper( inner : &Document ) -> Document
{
	let symbol = ( Kind::Symbol { layer : 2, rotation : 0 }, ( 0, 0 ), ( 0, 0 ) );
	let pins = inner.elements().iter().filter( | element | element.kind.pin() ).zip( Outline::of( inner ).terminals( Point { x : 0, y : 0 }, 0 ) ).map( | ( pin, end ) | match pin.kind
	{
		Kind::Output => ( Kind::Output, ( end.x + 1, end.y ), ( end.x, end.y ) ),
		kind => ( kind, ( end.x - 1, end.y ), ( end.x, end.y ) ),
	} );

	layer( &pins.chain( [ symbol ] ).collect::<Vec<Part>>() )
}

fn characterize( layers : &[ Document ], models : &[ Option<Model> ], layer : usize ) -> Option<Model>
{
	let mut characterization = Characterization::new( layers, models, layer, STEP )?;

	while ! characterization.done() { characterization.advance( models )?; }

	Some( characterization.model() )
}

fn walk( model : &Model, rows : &[ usize ] ) -> Vec<bool>
{
	let width = 4;
	let mut state = 0;

	rows.iter().map( | &row |
	{
		let entry = state * width + row;

		state = model.next[ entry ];

		model.rows[ entry ][ 0 ].high()
	} ).collect()
}


#[test]
fn it_measures_the_truth_table_of_an_inverter()
{
	let model = characterize( &[ Document::default(), inverter( &[] ) ], &[], 1 ).unwrap();

	assert_eq!( model.rows.len(), 2 );
	assert_eq!( model.rows.iter().map( | row | row.iter().map( | drive | drive.high() ).collect::<Vec<_>>() ).collect::<Vec<_>>(), vec![ vec![ true ], vec![ false ] ] );
	assert!( ( model.rows[ 0 ][ 0 ].volts - 5.0 ).abs() < 1e-3 && model.rows[ 1 ][ 0 ].volts.abs() < 1e-3 );
	assert!( model.rows.iter().all( | row | row[ 0 ].ohms > FLOOR && row[ 0 ].ohms < 1e6 ) );
	assert_eq!( model.next, vec![ 0; 2 ] );
	assert_eq!( model.delay, DELAY );
}

#[test]
fn it_measures_the_truth_table_of_a_four_input_nand_with_stacked_transistors()
{
	let model = characterize( &[ Document::default(), nand_4() ], &[], 1 ).unwrap();

	assert_eq!( model.rows.len(), 16 );
	assert_eq!( model.rows.iter().map( | row | row[ 0 ].high() ).collect::<Vec<_>>(), ( 0 .. 16 ).map( | row | row != 15 ).collect::<Vec<_>>() );
}

#[test]
fn it_measures_a_flip_flop_built_from_gates_as_a_gate_that_changes_its_state_on_a_rising_clock()
{
	let layers = [ Document::default(), nand(), flip_flop() ];
	let models = [ None, characterize( &layers, &[], 1 ) ];
	let model = characterize( &layers, &models, 2 ).unwrap();

	assert!( model.rows.len() > 4 );
	assert_eq!( model.next.len(), model.rows.len() );
	assert!( model.rows.iter().all( | row | row[ 0 ].high() != row[ 1 ].high() && row.iter().all( | drive | drive.ohms < 1e6 ) ) );
	assert_eq!( walk( &model, &[ 0, 1, 3, 2, 0, 1, 0, 2 ] ), vec![ false, false, true, true, true, true, true, false ] );
	assert!( ( model.delay - 3.0 * DELAY ).abs() < 1e-15 );
}

#[test]
fn it_keeps_the_state_of_a_flip_flop_inside_a_layer_without_feedback_of_its_own()
{
	let layers = [ Document::default(), nand(), flip_flop(), wrapper( &flip_flop() ) ];
	let gates = [ None, characterize( &layers, &[], 1 ) ];
	let models = [ None, gates[ 1 ].clone(), characterize( &layers, &gates, 2 ) ];
	let model = characterize( &layers, &models, 3 ).unwrap();

	assert_eq!( walk( &model, &[ 0, 1, 3, 2, 0, 1, 0, 2 ] ), vec![ false, false, true, true, true, true, true, false ] );
}

#[test]
fn it_leaves_a_layer_without_a_transistor_or_a_gate_analog()
{
	let resistor = layer( &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 0, 0 ), ( 4, 0 ) ), ( Kind::Output, ( 5, 0 ), ( 4, 0 ) ) ] );

	assert_eq!( characterize( &[ Document::default(), resistor ], &[], 1 ), None );
}

#[test]
fn it_leaves_a_layer_with_a_capacitor_an_inductor_or_a_clock_analog()
{
	let capacitor = inverter( &[ ( Kind::Capacitor { farads : 1e-6 }, ( 5, 1 ), ( 5, 3 ) ), ( Kind::Ground, ( 5, 3 ), ( 5, 3 ) ) ] );
	let inductor = inverter( &[ ( Kind::Inductor { henries : 1.0 }, ( 5, 1 ), ( 5, 3 ) ) ] );
	let clock = inverter( &[ ( Kind::Clock { hertz : 1.0, on : true }, ( 8, 4 ), ( 8, 6 ) ) ] );

	assert_eq!( characterize( &[ Document::default(), capacitor ], &[], 1 ), None );
	assert_eq!( characterize( &[ Document::default(), inductor ], &[], 1 ), None );
	assert_eq!( characterize( &[ Document::default(), clock ], &[], 1 ), None );
}

#[test]
fn it_measures_a_layer_with_an_led_between_two_pins_as_a_gate()
{
	let led = inverter( &[ ( Kind::Resistor { ohms : 1000.0 }, ( 5, 0 ), ( 9, 0 ) ), ( Kind::Led { color : 0 }, ( 9, 0 ), ( 9, 4 ) ), ( Kind::Wire, ( 9, 4 ), ( 9, 6 ) ), ( Kind::Wire, ( 9, 6 ), ( -2, 6 ) ), ( Kind::Wire, ( -2, 6 ), ( -2, 0 ) ), ( Kind::Wire, ( -2, 0 ), ( 0, 0 ) ) ] );
	let model = characterize( &[ Document::default(), led ], &[], 1 ).unwrap();

	assert_eq!( model.rows.iter().map( | row | row[ 0 ].high() ).collect::<Vec<_>>(), vec![ true, false ] );
}

#[test]
fn it_leaves_a_layer_with_an_led_on_ground_analog()
{
	let led = inverter( &[ ( Kind::Led { color : 0 }, ( 5, 1 ), ( 5, 3 ) ), ( Kind::Resistor { ohms : 1000.0 }, ( 5, 3 ), ( 5, 5 ) ), ( Kind::Ground, ( 5, 5 ), ( 5, 5 ) ) ] );

	assert_eq!( characterize( &[ Document::default(), led ], &[], 1 ), None );
}

#[test]
fn it_leaves_a_layer_with_feedback_analog()
{
	let ring = layer( &[ &[
		( Kind::Input { high : false }, ( -3, 0 ), ( -2, 0 ) ),
		( Kind::Output, ( 7, -2 ), ( 7, 0 ) ),
		( Kind::Wire, ( 5, 0 ), ( 10, 0 ) ),
		( Kind::Wire, ( 15, 0 ), ( 15, 5 ) ),
		( Kind::Wire, ( 15, 5 ), ( -2, 5 ) ),
		( Kind::Wire, ( -2, 5 ), ( -2, 0 ) ),
		( Kind::Wire, ( -2, 0 ), ( 0, 0 ) ),
	][ .. ], gate( 0 ).as_slice(), gate( 10 ).as_slice() ].concat() );

	assert_eq!( characterize( &[ Document::default(), ring ], &[], 1 ), None );
}

#[test]
fn it_leaves_a_layer_without_an_output_or_with_more_than_eight_inputs_analog()
{
	let silent = layer( &[ &[ ( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ) ][ .. ], gate( 0 ).as_slice() ].concat() );
	let inputs : Vec<Part> = ( 0 .. 9 ).map( | y | ( Kind::Input { high : false }, ( -20, y * 2 ), ( -19, y * 2 ) ) ).collect();

	assert_eq!( characterize( &[ Document::default(), silent ], &[], 1 ), None );
	assert_eq!( characterize( &[ Document::default(), inverter( &inputs ) ], &[], 1 ), None );
}

#[test]
fn it_leaves_an_output_on_the_net_of_an_input_analog()
{
	assert_eq!( characterize( &[ Document::default(), inverter( &[ ( Kind::Output, ( 0, 4 ), ( 0, 2 ) ) ] ) ], &[], 1 ), None );
}

#[test]
fn it_measures_one_input_combination_at_each_step()
{
	let layers = [ Document::default(), inverter( &[] ) ];
	let mut characterization = Characterization::new( &layers, &[], 1, STEP ).unwrap();
	let mut progress = vec![ characterization.progress() ];

	while ! characterization.done() { characterization.advance( &[] ).unwrap(); progress.push( characterization.progress() ); }

	assert_eq!( progress, vec![ 0.0, 0.5, 1.0 ] );
	assert_eq!( Some( characterization.model() ), characterize( &layers, &[], 1 ) );
}

#[test]
fn it_flips_each_input_of_each_combination_of_each_merged_state_after_the_combinations_of_a_layer_with_gates()
{
	let layers = [ Document::default(), nand(), flip_flop() ];
	let models = [ None, characterize( &layers, &[], 1 ) ];
	let mut characterization = Characterization::new( &layers, &models, 2, STEP ).unwrap();
	let mut steps = 0;

	while ! characterization.done() { characterization.advance( &models ).unwrap(); steps += 1; }

	assert_eq!( ( steps, characterization.model().rows.len(), characterization.progress() ), ( 9 * 4 + 4 * 4 * 2, 4 * 4, 1.0 ) );
	assert_eq!( Some( characterization.model() ), characterize( &layers, &models, 2 ) );
}

#[test]
fn it_leaves_a_missing_layer_without_a_model()
{
	assert_eq!( characterize( &[ Document::default() ], &[], 3 ), None );
}
