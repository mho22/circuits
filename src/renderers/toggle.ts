import { bit } from '../bit.ts';
import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as capsule from './capsule.ts';


const KNOB = 3;


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const tint = color( element.potentials[ 1 ], scale.voltage );
	const high = element.listening ? bit( element.potentials[ 1 ] ) : element.value;
	const side = high ? 1 : -1;

	capsule.draw( context, element, tint );

	context.fillStyle = tint;
	context.beginPath();
	context.arc( toScreen( element.from.x ) + side * ( capsule.WIDTH - capsule.HALF ), toScreen( element.from.y ), KNOB, 0, 2 * Math.PI );
	context.fill();
}
