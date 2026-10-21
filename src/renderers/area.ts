import { toScreen } from '../grid.ts';
import type { Area } from '../types/area.ts';


const COLOR = '#ffff00';
const RADIUS = 8;
const DASH = [ 6, 4 ];


export function draw( context : CanvasRenderingContext2D, area : Area ) : void
{
	context.strokeStyle = COLOR;
	context.lineWidth = 1;
	context.setLineDash( DASH );
	context.beginPath();
	context.roundRect( toScreen( area.low.x ), toScreen( area.low.y ), toScreen( area.high.x - area.low.x ), toScreen( area.high.y - area.low.y ), RADIUS );
	context.stroke();
	context.setLineDash( [] );
}
