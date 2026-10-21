import { describe, it, expect } from 'vitest';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/mask.ts';
import { record } from '../stubs/context.ts';


describe( 'mask renderer', () =>
{
	it( 'shades the view outside the area with an even-odd fill', () =>
	{
		const { context, calls } = record();

		draw( context, { low : { x : 1, y : 2 }, high : { x : 4, y : 3 } }, -10, -20, 800, 600 );

		expect( calls.filter( call => call[ 0 ] === 'rect' ) ).toEqual( [ [ 'rect', -10, -20, 800, 600 ], [ 'rect', SPACING, 2 * SPACING, 3 * SPACING, SPACING ] ] );
		expect( calls.at( -1 ) ).toEqual( [ 'fill', 'evenodd' ] );
	} );
} );
