import { color } from '../colors/voltage.ts';
import { FINE, SPACING, toScreen } from '../grid.ts';
import { pins } from '../pins.ts';
import { turn } from '../turn.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as drawing from './drawing.ts';


const COLOR = '#9e9e9e';
const RADIUS = 4;
const DOT = 3;
const TEXT = '#8a8a8a';
const GAP = 6;


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const { outline, definition } = element;

	if( ! outline ) return;

	const corners = [ outline.low, outline.high ].map( corner => turn( corner, element.rotation ) );
	const left = element.from.x + Math.min( ... corners.map( corner => corner.x ) );
	const top = element.from.y + Math.min( ... corners.map( corner => corner.y ) );
	const right = element.from.x + Math.max( ... corners.map( corner => corner.x ) );
	const bottom = element.from.y + Math.max( ... corners.map( corner => corner.y ) );
	const center = { x : toScreen( element.from.x ), y : toScreen( element.from.y ) };
	const ends = pins( element.from, outline, element.rotation );

	if( definition?.drawing.length ) drawing.draw( context, definition.drawing, center, element.rotation, SPACING / FINE );

	else
	{
		context.strokeStyle = COLOR;
		context.lineWidth = 2;
		context.beginPath();
		context.roundRect( toScreen( left ), toScreen( top ), toScreen( right - left ), toScreen( bottom - top ), RADIUS );
		context.stroke();
	}

	ends.forEach( ( pin, index ) =>
	{
		context.fillStyle = color( element.potentials[ index ], scale.voltage );
		context.beginPath();
		context.arc( toScreen( pin.x ), toScreen( pin.y ), DOT, 0, 2 * Math.PI );
		context.fill();
	} );

	if( ! definition ) return;

	context.fillStyle = TEXT;
	context.font = '10px ui-monospace, monospace';
	context.textAlign = 'center';
	context.textBaseline = 'bottom';

	ends.forEach( ( pin, index ) => { if( definition.labels[ index ] ) context.fillText( definition.labels[ index ], toScreen( pin.x ), toScreen( pin.y ) - GAP ); } );
}
