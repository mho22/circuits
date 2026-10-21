import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';


const WIDTH = 11;
const HALF = 6;
const KNOB = 3;
const BACKGROUND = '#000000';


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const x = toScreen( element.from.x );
	const y = toScreen( element.from.y );
	const dx = Math.sign( element.to.x - element.from.x );
	const dy = Math.sign( element.to.y - element.from.y );
	const tint = color( element.potentials[ 1 ], scale.voltage );
	const side = element.value ? 1 : -1;

	context.strokeStyle = tint;
	context.fillStyle = BACKGROUND;
	context.lineWidth = 2;
	context.lineCap = 'butt';
	context.beginPath();
	context.moveTo( x + dx * WIDTH, y + dy * HALF );
	context.lineTo( toScreen( element.to.x ), toScreen( element.to.y ) );
	context.stroke();

	context.beginPath();
	context.roundRect( x - WIDTH, y - HALF, 2 * WIDTH, 2 * HALF, HALF );
	context.fill();
	context.stroke();

	context.fillStyle = tint;
	context.beginPath();
	context.arc( x + side * ( WIDTH - HALF ), y, KNOB, 0, 2 * Math.PI );
	context.fill();
}
