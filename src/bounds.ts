import { arc } from './arc.ts';
import { Figure } from './enums/figure.ts';
import { side } from './side.ts';
import type { Point } from './types/point.ts';
import type { Shape } from './types/shape.ts';


export function bounds( shape : Shape ) : Point[]
{
	if( shape.figure === Figure.Line ) return [ shape.from, shape.to ];

	if( shape.figure === Figure.Circle ) return [ { x : shape.center.x - shape.radius, y : shape.center.y - shape.radius }, { x : shape.center.x + shape.radius, y : shape.center.y + shape.radius } ];

	const { center, radius } = arc( shape.from, shape.through, shape.to )!;
	const extremes = [ { x : center.x - radius, y : center.y }, { x : center.x + radius, y : center.y }, { x : center.x, y : center.y - radius }, { x : center.x, y : center.y + radius } ].map( point => ( { x : Math.round( point.x ), y : Math.round( point.y ) } ) );
	const bulge = side( shape.through, shape.from, shape.to );

	return [ shape.from, shape.through, shape.to, ... extremes.filter( point => side( point, shape.from, shape.to ) === bulge ) ];
}
