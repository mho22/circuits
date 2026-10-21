import { SPACING } from '../grid.ts';


const BACKGROUND = '#000000';
const DOT = '#2a2a2a';
const FAINT = 0.5;


export function draw( context : CanvasRenderingContext2D, left : number, top : number, width : number, height : number, halves : boolean ) : void
{
	context.fillStyle = BACKGROUND;
	context.fillRect( left, top, width, height );

	context.fillStyle = DOT;

	for( let x = Math.ceil( left / SPACING ) * SPACING; x <= left + width; x += SPACING )
	{
		for( let y = Math.ceil( top / SPACING ) * SPACING; y <= top + height; y += SPACING ) context.fillRect( x - 1, y - 1, 2, 2 );
	}

	if( ! halves ) return;

	const step = SPACING / 2;

	context.globalAlpha = FAINT;

	for( let x = Math.ceil( left / step ) * step; x <= left + width; x += step )
	{
		for( let y = Math.ceil( top / step ) * step; y <= top + height; y += step )
		{
			if( x % SPACING === 0 && y % SPACING === 0 ) continue;

			context.fillRect( x - 1, y - 1, 2, 2 );
		}
	}

	context.globalAlpha = 1;
}
