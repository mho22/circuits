import { describe, it, expect } from 'vitest';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/area.ts';
import { record } from '../stubs/context.ts';


describe( 'area renderer', () =>
{
	it( 'draws a dashed rounded rectangle between the corners, then clears the dash', () =>
	{
		const { context, calls } = record();

		draw( context, { low : { x : 1, y : 2 }, high : { x : 4, y : 3 } } );

		expect( calls ).toContainEqual( [ 'roundRect', SPACING, 2 * SPACING, 3 * SPACING, SPACING, 8 ] );
		expect( calls.filter( call => call[ 0 ] === 'setLineDash' ) ).toEqual( [ [ 'setLineDash', [ 6, 4 ] ], [ 'setLineDash', [] ] ] );
	} );
} );
