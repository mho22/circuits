import { describe, it, expect } from 'vitest';
import { bus, color } from '../../src/colors/voltage.ts';
import { SPACING } from '../../src/grid.ts';
import { BUS, draw } from '../../src/renderers/wire.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'wire renderer', () =>
{
	it( 'strokes the wire in screen coordinates with the color of its voltage', () =>
	{
		const { context, calls } = record();

		draw( context, element( { from : { x : 1, y : 1 }, to : { x : 3, y : 1 }, potentials : [ 5, 5 ] } ), { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'set', 'strokeStyle', color( 5, 5 ) ] );
		expect( calls ).toContainEqual( [ 'moveTo', SPACING, SPACING ] );
		expect( calls ).toContainEqual( [ 'lineTo', SPACING * 3, SPACING ] );
		expect( calls.at( -1 ) ).toEqual( [ 'stroke' ] );
	} );

	it( 'strokes a bus wider than a wire', () =>
	{
		const { context, calls } = record();

		draw( context, element( { from : { x : 1, y : 1 }, to : { x : 3, y : 1 }, potentials : [ 5, 5 ] } ), { voltage : 5, current : 0 } );
		draw( context, element( { from : { x : 1, y : 1 }, to : { x : 3, y : 1 }, potentials : [ 5, 5 ], lanes : 8 } ), { voltage : 5, current : 0 } );

		expect( calls.filter( call => call[ 1 ] === 'lineWidth' ) ).toEqual( [ [ 'set', 'lineWidth', 2 ], [ 'set', 'lineWidth', BUS ] ] );
	} );

	it( 'strokes a bus in the color of its lanes', () =>
	{
		const { context, calls } = record();

		draw( context, element( { from : { x : 1, y : 1 }, to : { x : 3, y : 1 }, potentials : [ 0, 5, 0 ], lanes : 3 } ), { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'set', 'strokeStyle', bus( [ 0, 5, 0 ], 5 ) ] );
	} );
} );
