import { Figure } from './enums/figure.ts';
import { path } from './path.ts';
import type { Point } from './types/point.ts';
import type { Shape } from './types/shape.ts';


const REACH = 1;


function near( point : Point, from : Point, to : Point ) : boolean
{
	const dx = to.x - from.x;
	const dy = to.y - from.y;
	const length = dx * dx + dy * dy;
	const ratio = Math.max( 0, Math.min( 1, ( ( point.x - from.x ) * dx + ( point.y - from.y ) * dy ) / length ) );

	return Math.hypot( point.x - from.x - ratio * dx, point.y - from.y - ratio * dy ) <= REACH;
}


export function hit( shape : Shape, point : Point ) : boolean
{
	if( shape.figure === Figure.Circle ) return Math.abs( Math.hypot( point.x - shape.center.x, point.y - shape.center.y ) - shape.radius ) <= REACH;

	const points = path( shape );

	return points.slice( 1 ).some( ( end, index ) => near( point, points[ index ], end ) );
}
