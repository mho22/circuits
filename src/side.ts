import type { Point } from './types/point.ts';


export function side( point : Point, from : Point, to : Point ) : number
{
	return Math.sign( ( to.x - from.x ) * ( point.y - from.y ) - ( to.y - from.y ) * ( point.x - from.x ) );
}
