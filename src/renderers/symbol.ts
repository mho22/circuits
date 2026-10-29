import { brightness, DARK, mix } from '../colors/light.ts';
import { color } from '../colors/voltage.ts';
import type { Color } from '../enums/color.ts';
import { FINE, SPACING, toScreen } from '../grid.ts';
import { pins } from '../pins.ts';
import { turn } from '../turn.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as drawing from './drawing.ts';


const COLOR = '#9e9e9e';
const RADIUS = 4;
const DOT = 3;
const SQUARE = 9;


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

	const groups = new Map<string, number[]>();

	outline.lights.forEach( ( light, index ) =>
	{
		const key = `${ light.point.x },${ light.point.y }`;
		const group = groups.get( key ) ?? [];

		group.push( index );
		groups.set( key, group );
	} );

	groups.forEach( indexes =>
	{
		const point = turn( outline.lights[ indexes[ 0 ] ].point, element.rotation );
		const x = toScreen( element.from.x + point.x );
		const y = toScreen( element.from.y + point.y );
		const glows = indexes.map( ( index ) : [ Color, number ] => [ outline.lights[ index ].color, brightness( element.lights?.[ index ] ?? 0 ) ] ).filter( ( [ , glow ] ) => glow > 0 );

		context.fillStyle = glows.length ? `rgb(${ mix( glows ) })` : DARK;
		context.fillRect( x - SQUARE, y - SQUARE, 2 * SQUARE, 2 * SQUARE );
	} );

	ends.forEach( ( pin, index ) =>
	{
		context.fillStyle = color( element.potentials[ index ], scale.voltage );
		context.beginPath();
		context.arc( toScreen( pin.x ), toScreen( pin.y ), DOT, 0, 2 * Math.PI );
		context.fill();
	} );
}
