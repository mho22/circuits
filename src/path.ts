import { Figure } from './enums/figure.ts';
import type { Point } from './types/point.ts';
import type { Shape } from './types/shape.ts';


export function path( shape : Shape ) : Point[]
{
	if( shape.figure === Figure.Circle ) return [];

	const { from, to } = shape;

	if( shape.figure === Figure.Line ) return [ from, to ];

	return [ from, { x : to.x, y : from.y }, to, { x : from.x, y : to.y }, from ];
}
