import { LIT } from '../colors/light.ts';
import { FINE } from '../grid.ts';
import type { Board } from '../types/board.ts';
import type { Pin } from '../types/pin.ts';
import type { Point } from '../types/point.ts';


const COLOR = '#9e9e9e';
const CURSOR = '#ffff00';
const HALF = 6;
const LIGHT = 9;
const RADIUS = 3;
const RING = 5;
const GAP = 6;


export function draw( context : CanvasRenderingContext2D, board : Board, pins : Pin[], cursor : Point | null ) : void
{
	const at = ( x : number, y : number ) : [ number, number ] => [ board.x + x * board.unit, board.y + y * board.unit ];

	context.strokeStyle = COLOR;
	context.fillStyle = COLOR;
	context.lineWidth = 2;
	context.font = '12px ui-monospace, monospace';
	context.textBaseline = 'middle';

	pins.forEach( pin =>
	{
		const [ px, py ] = at( pin.point.x * FINE, pin.point.y * FINE );
		const left = pin.point.x <= 0;

		if( pin.light !== undefined )
		{
			context.fillStyle = `rgb(${ LIT[ pin.light ] })`;
			context.fillRect( px - LIGHT, py - LIGHT, 2 * LIGHT, 2 * LIGHT );
			context.fillStyle = COLOR;

			return;
		}

		context.beginPath();
		context.roundRect( px - HALF, py - HALF, 2 * HALF, 2 * HALF, pin.output ? RADIUS : 0 );
		context.stroke();

		context.textAlign = left ? 'right' : 'left';
		context.fillText( pin.label, left ? px - HALF - GAP : px + HALF + GAP, py );
	} );

	if( ! cursor ) return;

	context.strokeStyle = CURSOR;
	context.lineWidth = 1;
	context.beginPath();
	context.arc( ... at( cursor.x, cursor.y ), RING, 0, 2 * Math.PI );
	context.stroke();
}
