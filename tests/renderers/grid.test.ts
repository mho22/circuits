import { describe, it, expect } from 'vitest';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/grid.ts';
import { record } from '../stubs/context.ts';


describe( 'grid renderer', () =>
{
	it( 'clears the visible area to black', () =>
	{
		const { context, calls } = record();

		draw( context, 5, 7, SPACING, SPACING, false );

		expect( calls.slice( 0, 2 ) ).toEqual( [ [ 'set', 'fillStyle', '#000000' ], [ 'fillRect', 5, 7, SPACING, SPACING ] ] );
	} );

	it( 'draws one dot on each grid point', () =>
	{
		const { context, calls } = record();

		draw( context, 0, 0, SPACING, SPACING * 2, false );

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

	it( 'draws only the dots inside a moved area', () =>
	{
		const { context, calls } = record();

		draw( context, - SPACING * 3 - 5, SPACING * 2 + 5, SPACING, SPACING, false );

		expect( calls.slice( 3 ) ).toEqual( [ [ 'fillRect', - SPACING * 3 - 1, SPACING * 3 - 1, 2, 2 ] ] );
	} );

	it( 'draws faint dots on the half steps between the grid points', () =>
	{
		const { context, calls } = record();
		const half = SPACING / 2;

		draw( context, 0, 0, SPACING, SPACING, true );

		expect( calls.slice( 7 ) ).toEqual( [
			[ 'set', 'globalAlpha', 0.5 ],
			[ 'fillRect', -1, half - 1, 2, 2 ],
			[ 'fillRect', half - 1, -1, 2, 2 ],
			[ 'fillRect', half - 1, half - 1, 2, 2 ],
			[ 'fillRect', half - 1, SPACING - 1, 2, 2 ],
			[ 'fillRect', SPACING - 1, half - 1, 2, 2 ],
			[ 'set', 'globalAlpha', 1 ],
		] );
	} );
} );
