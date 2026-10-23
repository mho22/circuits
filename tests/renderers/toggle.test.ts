import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/toggle.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'toggle renderer', () =>
{
	const x = SPACING;
	const y = SPACING;

	function drawn( value : number, potential : number, to = { x : 3, y : 1 }, listening = false, label = '' ) : unknown[][]
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Toggle, value, from : { x : 1, y : 1 }, to, potentials : [ 0, potential ], listening, label } ), { voltage : 5, current : 0 } );

		return calls;
	}

	it( 'draws a slot around the point in the color of its voltage', () =>
	{
		const calls = drawn( 1, 5 );

		expect( calls ).toContainEqual( [ 'roundRect', x - 11, y - 6, 22, 12, 6 ] );
		expect( calls ).toContainEqual( [ 'set', 'strokeStyle', color( 5, 5 ) ] );
	} );

	it( 'draws a lead from the edge of the slot to the end of the toggle', () =>
	{
		expect( drawn( 1, 5 ) ).toContainEqual( [ 'moveTo', x + 11, y ] );
		expect( drawn( 1, 5 ) ).toContainEqual( [ 'lineTo', 3 * SPACING, y ] );
		expect( drawn( 1, 5, { x : 1, y : 0 } ) ).toContainEqual( [ 'moveTo', x, y - 6 ] );
	} );

	it( 'puts the knob on the right at 1 and on the left at 0', () =>
	{
		expect( drawn( 1, 5 ).filter( call => call[ 0 ] === 'arc' ) ).toEqual( [ [ 'arc', x + 5, y, 3, 0, 2 * Math.PI ] ] );
		expect( drawn( 0, 0 ).filter( call => call[ 0 ] === 'arc' ) ).toEqual( [ [ 'arc', x - 5, y, 3, 0, 2 * Math.PI ] ] );
	} );

	it( 'puts the knob of a toggle that reads its wire by the level of the wire', () =>
	{
		expect( drawn( 0, 5, undefined, true ).filter( call => call[ 0 ] === 'arc' ) ).toEqual( [ [ 'arc', x + 5, y, 3, 0, 2 * Math.PI ] ] );
		expect( drawn( 1, 0, undefined, true ).filter( call => call[ 0 ] === 'arc' ) ).toEqual( [ [ 'arc', x - 5, y, 3, 0, 2 * Math.PI ] ] );
	} );

	it( 'writes the label of its pin on the side away from the lead', () =>
	{
		expect( drawn( 1, 5, undefined, false, 'foo' ) ).toContainEqual( [ 'fillText', 'foo', x - 19, y ] );
		expect( drawn( 1, 5, undefined, false, 'foo' ) ).toContainEqual( [ 'set', 'textAlign', 'right' ] );
		expect( drawn( 1, 5, undefined, false, 'foo' ) ).toContainEqual( [ 'set', 'font', '12px ui-monospace, monospace' ] );
		expect( drawn( 1, 5, { x : 1, y : 0 }, false, 'foo' ) ).toContainEqual( [ 'fillText', 'foo', x, y + 14 ] );
		expect( drawn( 1, 5 ).filter( call => call[ 0 ] === 'fillText' ) ).toEqual( [] );
	} );
} );
