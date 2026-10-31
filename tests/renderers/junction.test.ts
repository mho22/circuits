import { describe, it, expect } from 'vitest';
import { bus, color } from '../../src/colors/voltage.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/junction.ts';
import { record } from '../stubs/context.ts';


describe( 'junction renderer', () =>
{
	it( 'fills a dot on the point in the color of its voltage', () =>
	{
		const { context, calls } = record();

		draw( context, [ { point : { x : 1, y : 2 }, potentials : [ 5, 5 ] } ], { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'set', 'fillStyle', color( 5, 5 ) ] );
		expect( calls ).toContainEqual( [ 'arc', SPACING, 2 * SPACING, 3, 0, 2 * Math.PI ] );
		expect( calls.at( -1 ) ).toEqual( [ 'fill' ] );
	} );

	it( 'fills the dots of one color in one path', () =>
	{
		const { context, calls } = record();

		draw( context, [ { point : { x : 1, y : 2 }, potentials : [ 5, 5 ] }, { point : { x : 3, y : 4 }, potentials : [ 0, 0 ] }, { point : { x : 5, y : 6 }, potentials : [ 5, 5 ] } ], { voltage : 5, current : 0 } );

		expect( calls.filter( call => call[ 1 ] === 'fillStyle' ).map( call => call[ 2 ] ) ).toEqual( [ color( 5, 5 ), color( 0, 5 ) ] );
		expect( calls.filter( call => call[ 0 ] === 'fill' ) ).toHaveLength( 2 );
		expect( calls.filter( call => call[ 0 ] === 'arc' ).map( call => call.slice( 1, 3 ) ) ).toEqual( [ [ SPACING, 2 * SPACING ], [ 5 * SPACING, 6 * SPACING ], [ 3 * SPACING, 4 * SPACING ] ] );
	} );

	it( 'fills a dot on a bus in the color of its lanes', () =>
	{
		const { context, calls } = record();

		draw( context, [ { point : { x : 1, y : 2 }, potentials : [ 0, 5, 0 ], lanes : 3 } ], { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'set', 'fillStyle', bus( [ 0, 5, 0 ], 5 ) ] );
	} );
} );
