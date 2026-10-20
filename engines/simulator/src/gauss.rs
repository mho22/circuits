use crate::Error;


const TOLERANCE : f64 = 1e-12;


pub fn eliminate( mut matrix : Vec<f64>, mut rhs : Vec<f64> ) -> Result<Vec<f64>, Error>
{
	let size = rhs.len();
	let mut scales : Vec<f64> = ( 0 .. size ).map( | row | matrix[ row * size .. ( row + 1 ) * size ].iter().fold( 0.0_f64, | largest, value | largest.max( value.abs() ) ) ).collect();

	for column in 0 .. size
	{
		let pivot = ( column .. size ).max_by( | &a, &b | matrix[ a * size + column ].abs().total_cmp( &matrix[ b * size + column ].abs() ) ).unwrap();

		if matrix[ pivot * size + column ].abs() <= scales[ pivot ] * TOLERANCE { return Err( Error::Singular ); }

		for index in 0 .. size { matrix.swap( column * size + index, pivot * size + index ); }

		rhs.swap( column, pivot );
		scales.swap( column, pivot );

		for row in column + 1 .. size
		{
			let factor = matrix[ row * size + column ] / matrix[ column * size + column ];

			if factor == 0.0 { continue; }

			for index in column .. size { matrix[ row * size + index ] -= factor * matrix[ column * size + index ]; }

			rhs[ row ] -= factor * rhs[ column ];
		}
	}

	let mut solution = vec![ 0.0; size ];

	for row in ( 0 .. size ).rev()
	{
		let known : f64 = ( row + 1 .. size ).map( | index | matrix[ row * size + index ] * solution[ index ] ).sum();

		solution[ row ] = ( rhs[ row ] - known ) / matrix[ row * size + row ];
	}

	Ok( solution )
}
