import { Figure } from './enums/figure.ts';
import type { Point } from './types/point.ts';
import type { Shape } from './types/shape.ts';


export function shift( shape : Shape, offset : Point ) : Shape
{
	const move = ( point : Point ) => ( { x : point.x + offset.x, y : point.y + offset.y } );

	if( shape.figure === Figure.Circle ) return { ... shape, center : move( shape.center ) };

	if( shape.figure === Figure.Curve ) return { ... shape, from : move( shape.from ), through : move( shape.through ), to : move( shape.to ) };

	return { ... shape, from : move( shape.from ), to : move( shape.to ) };
}
