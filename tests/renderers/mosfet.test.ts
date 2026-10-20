import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { draw } from '../../src/renderers/mosfet.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'mosfet renderer', () =>
{
	const nmos = element( { kind : Kind.Nmos, from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, potentials : [ 2, 2.5, 0 ], currents : [ 0, 2.5e-3, -2.5e-3 ] } );

	function drawn( kind : Kind ) : unknown[][]
	{
		const { context, calls } = record();

		draw( context, { ... nmos, kind }, { voltage : 5, current : 0 } );

		return calls;
	}

	it( 'colors the gate, the drain and the source leads with their voltages', () =>
	{
		const strokes = drawn( Kind.Nmos ).filter( call => call[ 1 ] === 'strokeStyle' ).map( call => call[ 2 ] );

		expect( strokes.slice( 0, 4 ) ).toEqual( [ color( 2, 5 ), color( 2, 5 ), color( 2.5, 5 ), color( 0, 5 ) ] );
	} );

	it( 'draws the gate apart from the channel and the drain and the source out from its ends', () =>
	{
		const calls = drawn( Kind.Nmos );

		expect( calls ).toContainEqual( [ 'moveTo', 0, 0 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 34, 0 ] );
		expect( calls ).toContainEqual( [ 'moveTo', 34, -20 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 34, 20 ] );
		expect( calls ).toContainEqual( [ 'moveTo', 40, -20 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 60, -20 ] );
		expect( calls ).toContainEqual( [ 'moveTo', 40, 20 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 60, 20 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 40, 20 ] );
	} );

	it( 'points the arrow of an nmos out of the source and the arrow of a pmos into it', () =>
	{
		const tip = ( kind : Kind ) => drawn( kind ).filter( call => call[ 0 ] === 'moveTo' ).at( -1 )!.slice( 1 );

		expect( tip( Kind.Nmos ) ).toEqual( [ 55, 20 ] );
		expect( tip( Kind.Pmos ) ).toEqual( [ 45, 20 ] );
	} );
} );
