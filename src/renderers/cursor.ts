import { toScreen } from '../grid.ts';
import type { Point } from '../types/point.ts';


const COLOR = '#5c5c5c';


export function draw( context : CanvasRenderingContext2D, point : Point | null ) : void
{
	if( ! point ) return;

	context.strokeStyle = COLOR;
	context.lineWidth = 1;
	context.beginPath();
	context.arc( toScreen( point.x ), toScreen( point.y ), 5, 0, Math.PI * 2 );
	context.stroke();
}
