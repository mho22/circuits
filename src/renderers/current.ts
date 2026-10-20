import { toScreen } from '../grid.ts';
import type { Point } from '../types/point.ts';


const COLOR = '#ffd400';
const SPACING = 16;
const SIZE = 3;
const LEAK = 1e-9;


export function draw( context : CanvasRenderingContext2D, from : Point, to : Point, current : number, scale : number, distance : number ) : void
{
	if( ! ( scale > 0 ) || ! ( Math.abs( current ) > Math.max( scale * 1e-6, LEAK ) ) ) return;

	const fromX = toScreen( from.x );
	const fromY = toScreen( from.y );
	const length = Math.hypot( toScreen( to.x ) - fromX, toScreen( to.y ) - fromY );
	const alongX = ( toScreen( to.x ) - fromX ) / length;
	const alongY = ( toScreen( to.y ) - fromY ) / length;
	const offset = ( distance % SPACING + SPACING ) % SPACING;

	context.fillStyle = COLOR;

	for( let along = offset; along < length; along += SPACING )
	{
		context.fillRect( fromX + alongX * along - SIZE / 2, fromY + alongY * along - SIZE / 2, SIZE, SIZE );
	}
}
