import { Kind } from './enums/kind.ts';
import { pins } from './pins.ts';
import { terminals } from './terminals.ts';
import type { Element } from './types/element.ts';
import type { Junction } from './types/junction.ts';
import type { Point } from './types/point.ts';


const BRANCHES = 3;
const WIRES = 2;


function ends( element : Element ) : Point[]
{
	if( element.kind !== Kind.Symbol ) return terminals( element.kind, element.from, element.to, element.mirrored );

	return element.outline ? pins( element.from, element.outline, element.rotation ) : [];
}


export function junctions( elements : Element[] ) : Junction[]
{
	const counts = new Map<string, number>();
	const meetings = new Map<string, number>();
	const wires = new Map<string, Junction>();

	for( const element of elements )
	{
		for( const key of new Set( ends( element ).map( point => `${ point.x },${ point.y }` ) ) ) counts.set( key, ( counts.get( key ) ?? 0 ) + 1 );

		if( element.kind !== Kind.Wire ) continue;

		for( const point of [ element.from, element.to ] )
		{
			const key = `${ point.x },${ point.y }`;

			meetings.set( key, ( meetings.get( key ) ?? 0 ) + 1 );
			wires.set( key, { point, potential : element.potentials[ 0 ] } );
		}
	}

	return [ ... wires ].filter( ( [ key ] ) => ( meetings.get( key ) ?? 0 ) >= WIRES || ( counts.get( key ) ?? 0 ) >= BRANCHES ).map( ( [ , junction ] ) => junction );
}
