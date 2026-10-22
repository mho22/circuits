import { color } from '../colors/voltage.ts';
import { toScreen } from '../grid.ts';
import type { Junction } from '../types/junction.ts';
import type { Scale } from '../types/scale.ts';


const DOT = 3;


export function draw( context : CanvasRenderingContext2D, junction : Junction, scale : Scale ) : void
{
	context.fillStyle = color( junction.potential, scale.voltage );
	context.beginPath();
	context.arc( toScreen( junction.point.x ), toScreen( junction.point.y ), DOT, 0, 2 * Math.PI );
	context.fill();
}
