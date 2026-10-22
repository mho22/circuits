import { pins } from './pins.ts';
import type { Element } from './types/element.ts';
import type { Point } from './types/point.ts';


export function socket( element : Element, point : Point ) : number
{
	if( ! element.outline ) return -1;

	return pins( element.from, element.outline, element.rotation ).findIndex( pin => pin.x === point.x && pin.y === point.y );
}
