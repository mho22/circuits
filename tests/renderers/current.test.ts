import { describe, it, expect } from 'vitest';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/current.ts';
import { record } from '../stubs/context.ts';


describe( 'current renderer', () =>
{
	const from = { x : 0, y : 0 };
	const to = { x : 2, y : 0 };

	function dots( current : number, scale : number, distance : number ) : number[]
	{
		const { context, calls } = record();

		draw( context, from, to, current, scale, distance );

		return calls.filter( call => call[ 0 ] === 'fillRect' ).map( call => ( call[ 1 ] as number ) + 1.5 );
	}

	it( 'spaces dots evenly along the path', () =>
	{
		expect( dots( 1, 1, 0 ) ).toEqual( [ 0, 16, 32 ] );
	} );

	it( 'shifts the dots by the distance they drifted', () =>
	{
		expect( dots( 1, 1, 4 ) ).toEqual( [ 4, 20, 36 ] );
		expect( dots( 1, 1, 36 ) ).toEqual( [ 4, 20, 36 ] );
	} );

	it( 'shifts the dots backward with a negative distance', () =>
	{
		expect( dots( -1, 1, -4 ) ).toEqual( [ 12, 28 ] );
	} );

	it( 'draws no dot without current or scale', () =>
	{
		expect( dots( 0, 1, 0 ) ).toEqual( [] );
		expect( dots( NaN, 1, 0 ) ).toEqual( [] );
		expect( dots( 1, 0, 0 ) ).toEqual( [] );
	} );

	it( 'stays within the path length', () =>
	{
		expect( Math.max( ... dots( 1, 1, 15.2 ) ) ).toBeLessThan( SPACING * 2 );
	} );
} );
