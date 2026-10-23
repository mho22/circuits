import { describe, it, expect } from 'vitest';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/grid.ts';
import { record } from '../stubs/context.ts';


describe( 'grid renderer', () =>
{
	it( 'clears the visible area to black', () =>
	{
		const { context, calls } = record();

		draw( context, 5, 7, SPACING, SPACING, false, 1 );

		expect( calls.slice( 0, 2 ) ).toEqual( [ [ 'set', 'fillStyle', '#000000' ], [ 'fillRect', 5, 7, SPACING, SPACING ] ] );
	} );

	it( 'draws one dot on each grid point', () =>
	{
		const { context, calls } = record();

		draw( context, 0, 0, SPACING, SPACING * 2, false, 1 );

		const dots = calls.slice( 4, -1 );

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

		draw( context, - SPACING * 3 - 5, SPACING * 2 + 5, SPACING, SPACING, false, 1 );

		expect( calls.slice( 4, -1 ) ).toEqual( [ [ 'fillRect', - SPACING * 3 - 1, SPACING * 3 - 1, 2, 2 ] ] );
	} );

	it( 'draws faint dots on the half steps between the grid points', () =>
	{
		const { context, calls } = record();
		const half = SPACING / 2;

		draw( context, 0, 0, SPACING, SPACING, true, 1 );

		expect( calls.slice( 9 ) ).toEqual( [
			[ 'set', 'globalAlpha', 0.5 ],
			[ 'fillRect', -1, half - 1, 2, 2 ],
			[ 'fillRect', half - 1, -1, 2, 2 ],
			[ 'fillRect', half - 1, half - 1, 2, 2 ],
			[ 'fillRect', half - 1, SPACING - 1, 2, 2 ],
			[ 'fillRect', SPACING - 1, half - 1, 2, 2 ],
			[ 'set', 'globalAlpha', 1 ],
		] );
	} );

	it( 'draws one larger and fainter dot on every second or fourth grid point when the view is zoomed out', () =>
	{
		const { context, calls } = record();

		draw( context, 0, 0, SPACING * 4, SPACING, false, 4 );

		expect( calls.slice( 3 ) ).toEqual( [ [ 'set', 'globalAlpha', 0.25 ], [ 'fillRect', -4, -4, 8, 8 ], [ 'fillRect', SPACING * 4 - 4, -4, 8, 8 ], [ 'set', 'globalAlpha', 1 ] ] );

		calls.length = 0;

		draw( context, 0, 0, 0, 0, false, 2 );

		expect( calls[ 3 ] ).toEqual( [ 'set', 'globalAlpha', 0.5 ] );
	} );

	it( 'draws fainter half steps between the larger dots when the view is zoomed out', () =>
	{
		const { context, calls } = record();

		draw( context, 0, 0, SPACING * 2, 0, true, 2 );

		expect( calls.slice( 7 ) ).toEqual( [ [ 'set', 'globalAlpha', 0.25 ], [ 'fillRect', SPACING - 2, -2, 4, 4 ], [ 'set', 'globalAlpha', 1 ] ] );
	} );
} );
