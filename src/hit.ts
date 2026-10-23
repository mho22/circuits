import { arc } from './arc.ts';
import { bounds } from './bounds.ts';
import { Figure } from './enums/figure.ts';
import { path } from './path.ts';
import { side } from './side.ts';
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

function ring( point : Point, center : Point, radius : number ) : boolean
{
	return Math.abs( Math.hypot( point.x - center.x, point.y - center.y ) - radius ) <= REACH;
}


export function hit( shape : Shape, point : Point ) : boolean
{
	if( shape.figure === Figure.Text ) { const [ low, high ] = bounds( shape ); return point.x >= low.x && point.x <= high.x && point.y >= low.y && point.y <= high.y; }

	if( shape.figure === Figure.Circle ) return ring( point, shape.center, shape.radius );

	if( shape.figure === Figure.Curve )
	{
		const { center, radius } = arc( shape.from, shape.through, shape.to )!;

		return ring( point, center, radius ) && side( point, shape.from, shape.to ) === side( shape.through, shape.from, shape.to );
	}

	const points = path( shape );

	return points.slice( 1 ).some( ( end, index ) => near( point, points[ index ], end ) );
}
