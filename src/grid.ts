import type { Point } from './types/point.ts';


export const SPACING = 20;


export function toGrid( x : number, y : number ) : Point
{
	return { x : Math.round( x / SPACING ), y : Math.round( y / SPACING ) };
}

export function toScreen( value : number ) : number
{
	return value * SPACING;
}
