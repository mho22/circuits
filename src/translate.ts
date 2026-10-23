import { Figure } from './enums/figure.ts';
import type { Point } from './types/point.ts';
import type { Shape } from './types/shape.ts';


function shift( point : Point, offset : Point ) : Point
{
	return { x : point.x + offset.x, y : point.y + offset.y };
}


export function translate( shape : Shape, offset : Point ) : Shape
{
	if( shape.figure === Figure.Text ) return { ... shape, at : shift( shape.at, offset ) };

	if( shape.figure === Figure.Circle ) return { ... shape, center : shift( shape.center, offset ) };

	if( shape.figure === Figure.Curve ) return { ... shape, from : shift( shape.from, offset ), through : shift( shape.through, offset ), to : shift( shape.to, offset ) };

	return { ... shape, from : shift( shape.from, offset ), to : shift( shape.to, offset ) };
}
