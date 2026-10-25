import { SPACING, toScreen } from './grid.ts';
import { HALF, WIDTH as BODY } from './renderers/capsule.ts';
import type { Point } from './types/point.ts';


export const WIDTH = BODY + SPACING / 2;

const HIGH = 5;
const STEPS = 50;
const TRAVEL = WIDTH - HALF;


export function position( from : Point, volts : number ) : number
{
	return toScreen( from.x ) + ( 2 * Math.min( HIGH, Math.max( 0, volts ) ) / HIGH - 1 ) * TRAVEL;
}

export function level( from : Point, x : number ) : number
{
	const ratio = Math.min( 1, Math.max( 0, ( x - toScreen( from.x ) + TRAVEL ) / ( 2 * TRAVEL ) ) );

	return Math.round( ratio * STEPS ) * HIGH / STEPS;
}

export function covers( from : Point, point : Point ) : boolean
{
	return Math.abs( point.x - toScreen( from.x ) ) <= WIDTH && Math.abs( point.y - toScreen( from.y ) ) <= HALF;
}
