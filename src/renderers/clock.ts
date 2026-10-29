import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as capsule from './capsule.ts';
import * as label from './label.ts';


const WAVE = [ [ -6, 3 ], [ -3, 3 ], [ -3, -3 ], [ 3, -3 ], [ 3, 3 ], [ 6, 3 ] ];
const FLAT = [ [ -6, 3 ], [ 6, 3 ] ];


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const x = toScreen( element.from.x );
	const y = toScreen( element.from.y );
	const [ start, ... rest ] = element.on === false ? FLAT : WAVE;

	capsule.draw( context, element, color( element.potentials[ 1 ], scale.voltage ) );

	context.lineWidth = 1.5;
	context.lineJoin = 'miter';
	context.beginPath();
	context.moveTo( x + start[ 0 ], y + start[ 1 ] );
	rest.forEach( ( [ dx, dy ] ) => context.lineTo( x + dx, y + dy ) );
	context.stroke();

	label.draw( context, element );
}
