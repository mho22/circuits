import { bounds } from './bounds.ts';
import { FINE } from './grid.ts';
import type { Area } from './types/area.ts';
import type { Grab } from './types/grab.ts';
import type { Pin } from './types/pin.ts';
import type { Point } from './types/point.ts';
import type { Shape } from './types/shape.ts';


export function grab( shapes : Shape[], pins : Pin[], area : Area ) : Grab
{
	const within = ( point : Point ) => point.x >= area.low.x && point.x <= area.high.x && point.y >= area.low.y && point.y <= area.high.y;
	const indexes = <T>( items : T[], keep : ( item : T ) => boolean ) => items.flatMap( ( item, index ) => keep( item ) ? [ index ] : [] );

	return { shapes : indexes( shapes, shape => bounds( shape ).every( within ) ), pins : indexes( pins, pin => within( { x : pin.point.x * FINE, y : pin.point.y * FINE } ) ) };
}
