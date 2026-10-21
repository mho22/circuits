import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';


export const WIDTH = 11;
export const HALF = 6;
const BACKGROUND = '#000000';


export function draw( context : CanvasRenderingContext2D, element : Element, tint : string ) : void
{
	const x = toScreen( element.from.x );
	const y = toScreen( element.from.y );
	const dx = Math.sign( element.to.x - element.from.x );
	const dy = Math.sign( element.to.y - element.from.y );

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
}
