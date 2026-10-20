import { SPACING } from '../grid.ts';


const BACKGROUND = '#000000';
const DOT = '#2a2a2a';


export function draw( context : CanvasRenderingContext2D, width : number, height : number ) : void
{
	context.fillStyle = BACKGROUND;
	context.fillRect( 0, 0, width, height );

	context.fillStyle = DOT;

	for( let x = 0; x <= width; x += SPACING )
	{
		for( let y = 0; y <= height; y += SPACING ) context.fillRect( x - 1, y - 1, 2, 2 );
	}
}
