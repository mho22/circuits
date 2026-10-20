import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	context.strokeStyle = color( element.potentials[ 0 ], scale.voltage );
	context.lineWidth = 2;
	context.lineCap = 'round';
	context.beginPath();
	context.moveTo( toScreen( element.from.x ), toScreen( element.from.y ) );
	context.lineTo( toScreen( element.to.x ), toScreen( element.to.y ) );
	context.stroke();
}
