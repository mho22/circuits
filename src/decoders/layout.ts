import type { Point } from '../types/point.ts';


export function decode( data : ArrayLike<number> ) : Point[]
{
	return Array.from( { length : Math.floor( data.length / 2 ) }, ( _, index ) => ( { x : data[ 2 * index ], y : data[ 2 * index + 1 ] } ) );
}
