import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import { position, WIDTH } from '../slider.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as capsule from './capsule.ts';


const KNOB = 3;


function knob( context : CanvasRenderingContext2D, element : Element, scale : Scale, tint : string ) : void
{
	if( element.listening && Number.isNaN( scale.voltage ) ) return;

	const volts = element.listening ? element.potentials[ 1 ] : element.value;

	context.fillStyle = tint;
	context.beginPath();
	context.arc( position( element.from, volts ), toScreen( element.from.y ), KNOB, 0, 2 * Math.PI );
	context.fill();
}


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const tint = color( element.potentials[ 1 ], scale.voltage );

	capsule.draw( context, element, tint, WIDTH );

	knob( context, element, scale, tint );

	capsule.caption( context, element, WIDTH );
}
