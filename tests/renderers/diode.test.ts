import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { draw } from '../../src/renderers/diode.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'diode renderer', () =>
{
	const diode = element( { kind : Kind.Diode, from : { x : 0, y : 0 }, to : { x : 4, y : 0 }, potentials : [ 5, 4.35 ] } );

	it( 'colors each lead with the voltage of its end', () =>
	{
		const { context, calls } = record();

		draw( context, diode, { voltage : 5, current : 0 } );

		const strokes = calls.filter( call => call[ 1 ] === 'strokeStyle' ).map( call => call[ 2 ] );

		expect( strokes.slice( 0, 2 ) ).toEqual( [ color( 5, 5 ), color( 4.35, 5 ) ] );
	} );

	it( 'points the triangle from the anode to the bar of the cathode', () =>
	{
		const { context, calls } = record();

		draw( context, diode, { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'moveTo', 34, -8 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 34, 8 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 46, 0 ] );
		expect( calls.filter( call => call[ 0 ] === 'fill' ) ).toHaveLength( 1 );
		expect( calls ).toContainEqual( [ 'moveTo', 46, -8 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 46, 8 ] );
	} );

	it( 'draws no label', () =>
	{
		const { context, calls } = record();

		draw( context, diode, { voltage : 5, current : 0 } );

		expect( calls.some( call => call[ 0 ] === 'fillText' ) ).toBe( false );
	} );
} );
