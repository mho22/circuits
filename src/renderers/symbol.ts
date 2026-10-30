import { brightness, CHANNELS, DARK, fill } from '../colors/light.ts';
import { color } from '../colors/voltage.ts';
import { FINE, SPACING, toScreen } from '../grid.ts';
import { pins } from '../pins.ts';
import { turn } from '../turn.ts';
import type { Area } from '../types/area.ts';
import type { Element } from '../types/element.ts';
import type { Outline } from '../types/outline.ts';
import type { Point } from '../types/point.ts';
import type { Scale } from '../types/scale.ts';
import type { Spot } from '../types/spot.ts';
import * as drawing from './drawing.ts';


const COLOR = '#9e9e9e';
const RADIUS = 4;
const DOT = 3;
const SQUARE = 9;
const GLOW = 1 / 4;
const SPOTS = new WeakMap<Outline, Spot[][]>();


function spots( outline : Outline, rotation : number ) : Spot[]
{
	const turns = SPOTS.get( outline ) ?? [];
	const known = turns[ rotation ];

	SPOTS.set( outline, turns );

	if( known ) return known;

	const groups = new Map<string, number[]>();

	outline.lights.forEach( ( light, index ) =>
	{
		const key = `${ light.point.x },${ light.point.y }`;
		const group = groups.get( key ) ?? [];

		group.push( index );
		groups.set( key, group );
	} );

	turns[ rotation ] = [ ... groups.values() ].map( indexes => ( { point : turn( outline.lights[ indexes[ 0 ] ].point, rotation ), indexes } ) );

	return turns[ rotation ];
}

function continues( { low, high } : Area, point : Point ) : boolean
{
	const row = low.y === point.y && high.y === point.y && ( point.x === high.x + 1 || point.x === low.x - 1 );
	const column = low.x === point.x && high.x === point.x && ( point.y === high.y + 1 || point.y === low.y - 1 );

	return row || column;
}


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale, stride : number ) : void
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
	const half = SQUARE + GLOW * stride;

	if( definition?.drawing.length ) drawing.draw( context, definition.drawing, center, element.rotation, SPACING / FINE );

	else
	{
		context.strokeStyle = COLOR;
		context.lineWidth = 2;
		context.beginPath();
		context.roundRect( toScreen( left ), toScreen( top ), toScreen( right - left ), toScreen( bottom - top ), RADIUS );
		context.stroke();
	}

	const currents = element.lights ?? [];
	const touching = 2 * half >= SPACING;
	let style = '';
	let run : Area | null = null;

	const square = ( { low, high } : Area ) => context.rect( toScreen( element.from.x + low.x ) - half, toScreen( element.from.y + low.y ) - half, toScreen( high.x - low.x ) + 2 * half, toScreen( high.y - low.y ) + 2 * half );

	for( const { point, indexes } of spots( outline, element.rotation ) )
	{
		let red = 0;
		let green = 0;
		let blue = 0;
		let lit = false;

		for( const index of indexes )
		{
			const glow = brightness( currents[ index ] ?? 0 );

			if( glow === 0 ) continue;

			const channels = CHANNELS[ outline.lights[ index ].color ];

			red += channels[ 0 ] * glow;
			green += channels[ 1 ] * glow;
			blue += channels[ 2 ] * glow;
			lit = true;
		}

		const shade = lit ? fill( red, green, blue ) : DARK;

		if( run && touching && shade === style && continues( run, point ) )
		{
			run = { low : { x : Math.min( run.low.x, point.x ), y : Math.min( run.low.y, point.y ) }, high : { x : Math.max( run.high.x, point.x ), y : Math.max( run.high.y, point.y ) } };

			continue;
		}

		if( run ) square( run );

		if( shade !== style )
		{
			if( style ) context.fill();

			context.fillStyle = shade;
			context.beginPath();
			style = shade;
		}

		run = { low : point, high : point };
	}

	if( run ) square( run );

	if( style ) context.fill();

	ends.forEach( ( pin, index ) =>
	{
		context.fillStyle = color( element.potentials[ index ], scale.voltage );
		context.beginPath();
		context.arc( toScreen( pin.x ), toScreen( pin.y ), DOT, 0, 2 * Math.PI );
		context.fill();
	} );
}
