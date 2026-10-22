import { SPACING } from './grid.ts';
import type { Element } from './types/element.ts';
import type { Point } from './types/point.ts';


export const OFFSET = 11;
const SIZE = 11;

const WIDTH = 0.6 * SIZE;


function extent( direction : number, length : number ) : number[]
{
	if( direction === 0 ) return [ - length / 2, length / 2 ];

	return [ - direction * OFFSET, - direction * ( OFFSET + length ) ];
}


export function caption( element : Element ) : Point[]
{
	if( ! element.label ) return [];

	const xs = extent( Math.sign( element.to.x - element.from.x ), element.label.length * WIDTH );
	const ys = extent( Math.sign( element.to.y - element.from.y ), SIZE );

	return [
		{ x : element.from.x + Math.floor( Math.min( ... xs ) / SPACING ), y : element.from.y + Math.floor( Math.min( ... ys ) / SPACING ) },
		{ x : element.from.x + Math.ceil( Math.max( ... xs ) / SPACING ), y : element.from.y + Math.ceil( Math.max( ... ys ) / SPACING ) },
	];
}
