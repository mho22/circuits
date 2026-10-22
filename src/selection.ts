import { bounds } from './bounds.ts';
import { Kind } from './enums/kind.ts';
import { FINE } from './grid.ts';
import { pins } from './pins.ts';
import { terminals } from './terminals.ts';
import { turn } from './turn.ts';
import type { Area } from './types/area.ts';
import type { Element } from './types/element.ts';
import type { Point } from './types/point.ts';


const MARGIN = 1;


function points( element : Element ) : Point[]
{
	if( element.kind === Kind.Symbol ) return [ element.from, ... element.outline ? pins( element.from, element.outline, element.rotation ) : [] ];

	return [ element.from, element.to, ... terminals( element.kind, element.from, element.to, element.mirrored ) ];
}

function corners( element : Element ) : Point[]
{
	const shapes = element.definition?.drawing ?? [];

	if( shapes.length === 0 ) return element.outline ? [ element.outline.low, element.outline.high ].map( corner => turn( corner, element.rotation ) ) : [];

	return shapes.flatMap( bounds ).map( point => turn( point, element.rotation ) ).flatMap( point => [ { x : Math.floor( point.x / FINE ), y : Math.floor( point.y / FINE ) }, { x : Math.ceil( point.x / FINE ), y : Math.ceil( point.y / FINE ) } ] );
}

function reach( element : Element ) : Point[]
{
	const { x, y } = element.from;

	if( element.kind === Kind.Ground ) return [ element.from, { x, y : y + 1 } ];

	if( element.kind === Kind.Supply ) return [ element.from, { x, y : y - 1 } ];

	if( element.kind === Kind.Symbol ) return [ ... points( element ), ... corners( element ).map( corner => ( { x : x + corner.x, y : y + corner.y } ) ) ];

	return points( element );
}


export function inside( elements : Element[], area : Area ) : number[]
{
	const within = ( point : Point ) => point.x >= area.low.x && point.x <= area.high.x && point.y >= area.low.y && point.y <= area.high.y;

	return elements.filter( element => points( element ).every( within ) ).map( element => element.id );
}

export function span( elements : Element[] ) : Area | null
{
	const all = elements.flatMap( reach );

	if( all.length === 0 ) return null;

	return cover( { low : all[ 0 ], high : all[ 0 ] }, all );
}

export function cover( area : Area, points : Point[] ) : Area
{
	const xs = points.map( point => point.x );
	const ys = points.map( point => point.y );

	return { low : { x : Math.min( area.low.x, Math.min( ... xs ) - MARGIN ), y : Math.min( area.low.y, Math.min( ... ys ) - MARGIN ) }, high : { x : Math.max( area.high.x, Math.max( ... xs ) + MARGIN ), y : Math.max( area.high.y, Math.max( ... ys ) + MARGIN ) } };
}
