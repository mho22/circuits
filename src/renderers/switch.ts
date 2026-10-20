import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as line from './line.ts';


const BODY = 30;
const OPENING = Math.PI / 6;


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const fromX = toScreen( element.from.x );
	const fromY = toScreen( element.from.y );
	const toX = toScreen( element.to.x );
	const toY = toScreen( element.to.y );
	const length = Math.hypot( toX - fromX, toY - fromY );
	const alongX = ( toX - fromX ) / length;
	const alongY = ( toY - fromY ) / length;
	const body = Math.min( BODY, length );
	const pivotX = fromX + alongX * ( length - body ) / 2;
	const pivotY = fromY + alongY * ( length - body ) / 2;
	const contactX = pivotX + alongX * body;
	const contactY = pivotY + alongY * body;
	const turn = element.value ? 0 : - OPENING;
	const leverX = pivotX + ( alongX * Math.cos( turn ) - alongY * Math.sin( turn ) ) * body;
	const leverY = pivotY + ( alongX * Math.sin( turn ) + alongY * Math.cos( turn ) ) * body;
	const before = color( element.potentials[ 0 ], scale.voltage );

	context.lineCap = 'round';

	line.draw( context, before, 2, fromX, fromY, pivotX, pivotY );
	line.draw( context, color( element.potentials[ 1 ], scale.voltage ), 2, contactX, contactY, toX, toY );
	line.draw( context, before, 2, pivotX, pivotY, leverX, leverY );
}
