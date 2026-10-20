import { describe, it, expect } from 'vitest';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/grid.ts';
import { record } from '../stubs/context.ts';


describe( 'grid renderer', () =>
{
	it( 'clears the screen to black', () =>
	{
		const { context, calls } = record();

		draw( context, SPACING, SPACING );

		expect( calls.slice( 0, 2 ) ).toEqual( [ [ 'set', 'fillStyle', '#000000' ], [ 'fillRect', 0, 0, SPACING, SPACING ] ] );
	} );

	it( 'draws one dot on each grid point', () =>
	{
		const { context, calls } = record();

		draw( context, SPACING, SPACING * 2 );

		const dots = calls.slice( 3 );

		expect( dots ).toEqual( [
			[ 'fillRect', -1, -1, 2, 2 ],
			[ 'fillRect', -1, SPACING - 1, 2, 2 ],
			[ 'fillRect', -1, SPACING * 2 - 1, 2, 2 ],
			[ 'fillRect', SPACING - 1, -1, 2, 2 ],
			[ 'fillRect', SPACING - 1, SPACING - 1, 2, 2 ],
			[ 'fillRect', SPACING - 1, SPACING * 2 - 1, 2, 2 ],
		] );
	} );
} );
