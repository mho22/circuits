import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';


export const WIDTH = 11;
export const HALF = 6;
const BACKGROUND = '#000000';
const GAP = 8;
const TEXT = '#8a8a8a';


export function draw( context : CanvasRenderingContext2D, element : Element, tint : string, width = WIDTH ) : void
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
	context.moveTo( x + dx * width, y + dy * HALF );
	context.lineTo( toScreen( element.to.x ), toScreen( element.to.y ) );
	context.stroke();

	context.beginPath();
	context.roundRect( x - width, y - HALF, 2 * width, 2 * HALF, HALF );
	context.fill();
	context.stroke();
}

export function caption( context : CanvasRenderingContext2D, element : Element, width = WIDTH ) : void
{
	if( ! element.label ) return;

	const dx = Math.sign( element.to.x - element.from.x );
	const dy = Math.sign( element.to.y - element.from.y );

	context.fillStyle = TEXT;
	context.font = '12px ui-monospace, monospace';
	context.textAlign = dx > 0 ? 'right' : dx < 0 ? 'left' : 'center';
	context.textBaseline = dy > 0 ? 'bottom' : dy < 0 ? 'top' : 'middle';
	context.fillText( element.label, toScreen( element.from.x ) - dx * ( width + GAP ), toScreen( element.from.y ) - dy * ( HALF + GAP ) );
}
