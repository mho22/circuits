import init, { Circuit } from '../bindings/web/pkg/web.js';
import { toGrid } from './grid.ts';
import * as cursor from './renderers/cursor.ts';
import * as grid from './renderers/grid.ts';
import * as wire from './renderers/wire.ts';
import { WireTool } from './tools/wire.ts';
import type { Point } from './types/point.ts';


await init();


const canvas = document.querySelector( 'canvas' )!;
const context = canvas.getContext( '2d' )!;
const circuit = new Circuit();
const tool = new WireTool( circuit );

let hover : Point | null = null;


function render() : void
{
	grid.draw( context, innerWidth, innerHeight );
	wire.draw( context, circuit.wires() );
	wire.draw( context, tool.preview(), 0.5 );
	cursor.draw( context, hover );
}

function resize() : void
{
	canvas.width = innerWidth * devicePixelRatio;
	canvas.height = innerHeight * devicePixelRatio;

	context.setTransform( devicePixelRatio, 0, 0, devicePixelRatio, 0, 0 );

	render();
}


canvas.addEventListener( 'pointerdown', ( event : PointerEvent ) =>
{
	const point = toGrid( event.clientX, event.clientY );

	if( event.button === 2 ) tool.erase( point );

	if( event.button === 0 )
	{
		canvas.setPointerCapture( event.pointerId );

		tool.press( point );
	}

	render();
} );

canvas.addEventListener( 'pointermove', ( event : PointerEvent ) =>
{
	hover = toGrid( event.clientX, event.clientY );

	tool.move( hover );

	render();
} );

canvas.addEventListener( 'pointerup', () =>
{
	tool.release();

	render();
} );

canvas.addEventListener( 'pointerleave', () =>
{
	hover = null;

	render();
} );

canvas.addEventListener( 'contextmenu', ( event : MouseEvent ) => event.preventDefault() );

addEventListener( 'resize', resize );


resize();
