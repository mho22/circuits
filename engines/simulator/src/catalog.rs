use crate::characterize::INPUTS;
use crate::{ Characterization, Document, Element, Kind, Model };


fn source( elements : &[ Element ] ) -> Vec<Element>
{
	elements.iter().map( | element | match element.kind { Kind::Input { .. } => Element { kind : Kind::Input { high : false }, ..*element }, _ => *element } ).collect()
}

fn weight( elements : &[ Element ] ) -> f64
{
	let inputs = elements.iter().filter( | element | element.kind.pin() && element.kind != Kind::Output ).count().min( INPUTS );
	let bits = if elements.iter().any( | element | matches!( element.kind, Kind::Symbol { .. } ) ) { inputs } else { 0 };

	elements.len() as f64 * ( 1 << inputs ) as f64 * ( 1 + bits ) as f64
}


#[derive( Debug )]
pub struct Catalog
{
	models : Vec<Option<Model>>,
	sources : Vec<Vec<Element>>,
	order : Vec<usize>,
	weights : Vec<f64>,
	states : Vec<u8>,
	changed : Vec<bool>,
	index : usize,
	current : Option<Characterization>,
	step : f64,
}

impl Catalog
{
	pub fn new( layers : &[ Document ], models : &[ Option<Model> ], sources : &[ Vec<Element> ], step : f64 ) -> Catalog
	{
		let count = layers.len();
		let mut models = models.to_vec();
		let mut sources = sources.to_vec();

		models.resize( count, None );
		sources.resize( count, Vec::new() );

		let mut catalog = Catalog { models, sources, order : Vec::new(), weights : Vec::new(), states : vec![ 0; count ], changed : vec![ false; count ], index : 0, current : None, step };

		for layer in 1 .. count { catalog.visit( layers, layer ); }

		catalog
	}

	fn visit( &mut self, layers : &[ Document ], layer : usize )
	{
		if self.states[ layer ] != 0 { return; }

		self.states[ layer ] = 1;

		let elements = layers[ layer ].elements();
		let source = source( elements );
		let children : Vec<usize> = elements.iter().filter_map( | element | match element.kind { Kind::Symbol { layer, .. } => Some( layer as usize ), _ => None } ).filter( | &child | child > 0 && child < layers.len() ).collect();

		for &child in &children { self.visit( layers, child ); }

		self.changed[ layer ] = self.sources[ layer ] != source || children.iter().any( | &child | self.changed[ child ] );

		if self.changed[ layer ]
		{
			self.order.push( layer );
			self.weights.push( weight( elements ) );
			self.sources[ layer ] = source;
		}

		self.states[ layer ] = 2;
	}

	fn layer( &self ) -> Option<usize>
	{
		self.order.get( self.index ).copied()
	}

	pub fn progress( &self ) -> f64
	{
		let total : f64 = self.weights.iter().sum();

		if total == 0.0 { return 1.0; }

		let done : f64 = self.weights[ .. self.index ].iter().sum();
		let part = self.current.as_ref().map_or( 0.0, | characterization | self.weights[ self.index ] * characterization.progress() );

		( done + part ) / total
	}

	pub fn advance( &mut self, layers : &[ Document ] ) -> bool
	{
		let Some( layer ) = self.layer() else { return false; };

		let Some( characterization ) = &mut self.current else
		{
			self.current = Characterization::new( layers, &self.models, layer, self.step );

			if self.current.is_none() { self.models[ layer ] = None; self.index += 1; }

			return true;
		};

		let model = match characterization.done()
		{
			true => Some( characterization.model() ),
			false => match characterization.advance( &self.models ) { Some( () ) => return true, None => None },
		};

		self.models[ layer ] = model;
		self.current = None;
		self.index += 1;

		true
	}

	pub fn finish( self ) -> ( Vec<Option<Model>>, Vec<Vec<Element>> )
	{
		( self.models, self.sources )
	}
}


pub fn catalog( layers : &[ Document ], models : &[ Option<Model> ], sources : &[ Vec<Element> ], step : f64 ) -> ( Vec<Option<Model>>, Vec<Vec<Element>> )
{
	let mut catalog = Catalog::new( layers, models, sources, step );

	while catalog.advance( layers ) {}

	catalog.finish()
}
