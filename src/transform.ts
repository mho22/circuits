import { Figure } from './enums/figure.ts';
import type { Point } from './types/point.ts';
import type { Shape } from './types/shape.ts';


export function shift( shape : Shape, offset : Point ) : Shape
{
	const move = ( point : Point ) => ( { x : point.x + offset.x, y : point.y + offset.y } );

	if( shape.figure === Figure.Circle ) return { ... shape, center : move( shape.center ) };

	return { ... shape, from : move( shape.from ), to : move( shape.to ) };
}

export function rotate( shape : Shape ) : Shape
{
	if( shape.figure === Figure.Circle ) return shape;

	const center = { x : Math.round( ( shape.from.x + shape.to.x ) / 2 ), y : Math.round( ( shape.from.y + shape.to.y ) / 2 ) };
	const turn = ( point : Point ) => ( { x : center.x - point.y + center.y, y : center.y + point.x - center.x } );

	return { ... shape, from : turn( shape.from ), to : turn( shape.to ) };
}
