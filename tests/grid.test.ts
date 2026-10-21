import { describe, it, expect } from 'vitest';
import { SPACING, snap, toGrid, toScreen } from '../src/grid.ts';


describe( 'grid', () =>
{
	it( 'rounds a screen position to the nearest grid point', () =>
	{
		expect( toGrid( SPACING * 3 + 9, SPACING * 2 - 9 ) ).toEqual( { x : 3, y : 2 } );

		expect( toGrid( SPACING * 3 + 11, SPACING * 2 - 11 ) ).toEqual( { x : 4, y : 1 } );
	} );

	it( 'snaps a fine point to the nearest grid step', () =>
	{
		expect( snap( { x : 5, y : 7 } ) ).toEqual( { x : 4, y : 8 } );
		expect( snap( { x : -6, y : 13 } ) ).toEqual( { x : -4, y : 12 } );
	} );

	it( 'converts a grid coordinate to a screen position', () =>
	{
		expect( toScreen( 4 ) ).toBe( SPACING * 4 );
	} );
} );
