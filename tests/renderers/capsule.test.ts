import { describe, it, expect } from 'vitest';
import { Kind } from '../../src/enums/kind.ts';
import { SPACING } from '../../src/grid.ts';
import { caption, draw, HALF, WIDTH } from '../../src/renderers/capsule.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'capsule renderer', () =>
{
	function drawn( to : { x : number, y : number } ) : unknown[][]
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Clock, from : { x : 1, y : 1 }, to } ), '#ff0000' );

		return calls;
	}

	it( 'draws a filled capsule around the point in its tint', () =>
	{
		const calls = drawn( { x : 3, y : 1 } );

		expect( calls ).toContainEqual( [ 'roundRect', SPACING - WIDTH, SPACING - HALF, 2 * WIDTH, 2 * HALF, HALF ] );
		expect( calls ).toContainEqual( [ 'set', 'strokeStyle', '#ff0000' ] );
		expect( calls.filter( call => call[ 0 ] === 'fill' ) ).toHaveLength( 1 );
	} );

	it( 'draws a lead from the edge of the capsule to the end of the part', () =>
	{
		expect( drawn( { x : 3, y : 1 } ) ).toContainEqual( [ 'moveTo', SPACING + WIDTH, SPACING ] );
		expect( drawn( { x : 3, y : 1 } ) ).toContainEqual( [ 'lineTo', 3 * SPACING, SPACING ] );
		expect( drawn( { x : 1, y : 3 } ) ).toContainEqual( [ 'moveTo', SPACING, SPACING + HALF ] );
	} );

	it( 'draws a wider capsule and starts its lead at its edge', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Slider, from : { x : 1, y : 1 }, to : { x : 3, y : 1 } } ), '#ff0000', 21 );

		expect( calls ).toContainEqual( [ 'roundRect', SPACING - 21, SPACING - HALF, 42, 2 * HALF, HALF ] );
		expect( calls ).toContainEqual( [ 'moveTo', SPACING + 21, SPACING ] );
	} );

	it( 'writes a caption past the edge of the capsule on the side away from the lead, and nothing without one', () =>
	{
		const written = ( label : string, width? : number ) =>
		{
			const { context, calls } = record();

			caption( context, element( { kind : Kind.Toggle, from : { x : 1, y : 1 }, to : { x : 2, y : 1 }, label } ), width );

			return calls.filter( call => call[ 0 ] === 'fillText' );
		};

		expect( written( 'foo' ) ).toEqual( [ [ 'fillText', 'foo', SPACING - WIDTH - 8, SPACING ] ] );
		expect( written( 'foo', 21 ) ).toEqual( [ [ 'fillText', 'foo', SPACING - 29, SPACING ] ] );
		expect( written( '' ) ).toEqual( [] );
	} );
} );
