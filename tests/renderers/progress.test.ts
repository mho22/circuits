import { describe, it, expect } from 'vitest';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/progress.ts';
import { record } from '../stubs/context.ts';


describe( 'progress renderer', () =>
{
	const from = { x : 0, y : 0 };
	const to = { x : 4, y : 0 };

	function dots( progress : number ) : number[]
	{
		const { context, calls } = record();

		draw( context, from, to, progress );

		return calls.filter( call => call[ 0 ] === 'fillRect' ).map( call => ( call[ 1 ] as number ) + 1.5 );
	}

	it( 'draws the wire in dark grey, then the filled part thicker in light grey with lighter grey dots', () =>
	{
		const { context, calls } = record();

		draw( context, from, to, 0.25 );

		const strokes = calls.filter( call => ( call[ 0 ] === 'set' && [ 'strokeStyle', 'lineWidth' ].includes( call[ 1 ] as string ) ) || call[ 0 ] === 'lineTo' );

		expect( strokes ).toEqual( [ [ 'set', 'strokeStyle', '#1d1d1d' ], [ 'set', 'lineWidth', 2 ], [ 'lineTo', SPACING * 4, 0 ], [ 'set', 'strokeStyle', '#4b4b4b' ], [ 'set', 'lineWidth', 3 ], [ 'lineTo', SPACING, 0 ] ] );
		expect( calls ).toContainEqual( [ 'set', 'fillStyle', '#656565' ] );
	} );

	it( 'places still dots along the filled part of the wire only', () =>
	{
		expect( dots( 0 ) ).toEqual( [] );
		expect( dots( 0.5 ) ).toEqual( [ 0, 16, 32 ] );
		expect( dots( 1 ) ).toEqual( [ 0, 16, 32, 48, 64 ] );
	} );
} );
