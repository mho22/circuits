import type { Arc } from './types/arc.ts';
import type { Point } from './types/point.ts';


const TURN = 2 * Math.PI;


export function arc( from : Point, through : Point, to : Point ) : Arc | null
{
	const divisor = 2 * ( from.x * ( through.y - to.y ) + through.x * ( to.y - from.y ) + to.x * ( from.y - through.y ) );

	if( divisor === 0 ) return null;

	const square = ( point : Point ) => point.x * point.x + point.y * point.y;
	const center = {
		x : ( square( from ) * ( through.y - to.y ) + square( through ) * ( to.y - from.y ) + square( to ) * ( from.y - through.y ) ) / divisor,
		y : ( square( from ) * ( to.x - through.x ) + square( through ) * ( from.x - to.x ) + square( to ) * ( through.x - from.x ) ) / divisor,
	};
	const angle = ( point : Point ) => Math.atan2( point.y - center.y, point.x - center.x );
	const start = angle( from );
	const end = angle( to );
	const span = ( value : number ) => ( ( value - start ) % TURN + TURN ) % TURN;

	return { center, radius : Math.hypot( from.x - center.x, from.y - center.y ), start, end, counterclockwise : span( angle( through ) ) > span( end ) };
}
