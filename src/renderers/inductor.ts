import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as label from './label.ts';


const BODY = 40;
const LOOPS = 4;


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const fromX = toScreen( element.from.x );
	const fromY = toScreen( element.from.y );
	const toX = toScreen( element.to.x );
	const toY = toScreen( element.to.y );
	const length = Math.hypot( toX - fromX, toY - fromY );
	const alongX = ( toX - fromX ) / length;
	const alongY = ( toY - fromY ) / length;
	const angle = Math.atan2( alongY, alongX );
	const body = Math.min( BODY, length );
	const start = ( length - body ) / 2;
	const radius = body / LOOPS / 2;

	const gradient = context.createLinearGradient( fromX, fromY, toX, toY );

	gradient.addColorStop( 0, color( element.potentials[ 0 ], scale.voltage ) );
	gradient.addColorStop( 1, color( element.potentials[ 1 ], scale.voltage ) );

	context.strokeStyle = gradient;
	context.lineWidth = 2;
	context.lineCap = 'butt';
	context.beginPath();
	context.moveTo( fromX, fromY );
	context.lineTo( fromX + alongX * start, fromY + alongY * start );

	for( let loop = 0; loop < LOOPS; loop++ )
	{
		const along = start + radius * ( 2 * loop + 1 );

		context.arc( fromX + alongX * along, fromY + alongY * along, radius, angle + Math.PI, angle );
	}

	context.lineTo( toX, toY );
	context.stroke();

	label.draw( context, element );
}
