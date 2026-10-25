import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/slider.ts';
import { FROZEN } from '../../src/scale.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'slider renderer', () =>
{
	const x = SPACING;
	const y = SPACING;

	function drawn( value : number, potential : number, listening = false, label = '', scale = { voltage : 5, current : 0 } ) : unknown[][]
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Slider, value, from : { x : 1, y : 1 }, to : { x : 3, y : 1 }, potentials : [ 0, potential ], listening, label } ), scale );

		return calls;
	}

	it( 'draws a slot one dot wider than a toggle around the point in the color of its voltage', () =>
	{
		const calls = drawn( 5, 5 );

		expect( calls ).toContainEqual( [ 'roundRect', x - 21, y - 6, 42, 12, 6 ] );
		expect( calls ).toContainEqual( [ 'set', 'strokeStyle', color( 5, 5 ) ] );
		expect( calls ).toContainEqual( [ 'moveTo', x + 21, y ] );
	} );

	it( 'puts the knob along the slot by its voltage', () =>
	{
		expect( drawn( 0, 0 ).filter( call => call[ 0 ] === 'arc' ) ).toEqual( [ [ 'arc', x - 15, y, 3, 0, 2 * Math.PI ] ] );
		expect( drawn( 2.5, 2.5 ).filter( call => call[ 0 ] === 'arc' ) ).toEqual( [ [ 'arc', x, y, 3, 0, 2 * Math.PI ] ] );
		expect( drawn( 5, 5 ).filter( call => call[ 0 ] === 'arc' ) ).toEqual( [ [ 'arc', x + 15, y, 3, 0, 2 * Math.PI ] ] );
	} );

	it( 'puts the knob of a slider that reads its wire by the voltage of the wire, and draws none while the view is frozen', () =>
	{
		expect( drawn( 0, 5, true ).filter( call => call[ 0 ] === 'arc' ) ).toEqual( [ [ 'arc', x + 15, y, 3, 0, 2 * Math.PI ] ] );
		expect( drawn( 0, 5, true, '', FROZEN ).filter( call => call[ 0 ] === 'arc' ) ).toEqual( [] );
	} );

	it( 'writes the label of its pin past the edge of the slot on the side away from the lead', () =>
	{
		expect( drawn( 0, 0, true, 'foo' ) ).toContainEqual( [ 'fillText', 'foo', x - 29, y ] );
	} );
} );
