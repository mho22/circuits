import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/junction.ts';
import { record } from '../stubs/context.ts';


describe( 'junction renderer', () =>
{
	it( 'fills a dot on the point in the color of its voltage', () =>
	{
		const { context, calls } = record();

		draw( context, { point : { x : 1, y : 2 }, potential : 5 }, { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'set', 'fillStyle', color( 5, 5 ) ] );
		expect( calls ).toContainEqual( [ 'arc', SPACING, 2 * SPACING, 3, 0, 2 * Math.PI ] );
		expect( calls.at( -1 ) ).toEqual( [ 'fill' ] );
	} );
} );
