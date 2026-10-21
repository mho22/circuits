import type { Outline } from '../types/outline.ts';
import { decode as layout } from './layout.ts';


export function decode( data : Int32Array ) : Outline
{
	const [ lowX = 0, lowY = 0, highX = 0, highY = 0 ] = data;

	return { low : { x : lowX, y : lowY }, high : { x : highX, y : highY }, pins : layout( data.subarray( 4 ) ) };
}
