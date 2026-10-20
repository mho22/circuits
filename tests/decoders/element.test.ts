import { describe, it, expect } from 'vitest';
import { decode } from '../../src/decoders/element.ts';
import { Kind } from '../../src/enums/kind.ts';


describe( 'element decoder', () =>
{
	it( 'reads ten numbers per part', () =>
	{
		const data = new Float64Array( [ 4, 1, 0, 1, 2, 3, 1000, 5, 0, 0.005, 7, 3, 2, 2, 2, 2, 0, 0, 0, -0.1 ] );

		expect( decode( data ) ).toEqual( [
			{ id : 4, kind : Kind.Resistor, from : { x : 0, y : 1 }, to : { x : 2, y : 3 }, value : 1000, potentials : [ 5, 0 ], current : 0.005 },
			{ id : 7, kind : Kind.Ground, from : { x : 2, y : 2 }, to : { x : 2, y : 2 }, value : 0, potentials : [ 0, 0 ], current : -0.1 },
		] );
	} );

	it( 'reads no part from empty data', () =>
	{
		expect( decode( new Float64Array() ) ).toEqual( [] );
	} );
} );
