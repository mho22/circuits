import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/wire.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'wire renderer', () =>
{
	it( 'strokes the wire in screen coordinates with the color of its voltage', () =>
	{
		const { context, calls } = record();

		draw( context, element( { from : { x : 1, y : 1 }, to : { x : 3, y : 3 }, potentials : [ 5, 5 ] } ), { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'set', 'strokeStyle', color( 5, 5 ) ] );
		expect( calls ).toContainEqual( [ 'moveTo', SPACING, SPACING ] );
		expect( calls ).toContainEqual( [ 'lineTo', SPACING * 3, SPACING * 3 ] );
		expect( calls.at( -1 ) ).toEqual( [ 'stroke' ] );
	} );
} );
