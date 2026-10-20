import { toScreen } from '../grid.ts';


const COLOR = '#9e9e9e';


export function draw( context : CanvasRenderingContext2D, segments : ArrayLike<number>, alpha = 1 ) : void
{
	context.globalAlpha = alpha;
	context.strokeStyle = COLOR;
	context.lineWidth = 2;
	context.lineCap = 'round';
	context.beginPath();

	for( let index = 0; index < segments.length; index += 4 )
	{
		context.moveTo( toScreen( segments[ index ] ), toScreen( segments[ index + 1 ] ) );
		context.lineTo( toScreen( segments[ index + 2 ] ), toScreen( segments[ index + 3 ] ) );
	}

	context.stroke();
	context.globalAlpha = 1;
}
