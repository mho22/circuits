import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';


const LEAD = 10;
const BARS = [ 10, 6, 2 ];
const SPACING = 4;


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const x = toScreen( element.from.x );
	const y = toScreen( element.from.y );

	context.strokeStyle = color( element.potentials[ 0 ], scale.voltage );
	context.lineWidth = 2;
	context.lineCap = 'butt';
	context.beginPath();
	context.moveTo( x, y );
	context.lineTo( x, y + LEAD );

	BARS.forEach( ( half, index ) =>
	{
		context.moveTo( x - half, y + LEAD + index * SPACING );
		context.lineTo( x + half, y + LEAD + index * SPACING );
	} );

	context.stroke();
}
