import { FINE, SPACING, toScreen } from './grid.ts';
import type { Board } from './types/board.ts';
import type { Point } from './types/point.ts';


export function fit( center : Point, left : number, top : number ) : Board
{
	return { x : toScreen( center.x ) - left, y : toScreen( center.y ) - top, unit : SPACING / FINE };
}

export function locate( board : Board, x : number, y : number ) : Point
{
	return { x : Math.round( ( x - board.x ) / board.unit ), y : Math.round( ( y - board.y ) / board.unit ) };
}
