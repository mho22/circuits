import { TRANSISTORS } from '../enums/kind.ts';
import type { Element } from '../types/element.ts';


const HEAD = 9;


export function decode( data : Float64Array ) : Element[]
{
	const elements : Element[] = [];

	for( let index = 0; index < data.length; )
	{
		const kind = data[ index + 1 ];
		const value = data[ index + 6 ];
		const count = data[ index + 8 ];
		const start = index + HEAD;
		const potentials : number[] = [];
		const currents : number[] = [];

		for( let offset = 0; offset < count; offset++ )
		{
			potentials.push( data[ start + offset ] );
			currents.push( data[ start + count + offset ] );
		}

		elements.push( { id : data[ index ], kind, from : { x : data[ index + 2 ], y : data[ index + 3 ] }, to : { x : data[ index + 4 ], y : data[ index + 5 ] }, value, mirrored : TRANSISTORS.includes( kind ) && value === 1, rotation : data[ index + 7 ], potentials, currents } );

		index += HEAD + 2 * count;
	}

	return elements;
}
