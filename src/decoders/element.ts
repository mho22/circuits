import type { Element } from '../types/element.ts';


const STRIDE = 10;


export function decode( data : Float64Array ) : Element[]
{
	const elements : Element[] = [];

	for( let index = 0; index < data.length; index += STRIDE )
	{
		const [ id, kind, fromX, fromY, toX, toY, value, from, to, current ] = data.subarray( index, index + STRIDE );

		elements.push( { id, kind, from : { x : fromX, y : fromY }, to : { x : toX, y : toY }, value, potentials : [ from, to ], current } );
	}

	return elements;
}
