import { describe, it, expect } from 'vitest';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/cursor.ts';
import { record } from '../stubs/context.ts';


describe( 'cursor renderer', () =>
{
	it( 'circles the hovered grid point', () =>
	{
		const { context, calls } = record();

		draw( context, { x : 2, y : 3 } );

		expect( calls ).toContainEqual( [ 'arc', SPACING * 2, SPACING * 3, 5, 0, Math.PI * 2 ] );
	} );

	it( 'draws nothing without a hovered point', () =>
	{
		const { context, calls } = record();

		draw( context, null );

		expect( calls ).toEqual( [] );
	} );
} );
