import { bit } from '../bit.ts';
import { OFFSET } from '../caption.ts';
import { color } from '../colors/voltage.ts';
import { Kind } from '../enums/kind.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';


const HALF = 7;
const RADIUS = 4;
const TEXT = '#8a8a8a';
const PRESSED = '#000000';


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const x = toScreen( element.from.x );
	const y = toScreen( element.from.y );
	const dx = Math.sign( element.to.x - element.from.x );
	const dy = Math.sign( element.to.y - element.from.y );
	const tint = color( element.potentials[ 0 ], scale.voltage );

	context.strokeStyle = tint;
	context.fillStyle = tint;
	context.lineWidth = 2;
	context.lineCap = 'butt';
	context.beginPath();
	context.moveTo( x + dx * HALF, y + dy * HALF );
	context.lineTo( toScreen( element.to.x ), toScreen( element.to.y ) );
	context.stroke();

	context.beginPath();
	context.roundRect( x - HALF, y - HALF, 2 * HALF, 2 * HALF, element.kind === Kind.Output ? RADIUS : 0 );

	if( element.kind === Kind.Input ) context.fill();

	context.stroke();

	context.fillStyle = element.kind === Kind.Input ? PRESSED : tint;
	context.font = '11px ui-monospace, monospace';
	context.textAlign = 'center';
	context.textBaseline = 'middle';

	const text = String( bit( element.potentials[ 0 ] ) );
	const metrics = context.measureText( text );

	context.fillText( text, x + ( metrics.actualBoundingBoxLeft - metrics.actualBoundingBoxRight ) / 2, y + ( metrics.actualBoundingBoxAscent - metrics.actualBoundingBoxDescent ) / 2 );

	if( ! element.label ) return;

	context.fillStyle = TEXT;
	context.textAlign = dx > 0 ? 'right' : dx < 0 ? 'left' : 'center';
	context.textBaseline = dy > 0 ? 'bottom' : dy < 0 ? 'top' : 'middle';
	context.fillText( element.label, x - dx * OFFSET, y - dy * OFFSET );
}
