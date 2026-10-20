import { SPACING } from '../grid.ts';


const BACKGROUND = '#000000';
const DOT = '#2a2a2a';


export function draw( context : CanvasRenderingContext2D, left : number, top : number, width : number, height : number ) : void
{
	context.fillStyle = BACKGROUND;
	context.fillRect( left, top, width, height );

	context.fillStyle = DOT;

	for( let x = Math.ceil( left / SPACING ) * SPACING; x <= left + width; x += SPACING )
	{
		for( let y = Math.ceil( top / SPACING ) * SPACING; y <= top + height; y += SPACING ) context.fillRect( x - 1, y - 1, 2, 2 );
	}
}
