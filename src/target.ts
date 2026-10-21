import { FINE } from './grid.ts';
import { hit } from './hit.ts';
import type { Pin } from './types/pin.ts';
import type { Point } from './types/point.ts';
import type { Shape } from './types/shape.ts';
import type { Target } from './types/target.ts';


const REACH = 1;


export function target( shapes : Shape[], pins : Pin[], point : Point ) : Target | null
{
	const pin = pins.findIndex( pin => Math.abs( pin.point.x * FINE - point.x ) <= REACH && Math.abs( pin.point.y * FINE - point.y ) <= REACH );

	if( pin >= 0 ) return { pin : true, index : pin };

	const shape = shapes.findLastIndex( shape => hit( shape, point ) );

	return shape < 0 ? null : { pin : false, index : shape };
}
