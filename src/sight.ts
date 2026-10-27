import type { Area } from './types/area.ts';
import type { Point } from './types/point.ts';


const MARGIN = 4;


export function seen( sight : Area, from : Point, to : Point ) : boolean
{
	return Math.max( from.x, to.x ) >= sight.low.x - MARGIN && Math.min( from.x, to.x ) <= sight.high.x + MARGIN && Math.max( from.y, to.y ) >= sight.low.y - MARGIN && Math.min( from.y, to.y ) <= sight.high.y + MARGIN;
}
