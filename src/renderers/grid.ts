import { SPACING } from '../grid.ts';


const BACKGROUND = '#000000';
const DOT = '#2a2a2a';
const FAINT = 0.5;


export function draw( context : CanvasRenderingContext2D, left : number, top : number, width : number, height : number, halves : boolean, stride : number ) : void
{
	context.fillStyle = BACKGROUND;
	context.fillRect( left, top, width, height );

	context.fillStyle = DOT;
	context.globalAlpha = 1 / stride;

	const gap = SPACING * stride;

	for( let x = Math.ceil( left / gap ) * gap; x <= left + width; x += gap )
	{
		for( let y = Math.ceil( top / gap ) * gap; y <= top + height; y += gap ) context.fillRect( x - stride, y - stride, stride * 2, stride * 2 );
	}

	context.globalAlpha = 1;

	if( ! halves ) return;

	const step = gap / 2;

	context.globalAlpha = FAINT / stride;

	for( let x = Math.ceil( left / step ) * step; x <= left + width; x += step )
	{
		for( let y = Math.ceil( top / step ) * step; y <= top + height; y += step )
		{
			if( x % gap === 0 && y % gap === 0 ) continue;

			context.fillRect( x - stride, y - stride, stride * 2, stride * 2 );
		}
	}

	context.globalAlpha = 1;
}
