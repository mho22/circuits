use simulator::{ catalog, Document, Drive, Element, Kind, Model, Operation, Point, Segment, DELAY };


const STEP : f64 = 1e-6;


type Part = ( Kind, ( i32, i32 ), ( i32, i32 ) );


fn layer( parts : &[ Part ] ) -> Document
{
	let mut document = Document::default();

	for &( kind, from, to ) in parts { document.apply( Operation::Add { kind, segment : Segment { from : Point { x : from.0, y : from.1 }, to : Point { x : to.0, y : to.1 } }, mirrored : false } ).unwrap(); }

	document
}

fn inverter() -> Document
{
	layer( &[
		( Kind::Input { high : false }, ( -1, 0 ), ( 0, 0 ) ),
		( Kind::Output, ( 6, 0 ), ( 5, 0 ) ),
		( Kind::Supply { volts : 5.0 }, ( 3, -1 ), ( 3, -1 ) ),
		( Kind::Wire, ( 0, -2 ), ( 0, 2 ) ),
		( Kind::Pmos, ( 0, -2 ), ( 2, -2 ) ),
		( Kind::Nmos, ( 0, 2 ), ( 2, 2 ) ),
		( Kind::Wire, ( 3, -3 ), ( 5, -3 ) ),
		( Kind::Wire, ( 5, -3 ), ( 5, 1 ) ),
		( Kind::Wire, ( 5, 1 ), ( 3, 1 ) ),
		( Kind::Ground, ( 3, 3 ), ( 3, 3 ) ),
	] )
}

fn buffer() -> Document
{
	layer( &[
		( Kind::Input { high : false }, ( -3, 0 ), ( -2, 0 ) ),
		( Kind::Symbol { layer : 1, rotation : 0 }, ( 0, 0 ), ( 0, 0 ) ),
		( Kind::Symbol { layer : 1, rotation : 0 }, ( 4, 0 ), ( 4, 0 ) ),
		( Kind::Output, ( 7, 0 ), ( 6, 0 ) ),
	] )
}

fn fake() -> Model
{
	Model { rows : vec![ vec![ Drive { volts : 1.0, ohms : 1.0 } ]; 2 ], next : vec![ 0; 2 ], delay : DELAY }
}

fn highs( model : &Option<Model> ) -> Vec<bool>
{
	model.as_ref().unwrap().rows.iter().map( | row | row[ 0 ].high() ).collect()
}


#[test]
fn it_measures_each_layer_after_the_layers_inside_it()
{
	let ( models, sources ) = catalog( &[ Document::default(), inverter(), buffer() ], &[], &[], STEP );

	assert_eq!( models[ 0 ], None );
	assert_eq!( highs( &models[ 1 ] ), vec![ true, false ] );
	assert_eq!( highs( &models[ 2 ] ), vec![ false, true ] );
	assert_eq!( sources[ 1 ], inverter().elements().to_vec() );
}

#[test]
fn it_keeps_the_model_of_an_unchanged_layer()
{
	let layers = [ Document::default(), inverter() ];
	let ( models, _ ) = catalog( &layers, &[ None, Some( fake() ) ], &[ Vec::new(), inverter().elements().to_vec() ], STEP );

	assert_eq!( models[ 1 ], Some( fake() ) );
}

#[test]
fn it_measures_a_layer_again_when_its_delay_changes()
{
	let mut slow = inverter();

	slow.apply( Operation::Describe { name : String::new(), drawing : Vec::new(), layout : Vec::new(), delay : 5e-9 } ).unwrap();

	let ( fast, sources ) = catalog( &[ Document::default(), inverter() ], &[], &[], STEP );
	let ( models, _ ) = catalog( &[ Document::default(), slow ], &fast, &sources, STEP );

	assert_eq!( fast[ 1 ].as_ref().unwrap().delay, DELAY );

	assert_eq!( models[ 1 ].as_ref().unwrap().delay, 5e-9 );
}

#[test]
fn it_measures_a_layer_again_when_a_layer_inside_it_changes()
{
	let layers = [ Document::default(), inverter(), buffer() ];
	let sources : Vec<Vec<Element>> = vec![ Vec::new(), Vec::new(), buffer().elements().to_vec() ];
	let ( models, _ ) = catalog( &layers, &[ None, None, Some( fake() ) ], &sources, STEP );

	assert_eq!( highs( &models[ 2 ] ), vec![ false, true ] );
}
