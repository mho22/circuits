import type { Pin } from './types/pin.ts';
import type { Point } from './types/point.ts';


const STACK = 3;


export function crowded( pins : Pin[], point : Point ) : boolean
{
	const shared = pins.filter( pin => pin.point.x === point.x && pin.point.y === point.y );

	if( shared.length < 2 ) return false;

	return shared.length > STACK || shared.some( pin => pin.light === undefined );
}
