import { Figure } from '../enums/figure.ts';
import { path } from '../path.ts';
import { turn } from '../turn.ts';
import type { Point } from '../types/point.ts';
import type { Shape } from '../types/shape.ts';


const COLOR = '#9e9e9e';


export function draw( context : CanvasRenderingContext2D, shapes : Shape[], center : Point, rotation : number, unit : number ) : void
{
	const at = ( point : Point ) : [ number, number ] => { const turned = turn( point, rotation ); return [ center.x + turned.x * unit, center.y + turned.y * unit ]; };

	context.strokeStyle = COLOR;
	context.lineWidth = 2;
	context.beginPath();

	shapes.forEach( shape =>
	{
		path( shape ).forEach( ( point, index ) => index === 0 ? context.moveTo( ... at( point ) ) : context.lineTo( ... at( point ) ) );

		if( shape.figure !== Figure.Circle ) return;

		const [ x, y ] = at( shape.center );

		context.moveTo( x + shape.radius * unit, y );
		context.arc( x, y, shape.radius * unit, 0, 2 * Math.PI );
	} );

	context.stroke();
}
