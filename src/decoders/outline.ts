import type { Outline } from '../types/outline.ts';
import { decode as layout } from './layout.ts';


export function decode( data : Int32Array ) : Outline
{
	const [ lowX = 0, lowY = 0, highX = 0, highY = 0, count = 0 ] = data;
	const rest = data.subarray( 5 + 2 * count );
	const lights = Array.from( { length : Math.floor( rest.length / 3 ) }, ( _, index ) => ( { point : { x : rest[ 3 * index ], y : rest[ 3 * index + 1 ] }, color : rest[ 3 * index + 2 ] } ) );

	return { low : { x : lowX, y : lowY }, high : { x : highX, y : highY }, pins : layout( data.subarray( 5, 5 + 2 * count ) ), lights };
}
