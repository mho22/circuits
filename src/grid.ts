import type { Point } from './types/point.ts';


export const SPACING = 20;
export const FINE = 4;
export const HALF = FINE / 2;


export function toGrid( x : number, y : number ) : Point
{
	return { x : Math.round( x / SPACING ), y : Math.round( y / SPACING ) };
}

export function snap( point : Point ) : Point
{
	return { x : Math.round( point.x / HALF ) * HALF, y : Math.round( point.y / HALF ) * HALF };
}

export function toScreen( value : number ) : number
{
	return value * SPACING;
}
