import { UNITS } from '../enums/kind.ts';
import { toScreen } from '../grid.ts';
import { format } from '../si.ts';
import type { Element } from '../types/element.ts';


const COLOR = '#8a8a8a';
const OFFSET = 14;
const SIDEWAYS = 0.3;


export function draw( context : CanvasRenderingContext2D, element : Element ) : void
{
	const unit = UNITS[ element.kind ];

	if( ! unit ) return;

	const dx = element.to.x - element.from.x;
	const dy = element.to.y - element.from.y;
	const length = Math.hypot( dx, dy );
	const normalX = length ? dy / length : 0;
	const normalY = length ? - dx / length : -1;

	context.fillStyle = COLOR;
	context.font = '11px ui-monospace, monospace';
	context.textAlign = normalX > SIDEWAYS ? 'left' : normalX < - SIDEWAYS ? 'right' : 'center';
	context.textBaseline = normalY > SIDEWAYS ? 'top' : normalY < - SIDEWAYS ? 'bottom' : 'middle';
	context.fillText( format( element.value, unit ), toScreen( ( element.from.x + element.to.x ) / 2 ) + normalX * OFFSET, toScreen( ( element.from.y + element.to.y ) / 2 ) + normalY * OFFSET );
}
