import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/resistor.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'resistor renderer', () =>
{
	const resistor = element( { kind : Kind.Resistor, value : 1000, from : { x : 0, y : 0 }, to : { x : 4, y : 0 }, potentials : [ 5, 0 ] } );

	it( 'colors the resistor from the voltage at one end to the voltage at the other', () =>
	{
		const { context, calls } = record();

		draw( context, resistor, { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'createLinearGradient', 0, 0, SPACING * 4, 0 ] );
		expect( calls ).toContainEqual( [ 'addColorStop', 0, color( 5, 5 ) ] );
		expect( calls ).toContainEqual( [ 'addColorStop', 1, color( 0, 5 ) ] );
	} );

	it( 'draws leads and a zigzag body in the middle', () =>
	{
		const { context, calls } = record();

		draw( context, resistor, { voltage : 5, current : 0 } );

		const points = calls.filter( call => call[ 0 ] === 'moveTo' || call[ 0 ] === 'lineTo' ).slice( 0, 10 );

		expect( points.at( 0 ) ).toEqual( [ 'moveTo', 0, 0 ] );
		expect( points.at( 1 ) ).toEqual( [ 'lineTo', 20, 0 ] );
		expect( points.at( 2 ) ).toEqual( [ 'lineTo', 20 + 40 / 12, 6 ] );
		expect( points.at( 3 ) ).toEqual( [ 'lineTo', 20 + 40 * 1.5 / 6, -6 ] );
		expect( points.at( 8 ) ).toEqual( [ 'lineTo', 60, 0 ] );
		expect( points.at( 9 ) ).toEqual( [ 'lineTo', SPACING * 4, 0 ] );
	} );

	it( 'shrinks the body to fit a short resistor', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Resistor, value : 1000, to : { x : 1, y : 0 } } ), { voltage : 0, current : 0 } );

		expect( calls ).toContainEqual( [ 'lineTo', 0, 0 ] );
		expect( calls ).toContainEqual( [ 'lineTo', SPACING, 0 ] );
	} );

	it( 'labels the resistor with its value', () =>
	{
		const { context, calls } = record();

		draw( context, resistor, { voltage : 5, current : 0 } );

		expect( calls.some( call => call[ 0 ] === 'fillText' && call[ 1 ] === '1 kΩ' ) ).toBe( true );
	} );
} );
