import { bounds } from './bounds.ts';
import { FINE } from './grid.ts';
import { cover } from './selection.ts';
import type { Area } from './types/area.ts';
import type { Pin } from './types/pin.ts';
import type { Shape } from './types/shape.ts';


export function extent( shapes : Shape[], pins : Pin[] ) : Area | null
{
	const all = [ ... shapes.flatMap( bounds ).flatMap( point => [ { x : Math.floor( point.x / FINE ), y : Math.floor( point.y / FINE ) }, { x : Math.ceil( point.x / FINE ), y : Math.ceil( point.y / FINE ) } ] ), ... pins.map( pin => pin.point ) ];

	if( all.length === 0 ) return null;

	return cover( { low : all[ 0 ], high : all[ 0 ] }, all );
}
