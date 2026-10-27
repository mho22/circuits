import { Kind } from './enums/kind.ts';
import { pins } from './pins.ts';
import { terminals } from './terminals.ts';
import type { Element } from './types/element.ts';
import type { Junction } from './types/junction.ts';
import type { Point } from './types/point.ts';


const BRANCHES = 3;
const WIRES = 2;
const SPAN = 2 ** 32;


function ends( element : Element ) : Point[]
{
	if( element.kind !== Kind.Symbol ) return terminals( element.kind, element.from, element.to, element.mirrored );

	return element.outline ? pins( element.from, element.outline, element.rotation ) : [];
}

function key( point : Point ) : number
{
	return point.x * SPAN + point.y;
}

function layout( elements : Element[] ) : number[]
{
	const shape : number[] = [];

	for( const element of elements )
	{
		shape.push( element.kind, element.from.x, element.from.y, element.to.x, element.to.y, Number( element.mirrored ) );

		if( element.kind === Kind.Symbol ) for( const point of ends( element ) ) shape.push( point.x, point.y );
	}

	return shape;
}


export class Junctions
{
	private shape : number[] = [];

	private found : { point : Point, wire : number }[] = [];


	find( elements : Element[] ) : Junction[]
	{
		const shape = layout( elements );

		if( shape.length !== this.shape.length || shape.some( ( value, index ) => value !== this.shape[ index ] ) ) { this.shape = shape; this.found = this.locate( elements ); }

		return this.found.map( ( { point, wire } ) => ( { point, potential : elements[ wire ].potentials[ 0 ] } ) );
	}

	private locate( elements : Element[] ) : { point : Point, wire : number }[]
	{
		const counts = new Map<number, number>();
		const meetings = new Map<number, number>();
		const wires = new Map<number, { point : Point, wire : number }>();

		elements.forEach( ( element, index ) =>
		{
			const keys = ends( element ).map( key );

			keys.forEach( ( at, rank ) => { if( keys.indexOf( at ) === rank ) counts.set( at, ( counts.get( at ) ?? 0 ) + 1 ); } );

			if( element.kind !== Kind.Wire ) return;

			for( const point of [ element.from, element.to ] )
			{
				const at = key( point );

				meetings.set( at, ( meetings.get( at ) ?? 0 ) + 1 );
				wires.set( at, { point, wire : index } );
			}
		} );

		return [ ... wires ].filter( ( [ at ] ) => ( meetings.get( at ) ?? 0 ) >= WIRES || ( counts.get( at ) ?? 0 ) >= BRANCHES ).map( ( [ , found ] ) => found );
	}
}
