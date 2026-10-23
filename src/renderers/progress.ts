import { toScreen } from '../grid.ts';
import type { Point } from '../types/point.ts';
import * as current from './current.ts';
import * as line from './line.ts';


const EMPTY = '#1d1d1d';
const FULL = '#4b4b4b';
const DOT = '#656565';
const WIDTH = 2;
const THICK = 3;


export function draw( context : CanvasRenderingContext2D, from : Point, to : Point, progress : number ) : void
{
	const end = { x : from.x + ( to.x - from.x ) * progress, y : from.y + ( to.y - from.y ) * progress };

	context.lineCap = 'round';

	line.draw( context, EMPTY, WIDTH, toScreen( from.x ), toScreen( from.y ), toScreen( to.x ), toScreen( to.y ) );
	line.draw( context, FULL, THICK, toScreen( from.x ), toScreen( from.y ), toScreen( end.x ), toScreen( end.y ) );

	current.draw( context, from, end, 1, 1, 0, DOT );
}
