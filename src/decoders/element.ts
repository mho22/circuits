import { TRANSISTORS } from '../enums/kind.ts';
import { terminals } from '../terminals.ts';
import type { Element } from '../types/element.ts';


const STRIDE = 13;


export function decode( data : Float64Array ) : Element[]
{
	const elements : Element[] = [];

	for( let index = 0; index < data.length; index += STRIDE )
	{
		const [ id, kind, fromX, fromY, toX, toY, value, ... rest ] = data.subarray( index, index + STRIDE );
		const from = { x : fromX, y : fromY };
		const to = { x : toX, y : toY };
		const mirrored = TRANSISTORS.includes( kind ) && value === 1;
		const count = terminals( kind, from, to, mirrored ).length;

		elements.push( { id, kind, from, to, value, mirrored, potentials : rest.slice( 0, count ), currents : rest.slice( 3, 3 + count ) } );
	}

	return elements;
}
