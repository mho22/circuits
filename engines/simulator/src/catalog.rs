use crate::{ characterize, Document, Element, Kind, Model };


fn source( elements : &[ Element ] ) -> Vec<Element>
{
	elements.iter().map( | element | match element.kind { Kind::Input { .. } => Element { kind : Kind::Input { high : false }, ..*element }, _ => *element } ).collect()
}


struct Catalog<'a>
{
	layers : &'a [ Document ],
	step : f64,
	models : Vec<Option<Model>>,
	sources : Vec<Vec<Element>>,
	states : Vec<u8>,
	changed : Vec<bool>,
}

impl Catalog<'_>
{
	fn visit( &mut self, layer : usize )
	{
		if self.states[ layer ] != 0 { return; }

		self.states[ layer ] = 1;

		let elements = self.layers[ layer ].elements();
		let source = source( elements );
		let children : Vec<usize> = elements.iter().filter_map( | element | match element.kind { Kind::Symbol { layer, .. } => Some( layer as usize ), _ => None } ).filter( | &child | child > 0 && child < self.layers.len() ).collect();

		for &child in &children { self.visit( child ); }

		self.changed[ layer ] = self.sources[ layer ] != source || children.iter().any( | &child | self.changed[ child ] );

		if self.changed[ layer ]
		{
			self.models[ layer ] = characterize( self.layers, &self.models, layer, self.step );
			self.sources[ layer ] = source;
		}

		self.states[ layer ] = 2;
	}
}


pub fn catalog( layers : &[ Document ], models : &[ Option<Model> ], sources : &[ Vec<Element> ], step : f64 ) -> ( Vec<Option<Model>>, Vec<Vec<Element>> )
{
	let count = layers.len();
	let mut models = models.to_vec();
	let mut sources = sources.to_vec();

	models.resize( count, None );
	sources.resize( count, Vec::new() );

	let mut catalog = Catalog { layers, step, models, sources, states : vec![ 0; count ], changed : vec![ false; count ] };

	for layer in 1 .. count { catalog.visit( layer ); }

	( catalog.models, catalog.sources )
}
