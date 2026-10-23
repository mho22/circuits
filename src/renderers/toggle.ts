import { bit } from '../bit.ts';
import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as capsule from './capsule.ts';


const KNOB = 3;
const GAP = 8;
const TEXT = '#8a8a8a';


function knob( context : CanvasRenderingContext2D, element : Element, scale : Scale, tint : string ) : void
{
	if( element.listening && Number.isNaN( scale.voltage ) ) return;

	const high = element.listening ? bit( element.potentials[ 1 ] ) : element.value;
	const side = high ? 1 : -1;

	context.fillStyle = tint;
	context.beginPath();
	context.arc( toScreen( element.from.x ) + side * ( capsule.WIDTH - capsule.HALF ), toScreen( element.from.y ), KNOB, 0, 2 * Math.PI );
	context.fill();
}


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const tint = color( element.potentials[ 1 ], scale.voltage );

	capsule.draw( context, element, tint );

	knob( context, element, scale, tint );

	if( ! element.label ) return;

	const dx = Math.sign( element.to.x - element.from.x );
	const dy = Math.sign( element.to.y - element.from.y );

	context.fillStyle = TEXT;
	context.font = '12px ui-monospace, monospace';
	context.textAlign = dx > 0 ? 'right' : dx < 0 ? 'left' : 'center';
	context.textBaseline = dy > 0 ? 'bottom' : dy < 0 ? 'top' : 'middle';
	context.fillText( element.label, toScreen( element.from.x ) - dx * ( capsule.WIDTH + GAP ), toScreen( element.from.y ) - dy * ( capsule.HALF + GAP ) );
}
