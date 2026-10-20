import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { draw } from '../../src/renderers/capacitor.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'capacitor renderer', () =>
{
	const capacitor = element( { kind : Kind.Capacitor, value : 1e-5, from : { x : 0, y : 0 }, to : { x : 4, y : 0 }, potentials : [ 5, 0 ] } );

	it( 'colors each lead and its plate with the voltage of its end', () =>
	{
		const { context, calls } = record();

		draw( context, capacitor, { voltage : 5, current : 0 } );

		const strokes = calls.filter( call => call[ 1 ] === 'strokeStyle' ).map( call => call[ 2 ] );

		expect( strokes ).toEqual( [ color( 5, 5 ), color( 0, 5 ), color( 5, 5 ), color( 0, 5 ) ] );
	} );

	it( 'draws two parallel plates across the middle', () =>
	{
		const { context, calls } = record();

		draw( context, capacitor, { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'lineTo', 36, 0 ] );
		expect( calls ).toContainEqual( [ 'moveTo', 44, 0 ] );
		expect( calls ).toContainEqual( [ 'moveTo', 36, -12 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 36, 12 ] );
		expect( calls ).toContainEqual( [ 'moveTo', 44, -12 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 44, 12 ] );
	} );

	it( 'labels the capacitor with its value', () =>
	{
		const { context, calls } = record();

		draw( context, capacitor, { voltage : 5, current : 0 } );

		expect( calls.some( call => call[ 0 ] === 'fillText' && call[ 1 ] === '10 µF' ) ).toBe( true );
	} );
} );
