import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Figure } from '../../src/enums/figure.ts';
import { Kind } from '../../src/enums/kind.ts';
import { FINE, SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/symbol.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'symbol renderer', () =>
{
	const outline = { low : { x : -2, y : -3 }, high : { x : 2, y : 3 }, pins : [ { x : -2, y : 1 }, { x : 2, y : 0 } ] };

	it( 'draws a rounded box around its outline without a drawing and one dot per pin in the color of its voltage', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline, potentials : [ 5, 0 ] } ), { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'roundRect', 3 * SPACING, 2 * SPACING, 4 * SPACING, 6 * SPACING, 4 ] );
		expect( calls.filter( call => call[ 0 ] === 'arc' ).map( call => call.slice( 1, 3 ) ) ).toEqual( [ [ 3 * SPACING, 6 * SPACING ], [ 7 * SPACING, 5 * SPACING ] ] );
		expect( calls.filter( call => call[ 1 ] === 'fillStyle' ).map( call => call[ 2 ] ) ).toEqual( [ color( 5, 5 ), color( 0, 5 ) ] );
	} );

	it( 'swaps the width and the height of a box turned a quarter', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline, rotation : 1 } ), { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'roundRect', 2 * SPACING, 3 * SPACING, 6 * SPACING, 4 * SPACING, 4 ] );
		expect( calls.filter( call => call[ 0 ] === 'arc' ).map( call => call.slice( 1, 3 ) ) ).toEqual( [ [ 4 * SPACING, 3 * SPACING ], [ 5 * SPACING, 7 * SPACING ] ] );
	} );

	it( 'draws nothing before its outline is known', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Symbol } ), { voltage : 5, current : 0 } );

		expect( calls ).toEqual( [] );
	} );

	it( 'draws its drawing without a box, without its name and without the labels of its pins', () =>
	{
		const { context, calls } = record();
		const definition = { name : 'foo', drawing : [ { figure : Figure.Circle as const, center : { x : 0, y : 0 }, radius : 4 } ], labels : [ 'bar', '' ], layout : [], delay : 1e-9 };

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline, definition } ), { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'arc', 5 * SPACING, 5 * SPACING, 4 * SPACING / FINE, 0, 2 * Math.PI ] );
		expect( calls.some( call => call[ 0 ] === 'roundRect' ) ).toBe( false );
		expect( calls.some( call => call[ 0 ] === 'fillText' ) ).toBe( false );
	} );

	it( 'turns its drawing with the box', () =>
	{
		const { context, calls } = record();
		const definition = { name : '', drawing : [ { figure : Figure.Line as const, from : { x : 4, y : 0 }, to : { x : 8, y : 0 } } ], labels : [], layout : [], delay : 1e-9 };

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline, rotation : 1, definition } ), { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'moveTo', 5 * SPACING, 6 * SPACING ] );
		expect( calls ).toContainEqual( [ 'lineTo', 5 * SPACING, 7 * SPACING ] );
	} );
} );
