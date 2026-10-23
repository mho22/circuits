import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/pin.ts';
import { FROZEN } from '../../src/scale.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'pin renderer', () =>
{
	const scale = { voltage : 5, current : 0 };
	const input = element( { kind : Kind.Input, from : { x : 1, y : 1 }, to : { x : 2, y : 1 }, potentials : [ 5, 5 ] } );

	it( 'draws an input as a square on its start with a lead to its end, in the color of its voltage', () =>
	{
		const { context, calls } = record();

		draw( context, input, scale );

		expect( calls ).toContainEqual( [ 'set', 'strokeStyle', color( 5, 5 ) ] );
		expect( calls ).toContainEqual( [ 'moveTo', SPACING + 7, SPACING ] );
		expect( calls ).toContainEqual( [ 'lineTo', 2 * SPACING, SPACING ] );
		expect( calls ).toContainEqual( [ 'roundRect', SPACING - 7, SPACING - 7, 14, 14, 0 ] );
	} );

	it( 'draws an output as a rounded square', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Output, from : { x : 1, y : 1 }, to : { x : 1, y : 0 } } ), scale );

		expect( calls ).toContainEqual( [ 'roundRect', SPACING - 7, SPACING - 7, 14, 14, 4 ] );
		expect( calls ).toContainEqual( [ 'moveTo', SPACING, SPACING - 7 ] );
	} );

	it( 'writes its bit inside the square: 1 above 2.5 V, 0 at or below', () =>
	{
		const { context, calls } = record();

		draw( context, input, scale );
		draw( context, { ... input, potentials : [ 2.5, 2.5 ] }, scale );

		expect( calls.filter( call => call[ 0 ] === 'fillText' ).map( call => call[ 1 ] ) ).toEqual( [ '1', '0' ] );
		expect( calls ).toContainEqual( [ 'set', 'font', '12px ui-monospace, monospace' ] );
	} );

	it( 'writes no bit while the view is frozen', () =>
	{
		const { context, calls } = record();

		draw( context, input, FROZEN );

		expect( calls.filter( call => call[ 0 ] === 'fillText' ).map( call => call[ 1 ] ) ).toEqual( [ '' ] );
	} );

	it( 'centers the drawn glyph of its bit on the square', () =>
	{
		const { context, calls } = record();
		const metrics = { actualBoundingBoxLeft : 2, actualBoundingBoxRight : 3, actualBoundingBoxAscent : 4, actualBoundingBoxDescent : 3 };
		const measured = new Proxy( context, { get : ( target, name ) => name === 'measureText' ? () => metrics : Reflect.get( target, name ) } );

		draw( measured, input, scale );

		expect( calls ).toContainEqual( [ 'fillText', '1', SPACING - 0.5, SPACING + 0.5 ] );
	} );

	it( 'fills the square of an input and writes its bit in black, and leaves an output hollow', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Output, from : { x : 1, y : 1 }, to : { x : 1, y : 0 } } ), scale );

		expect( calls.some( call => call[ 0 ] === 'fill' ) ).toBe( false );

		draw( context, input, scale );

		expect( calls.filter( call => call[ 0 ] === 'fill' ).length ).toBe( 1 );
		expect( calls ).toContainEqual( [ 'set', 'fillStyle', '#000000' ] );
	} );

	it( 'writes its label on the side away from the lead', () =>
	{
		const { context, calls } = record();

		draw( context, { ... input, label : 'foo' }, scale );

		expect( calls ).toContainEqual( [ 'fillText', 'foo', SPACING - 15, SPACING ] );
		expect( calls ).toContainEqual( [ 'set', 'textAlign', 'right' ] );
	} );
} );
