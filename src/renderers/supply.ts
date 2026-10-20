import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as label from './label.ts';


const LEAD = 10;
const HALF = 8;


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const x = toScreen( element.from.x );
	const y = toScreen( element.from.y );

	context.strokeStyle = color( element.potentials[ 1 ], scale.voltage );
	context.lineWidth = 2;
	context.lineCap = 'butt';
	context.beginPath();
	context.moveTo( x, y );
	context.lineTo( x, y - LEAD );
	context.moveTo( x - HALF, y - LEAD );
	context.lineTo( x + HALF, y - LEAD );
	context.stroke();

	label.draw( context, element );
}
