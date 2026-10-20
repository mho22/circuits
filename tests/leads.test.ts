import { describe, it, expect } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { leads } from '../src/leads.ts';
import { element } from './stubs/element.ts';


describe( 'leads', () =>
{
	it( 'gives a two-ended part one lead with its current', () =>
	{
		expect( leads( element( { kind : Kind.Resistor, currents : [ 0.005, -0.005 ] } ) ) ).toEqual( [ { from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, current : 0.005 } ] );
	} );

	it( 'gives a transistor one lead per terminal, all running toward the emitter', () =>
	{
		const transistor = element( { kind : Kind.Npn, to : { x : 2, y : 0 }, currents : [ 2e-5, 2e-3, -2.02e-3 ] } );

		expect( leads( transistor ) ).toEqual( [
			{ from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, current : 2e-5 },
			{ from : { x : 3, y : -1 }, to : { x : 2, y : 0 }, current : 2e-3 },
			{ from : { x : 2, y : 0 }, to : { x : 3, y : 1 }, current : 2.02e-3 },
		] );
	} );
} );
