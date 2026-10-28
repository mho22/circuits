import type { Element } from '../types/element.ts';


function take( data : Float64Array, start : number, end : number ) : number[]
{
	const values : number[] = [];

	for( let index = start; index < end; index++ ) values.push( data[ index ] );

	return values;
}


export function decode( data : Float64Array ) : Pick<Element, 'potentials' | 'currents'>[]
{
	const readings : Pick<Element, 'potentials' | 'currents'>[] = [];

	for( let index = 0; index < data.length; )
	{
		const count = data[ index ];
		const start = index + 1;

		readings.push( { potentials : take( data, start, start + count ), currents : take( data, start + count, start + 2 * count ) } );

		index = start + 2 * count;
	}

	return readings;
}
