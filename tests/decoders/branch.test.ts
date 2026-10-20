import { describe, it, expect } from 'vitest';
import { decode } from '../../src/decoders/branch.ts';


describe( 'branch decoder', () =>
{
	it( 'reads six numbers per branch', () =>
	{
		const data = new Float64Array( [ 0, 0, 2, 0, 5, 0.01, 2, 0, 2, 4, 5, -0.01 ] );

		expect( decode( data ) ).toEqual( [
			{ from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, voltage : 5, current : 0.01 },
			{ from : { x : 2, y : 0 }, to : { x : 2, y : 4 }, voltage : 5, current : -0.01 },
		] );
	} );

	it( 'reads no branch from empty data', () =>
	{
		expect( decode( new Float64Array() ) ).toEqual( [] );
	} );
} );
