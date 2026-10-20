import { describe, it, expect } from 'vitest';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/current.ts';
import { record } from '../stubs/context.ts';


describe( 'current renderer', () =>
{
	const from = { x : 0, y : 0 };
	const to = { x : 2, y : 0 };

	function dots( current : number, scale : number, time : number ) : number[]
	{
		const { context, calls } = record();

		draw( context, from, to, current, scale, time );

		return calls.filter( call => call[ 0 ] === 'fillRect' ).map( call => ( call[ 1 ] as number ) + 1.5 );
	}

	it( 'spaces dots evenly along the path', () =>
	{
		expect( dots( 1, 1, 0 ) ).toEqual( [ 0, 16, 32 ] );
	} );

	it( 'moves the dots forward with a positive current', () =>
	{
		expect( dots( 1, 1, 0.05 ) ).toEqual( [ 4, 20, 36 ] );
	} );

	it( 'moves the dots backward with a negative current', () =>
	{
		expect( dots( -1, 1, 0.05 ) ).toEqual( [ 12, 28 ] );
	} );

	it( 'moves the dots slower with a smaller current', () =>
	{
		expect( dots( 0.5, 1, 0.05 ) ).toEqual( [ 2, 18, 34 ] );
	} );

	it( 'draws no dot without current or scale', () =>
	{
		expect( dots( 0, 1, 0 ) ).toEqual( [] );
		expect( dots( NaN, 1, 0 ) ).toEqual( [] );
		expect( dots( 1, 0, 0 ) ).toEqual( [] );
	} );

	it( 'stays within the path length', () =>
	{
		expect( Math.max( ... dots( 1, 1, 0.19 ) ) ).toBeLessThan( SPACING * 2 );
	} );
} );
