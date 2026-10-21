import { Figure } from './enums/figure.ts';
import type { Point } from './types/point.ts';
import type { Shape } from './types/shape.ts';


export function path( shape : Shape ) : Point[]
{
	if( shape.figure === Figure.Circle ) return [];

	return [ shape.from, shape.to ];
}
