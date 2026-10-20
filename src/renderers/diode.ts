import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as line from './line.ts';


const HALF = 6;
const WIDTH = 8;
const BODY = '#d0d0d0';


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
	const baseX = middleX - alongX * HALF;
	const baseY = middleY - alongY * HALF;
	const tipX = middleX + alongX * HALF;
	const tipY = middleY + alongY * HALF;

	context.lineCap = 'butt';

	line.draw( context, color( element.potentials[ 0 ], scale.voltage ), 2, fromX, fromY, baseX, baseY );
	line.draw( context, color( element.potentials[ 1 ], scale.voltage ), 2, tipX, tipY, toX, toY );

	context.fillStyle = BODY;
	context.beginPath();
	context.moveTo( baseX + alongY * WIDTH, baseY - alongX * WIDTH );
	context.lineTo( baseX - alongY * WIDTH, baseY + alongX * WIDTH );
	context.lineTo( tipX, tipY );
	context.closePath();
	context.fill();

	line.draw( context, BODY, 2, tipX + alongY * WIDTH, tipY - alongX * WIDTH, tipX - alongY * WIDTH, tipY + alongX * WIDTH );
}
