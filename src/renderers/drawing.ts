import { arc } from '../arc.ts';
import { Figure } from '../enums/figure.ts';
import { path } from '../path.ts';
import { turn } from '../turn.ts';
import type { Point } from '../types/point.ts';
import type { Shape } from '../types/shape.ts';


const COLOR = '#9e9e9e';
const FONT = '12px ui-monospace, monospace';


export function draw( context : CanvasRenderingContext2D, shapes : Shape[], center : Point, rotation : number, unit : number ) : void
{
	const at = ( point : Point ) : [ number, number ] => { const turned = turn( point, rotation ); return [ center.x + turned.x * unit, center.y + turned.y * unit ]; };

	context.strokeStyle = COLOR;
	context.lineWidth = 2;
	context.beginPath();

	shapes.forEach( shape =>
	{
		path( shape ).forEach( ( point, index ) => index === 0 ? context.moveTo( ... at( point ) ) : context.lineTo( ... at( point ) ) );

		if( shape.figure === Figure.Curve )
		{
			const [ from, through, to ] = [ shape.from, shape.through, shape.to ].map( point => { const [ x, y ] = at( point ); return { x, y }; } );
			const { center, radius, start, end, counterclockwise } = arc( from, through, to )!;

			context.moveTo( from.x, from.y );
			context.arc( center.x, center.y, radius, start, end, counterclockwise );

			return;
		}

		if( shape.figure !== Figure.Circle ) return;

		const [ x, y ] = at( shape.center );

		context.moveTo( x + shape.radius * unit, y );
		context.arc( x, y, shape.radius * unit, 0, 2 * Math.PI );
	} );

	context.stroke();

	const texts = shapes.flatMap( shape => shape.figure === Figure.Text ? [ shape ] : [] );

	if( texts.length === 0 ) return;

	context.fillStyle = COLOR;
	context.font = FONT;
	context.textAlign = 'left';
	context.textBaseline = 'alphabetic';

	texts.forEach( shape =>
	{
		context.save();
		context.translate( ... at( shape.at ) );
		context.rotate( rotation * Math.PI / 2 );
		context.fillText( shape.content, context.measureText( shape.content ).actualBoundingBoxLeft, 0 );
		context.restore();
	} );
}
