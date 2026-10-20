import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as label from './label.ts';
import * as line from './line.ts';


const GAP = 4;
const PLATE = 12;


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const fromX = toScreen( element.from.x );
	const fromY = toScreen( element.from.y );
	const toX = toScreen( element.to.x );
	const toY = toScreen( element.to.y );
	const length = Math.hypot( toX - fromX, toY - fromY );
	const alongX = ( toX - fromX ) / length;
	const alongY = ( toY - fromY ) / length;
	const middleX = ( fromX + toX ) / 2;
	const middleY = ( fromY + toY ) / 2;
	const plates : [ number, number, string ][] = [
		[ middleX - alongX * GAP, middleY - alongY * GAP, color( element.potentials[ 0 ], scale.voltage ) ],
		[ middleX + alongX * GAP, middleY + alongY * GAP, color( element.potentials[ 1 ], scale.voltage ) ],
	];

	context.lineCap = 'butt';

	line.draw( context, plates[ 0 ][ 2 ], 2, fromX, fromY, plates[ 0 ][ 0 ], plates[ 0 ][ 1 ] );
	line.draw( context, plates[ 1 ][ 2 ], 2, plates[ 1 ][ 0 ], plates[ 1 ][ 1 ], toX, toY );

	plates.forEach( ( [ x, y, style ] ) => line.draw( context, style, 2, x + alongY * PLATE, y - alongX * PLATE, x - alongY * PLATE, y + alongX * PLATE ) );

	label.draw( context, element );
}
