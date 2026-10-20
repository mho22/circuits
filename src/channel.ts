import type { Point } from './types/point.ts';


export function channel( from : Point, to : Point, mirrored : boolean ) : [ Point, Point ]
{
	const dx = Math.sign( to.x - from.x );
	const dy = Math.sign( to.y - from.y );
	const left = { x : to.x + dy, y : to.y - dx };
	const right = { x : to.x - dy, y : to.y + dx };

	return mirrored ? [ right, left ] : [ left, right ];
}
