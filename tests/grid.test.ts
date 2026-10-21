import { describe, it, expect } from 'vitest';
import { SPACING, snap, toGrid, toScreen } from '../src/grid.ts';


describe( 'grid', () =>
{
	it( 'rounds a screen position to the nearest grid point', () =>
	{
		expect( toGrid( SPACING * 3 + 9, SPACING * 2 - 9 ) ).toEqual( { x : 3, y : 2 } );

		expect( toGrid( SPACING * 3 + 11, SPACING * 2 - 11 ) ).toEqual( { x : 4, y : 1 } );
	} );

	it( 'snaps a fine point to the nearest half grid step', () =>
	{
		expect( snap( { x : 2.9, y : 5.2 } ) ).toEqual( { x : 2, y : 6 } );
		expect( snap( { x : -4.9, y : 13.1 } ) ).toEqual( { x : -4, y : 14 } );
	} );

	it( 'converts a grid coordinate to a screen position', () =>
	{
		expect( toScreen( 4 ) ).toBe( SPACING * 4 );
	} );
} );
