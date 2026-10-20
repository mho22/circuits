import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as label from './label.ts';


const GAP = 4;
const LONG = 12;
const SHORT = 6;
const PLATE = '#d0d0d0';


function line( context : CanvasRenderingContext2D, style : string, width : number, fromX : number, fromY : number, toX : number, toY : number ) : void
{
	context.strokeStyle = style;
	context.lineWidth = width;
	context.beginPath();
	context.moveTo( fromX, fromY );
	context.lineTo( toX, toY );
	context.stroke();
}


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const fromX = toScreen( element.from.x );
	const fromY = toScreen( element.from.y );
	const toX = toScreen( element.to.x );
	const toY = toScreen( element.to.y );
	const length = Math.hypot( toX - fromX, toY - fromY );
	const alongX = ( toX - fromX ) / length;
	const alongY = ( toY - fromY ) / length;
	const middleX = ( fromX + toX ) / 2;
	const middleY = ( fromY + toY ) / 2;
	const negativeX = middleX - alongX * GAP;
	const negativeY = middleY - alongY * GAP;
	const positiveX = middleX + alongX * GAP;
	const positiveY = middleY + alongY * GAP;

	context.lineCap = 'butt';

	line( context, color( element.potentials[ 0 ], scale.voltage ), 2, fromX, fromY, negativeX, negativeY );
	line( context, color( element.potentials[ 1 ], scale.voltage ), 2, positiveX, positiveY, toX, toY );
	line( context, PLATE, 4, negativeX + alongY * SHORT, negativeY - alongX * SHORT, negativeX - alongY * SHORT, negativeY + alongX * SHORT );
	line( context, PLATE, 2, positiveX + alongY * LONG, positiveY - alongX * LONG, positiveX - alongY * LONG, positiveY + alongX * LONG );

	label.draw( context, element );
}
