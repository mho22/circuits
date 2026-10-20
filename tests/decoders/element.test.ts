import { describe, it, expect } from 'vitest';
import { decode } from '../../src/decoders/element.ts';
import { Kind } from '../../src/enums/kind.ts';


describe( 'element decoder', () =>
{
	it( 'reads thirteen numbers per part, a mirrored transistor, and one voltage and one current per terminal', () =>
	{
		const data = new Float64Array( [
			4, 1, 0, 1, 2, 3, 1000, 5, 0, NaN, 0.005, -0.005, NaN,
			7, 3, 2, 2, 2, 2, 0, 0, 0, NaN, -0.1, 0.1, NaN,
			9, 8, 0, 0, 2, 0, 0, 0.7, 3, 0, 2e-5, 2e-3, -2.02e-3,
			11, 10, 0, 0, 2, 0, 1, 5, 0, 0, 0, 0, 0,
		] );

		expect( decode( data ) ).toEqual( [
			{ id : 4, kind : Kind.Resistor, from : { x : 0, y : 1 }, to : { x : 2, y : 3 }, value : 1000, mirrored : false, potentials : [ 5, 0 ], currents : [ 0.005, -0.005 ] },
			{ id : 7, kind : Kind.Ground, from : { x : 2, y : 2 }, to : { x : 2, y : 2 }, value : 0, mirrored : false, potentials : [ 0, 0 ], currents : [ -0.1, 0.1 ] },
			{ id : 9, kind : Kind.Npn, from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, value : 0, mirrored : false, potentials : [ 0.7, 3, 0 ], currents : [ 2e-5, 2e-3, -2.02e-3 ] },
			{ id : 11, kind : Kind.Nmos, from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, value : 1, mirrored : true, potentials : [ 5, 0, 0 ], currents : [ 0, 0, 0 ] },
		] );
	} );

	it( 'reads no part from empty data', () =>
	{
		expect( decode( new Float64Array() ) ).toEqual( [] );
	} );
} );
