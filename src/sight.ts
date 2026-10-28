import type { Area } from './types/area.ts';
import type { Point } from './types/point.ts';


const MARGIN = 4;


export function sight( low : Point, high : Point ) : Area
{
	return { low : { x : Math.floor( low.x ) - MARGIN, y : Math.floor( low.y ) - MARGIN }, high : { x : Math.ceil( high.x ) + MARGIN, y : Math.ceil( high.y ) + MARGIN } };
}

export function seen( area : Area, from : Point, to : Point ) : boolean
{
	return Math.max( from.x, to.x ) >= area.low.x && Math.min( from.x, to.x ) <= area.high.x && Math.max( from.y, to.y ) >= area.low.y && Math.min( from.y, to.y ) <= area.high.y;
}
