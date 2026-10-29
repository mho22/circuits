import { describe, it, expect } from 'vitest';
import { Color } from '../../src/enums/color.ts';
import { draw } from '../../src/renderers/board.ts';
import { record } from '../stubs/context.ts';


describe( 'board renderer', () =>
{
	const board = { x : 100, y : 100, unit : 10 };
	const pins = [ { point : { x : -2, y : 0 }, output : false, label : 'foo' }, { point : { x : 2, y : 1 }, output : true, label : 'bar' } ];

	it( 'draws no dots of its own', () =>
	{
		const { context, calls } = record();

		draw( context, board, [], null );

		expect( calls.some( call => call[ 0 ] === 'fillRect' ) ).toBe( false );
	} );

	it( 'draws an input as a square and an output as a rounded square, each with its label outside', () =>
	{
		const { context, calls } = record();

		draw( context, board, pins, null );

		expect( calls.filter( call => call[ 0 ] === 'roundRect' ) ).toEqual( [ [ 'roundRect', 14, 94, 12, 12, 0 ], [ 'roundRect', 174, 134, 12, 12, 3 ] ] );
		expect( calls.filter( call => call[ 0 ] === 'fillText' ) ).toEqual( [ [ 'fillText', 'foo', 8, 100 ], [ 'fillText', 'bar', 192, 140 ] ] );
	} );

	it( 'draws a light as a dark filled square of the size of a pin without a border and without a label', () =>
	{
		const { context, calls } = record();

		draw( context, board, [ { point : { x : 1, y : -1 }, output : false, label : '', light : Color.Blue } ], null );

		expect( calls ).toContainEqual( [ 'set', 'fillStyle', '#1a1a1a' ] );
		expect( calls ).toContainEqual( [ 'fillRect', 131, 51, 18, 18 ] );
		expect( calls.some( call => call[ 0 ] === 'roundRect' || call[ 0 ] === 'fillText' ) ).toBe( false );
	} );

	it( 'rings the fine point under the cursor', () =>
	{
		const { context, calls } = record();

		draw( context, board, [], { x : 1, y : -1 } );

		expect( calls.slice( -5 ) ).toEqual( [ [ 'set', 'strokeStyle', '#ffff00' ], [ 'set', 'lineWidth', 1 ], [ 'beginPath' ], [ 'arc', 110, 90, 5, 0, 2 * Math.PI ], [ 'stroke' ] ] );
	} );
} );
