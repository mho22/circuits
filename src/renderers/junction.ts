import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Junction } from '../types/junction.ts';
import type { Scale } from '../types/scale.ts';


const DOT = 3;


export function draw( context : CanvasRenderingContext2D, junctions : Junction[], scale : Scale ) : void
{
	const groups = new Map<string, Junction[]>();

	junctions.forEach( junction =>
	{
		const fill = color( junction.potential, scale.voltage );
		const group = groups.get( fill ) ?? [];

		group.push( junction );
		groups.set( fill, group );
	} );

	groups.forEach( ( group, fill ) =>
	{
		context.fillStyle = fill;
		context.beginPath();

		group.forEach( ( { point } ) =>
		{
			context.moveTo( toScreen( point.x ) + DOT, toScreen( point.y ) );
			context.arc( toScreen( point.x ), toScreen( point.y ), DOT, 0, 2 * Math.PI );
		} );

		context.fill();
	} );
}
