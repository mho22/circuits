use simulator::{ Catalog, Document, Drive, Element, Kind, Model, Operation, Point, Segment, DELAY };


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
		( Kind::Wire, ( 0, -2 ), ( 0, 0 ) ),
		( Kind::Wire, ( 0, 0 ), ( 0, 2 ) ),
		( Kind::Pmos, ( 0, -2 ), ( 2, -2 ) ),
		( Kind::Nmos, ( 0, 2 ), ( 2, 2 ) ),
		( Kind::Wire, ( 3, -3 ), ( 5, -3 ) ),
		( Kind::Wire, ( 5, -3 ), ( 5, 0 ) ),
		( Kind::Wire, ( 5, 0 ), ( 5, 1 ) ),
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

fn measure( layers : &[ Document ], models : &[ Option<Model> ], sources : &[ Vec<Element> ] ) -> ( Vec<Option<Model>>, Vec<Vec<Element>> )
{
	let mut catalog = Catalog::new( layers, models, sources, STEP );

	while catalog.advance( layers ) {}

	catalog.finish()
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
	let ( models, sources ) = measure( &[ Document::default(), inverter(), buffer() ], &[], &[] );

	assert_eq!( models[ 0 ], None );
	assert_eq!( highs( &models[ 1 ] ), vec![ true, false ] );
	assert_eq!( highs( &models[ 2 ] ), vec![ false, true ] );
	assert_eq!( sources[ 1 ], inverter().elements().to_vec() );
}

#[test]
fn it_keeps_the_model_of_an_unchanged_layer()
{
	let layers = [ Document::default(), inverter() ];
	let ( models, _ ) = measure( &layers, &[ None, Some( fake() ) ], &[ Vec::new(), inverter().elements().to_vec() ] );

	assert_eq!( models[ 1 ], Some( fake() ) );
}

#[test]
fn it_keeps_the_model_of_a_layer_whose_input_changes_level()
{
	let mut changed = inverter();
	let id = changed.elements()[ 0 ].id;

	changed.apply( Operation::SetHigh { id, high : true } ).unwrap();

	let ( models, _ ) = measure( &[ Document::default(), changed ], &[ None, Some( fake() ) ], &[ Vec::new(), inverter().elements().to_vec() ] );

	assert_eq!( models[ 1 ], Some( fake() ) );
}

#[test]
fn it_adds_the_delays_of_the_layers_inside_a_layer()
{
	let ( models, _ ) = measure( &[ Document::default(), inverter(), buffer() ], &[], &[] );

	assert_eq!( [ 1, 2 ].map( | layer | models[ layer ].as_ref().unwrap().delay ), [ DELAY, 2.0 * DELAY ] );
}

#[test]
fn it_measures_a_layer_again_when_a_layer_inside_it_changes()
{
	let layers = [ Document::default(), inverter(), buffer() ];
	let sources : Vec<Vec<Element>> = vec![ Vec::new(), Vec::new(), buffer().elements().to_vec() ];
	let ( models, _ ) = measure( &layers, &[ None, None, Some( fake() ) ], &sources );

	assert_eq!( highs( &models[ 2 ] ), vec![ false, true ] );
}

#[test]
fn it_measures_one_step_at_a_time_from_the_layers_inside_a_layer()
{
	let layers = [ Document::default(), inverter(), buffer() ];
	let mut stepped = Catalog::new( &layers, &[], &[], STEP );
	let mut steps = vec![ stepped.progress() ];

	while stepped.advance( &layers ) { steps.push( stepped.progress() ); }

	assert!( steps.windows( 2 ).all( | pair | pair[ 0 ] <= pair[ 1 ] ) );
	assert_eq!( ( steps[ 0 ], steps[ steps.len() - 1 ] ), ( 0.0, 1.0 ) );
	assert_eq!( highs( &stepped.finish().0[ 2 ] ), vec![ false, true ] );
}

#[test]
fn it_has_nothing_to_measure_when_no_layer_changed()
{
	let layers = [ Document::default(), inverter() ];
	let mut stepped = Catalog::new( &layers, &[ None, Some( fake() ) ], &[ Vec::new(), inverter().elements().to_vec() ], STEP );

	assert_eq!( ( stepped.progress(), stepped.advance( &layers ) ), ( 1.0, false ) );
}

#[test]
fn it_gives_no_model_to_a_changed_layer_until_it_is_measured()
{
	let layers = [ Document::default(), inverter(), buffer() ];
	let mut stepped = Catalog::new( &layers, &[ None, Some( fake() ), Some( fake() ) ], &[ Vec::new(), inverter().elements().to_vec(), Vec::new() ], STEP );

	assert_eq!( ( stepped.done(), stepped.ready() ), ( false, vec![ None, Some( fake() ), None ] ) );

	while stepped.advance( &layers ) {}

	assert!( stepped.done() );
	assert_eq!( stepped.ready(), stepped.finish().0 );
}

#[test]
fn it_waits_for_a_changed_layer_until_it_is_measured()
{
	let layers = [ Document::default(), inverter(), buffer() ];
	let mut stepped = Catalog::new( &layers, &[ None, Some( fake() ), Some( fake() ) ], &[ Vec::new(), inverter().elements().to_vec(), Vec::new() ], STEP );

	assert_eq!( [ 1, 2 ].map( | layer | stepped.waits( layer ) ), [ false, true ] );

	while stepped.advance( &layers ) {}

	assert!( ! stepped.waits( 2 ) );
}
