import { describe, it, expect } from 'vitest';
import { Kind } from '../../src/enums/kind.ts';
import { draw } from '../../src/renderers/led.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'led renderer', () =>
{
	function drawn( current : number, value = 0 ) : unknown[][]
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Led, value, from : { x : 0, y : 0 }, to : { x : 4, y : 0 }, potentials : [ 5, 3.1 ], currents : [ current, - current ] } ), { voltage : 5, current : 0 } );

		return calls;
	}

	it( 'draws a diode with two dark rays and no halo below 0.1 mA forward', () =>
	{
		for( const current of [ -1e-3, 5e-5, NaN ] )
		{
			const calls = drawn( current );

			expect( calls.filter( call => call[ 0 ] === 'arc' ) ).toEqual( [] );
			expect( calls.filter( call => call[ 1 ] === 'strokeStyle' ).slice( -2 ).map( call => call[ 2 ] ) ).toEqual( [ '#1a1a1a', '#1a1a1a' ] );
		}
	} );

	it( 'glows behind the diode in proportion to its current up to 5 mA', () =>
	{
		expect( drawn( 0.0025 ).filter( call => call[ 1 ] === 'fillStyle' )[ 0 ][ 2 ] ).toBe( 'rgba(255, 48, 48, 0.25)' );
		expect( drawn( 0.05 ).filter( call => call[ 1 ] === 'fillStyle' )[ 0 ][ 2 ] ).toBe( 'rgba(255, 48, 48, 0.5)' );
		expect( drawn( 0.01 ) ).toContainEqual( [ 'arc', 40, 0, 14, 0, 2 * Math.PI ] );
		expect( drawn( 0.01 ).filter( call => call[ 1 ] === 'strokeStyle' ).slice( -2 ).map( call => call[ 2 ] ) ).toEqual( [ 'rgb(255, 48, 48)', 'rgb(255, 48, 48)' ] );
	} );

	it( 'glows in the color of the led', () =>
	{
		expect( drawn( 0.01, 1 ).filter( call => call[ 1 ] === 'fillStyle' )[ 0 ][ 2 ] ).toBe( 'rgba(48, 220, 96, 0.5)' );
		expect( drawn( 0.01, 2 ).filter( call => call[ 1 ] === 'strokeStyle' ).slice( -1 )[ 0 ][ 2 ] ).toBe( 'rgb(48, 128, 255)' );
	} );
} );
