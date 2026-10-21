import { TRANSISTORS } from '../enums/kind.ts';
import type { Element } from '../types/element.ts';


const HEAD = 9;


export function decode( data : Float64Array ) : Element[]
{
	const elements : Element[] = [];

	for( let index = 0; index < data.length; )
	{
		const [ id, kind, fromX, fromY, toX, toY, value, rotation, count ] = data.subarray( index, index + HEAD );
		const readings = Array.from( data.subarray( index + HEAD, index + HEAD + 2 * count ) );
		const mirrored = TRANSISTORS.includes( kind ) && value === 1;

		elements.push( { id, kind, from : { x : fromX, y : fromY }, to : { x : toX, y : toY }, value, mirrored, rotation, potentials : readings.slice( 0, count ), currents : readings.slice( count ) } );

		index += HEAD + 2 * count;
	}

	return elements;
}
