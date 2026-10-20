import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as label from './label.ts';


const BODY = 40;
const AMPLITUDE = 6;
const PEAKS = 6;


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const fromX = toScreen( element.from.x );
	const fromY = toScreen( element.from.y );
	const toX = toScreen( element.to.x );
	const toY = toScreen( element.to.y );
	const length = Math.hypot( toX - fromX, toY - fromY );
	const alongX = ( toX - fromX ) / length;
	const alongY = ( toY - fromY ) / length;
	const body = Math.min( BODY, length );
	const start = ( length - body ) / 2;

	const gradient = context.createLinearGradient( fromX, fromY, toX, toY );

	gradient.addColorStop( 0, color( element.potentials[ 0 ], scale.voltage ) );
	gradient.addColorStop( 1, color( element.potentials[ 1 ], scale.voltage ) );

	context.strokeStyle = gradient;
	context.lineWidth = 2;
	context.lineJoin = 'round';
	context.beginPath();
	context.moveTo( fromX, fromY );
	context.lineTo( fromX + alongX * start, fromY + alongY * start );

	for( let peak = 0; peak < PEAKS; peak++ )
	{
		const along = start + body * ( peak + 0.5 ) / PEAKS;
		const side = peak % 2 === 0 ? AMPLITUDE : - AMPLITUDE;

		context.lineTo( fromX + alongX * along - alongY * side, fromY + alongY * along + alongX * side );
	}

	context.lineTo( fromX + alongX * ( start + body ), fromY + alongY * ( start + body ) );
	context.lineTo( toX, toY );
	context.stroke();

	label.draw( context, element );
}
