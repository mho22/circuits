import type { Point } from './types/point.ts';


export function channel( from : Point, to : Point ) : [ Point, Point ]
{
	const dx = Math.sign( to.x - from.x );
	const dy = Math.sign( to.y - from.y );

	return [ { x : to.x + dy, y : to.y - dx }, { x : to.x - dy, y : to.y + dx } ];
}
