import { toScreen } from '../grid.ts';
import type { Area } from '../types/area.ts';


const SHADE = 'rgba( 0, 0, 0, 0.6 )';


export function draw( context : CanvasRenderingContext2D, area : Area, x : number, y : number, width : number, height : number ) : void
{
	context.fillStyle = SHADE;
	context.beginPath();
	context.rect( x, y, width, height );
	context.rect( toScreen( area.low.x ), toScreen( area.low.y ), toScreen( area.high.x - area.low.x ), toScreen( area.high.y - area.low.y ) );
	context.fill( 'evenodd' );
}
