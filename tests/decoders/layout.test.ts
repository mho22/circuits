import { describe, it, expect } from 'vitest';
import { decode } from '../../src/decoders/layout.ts';


describe( 'layout decoder', () =>
{
	it( 'reads two numbers per point', () =>
	{
		expect( decode( new Int32Array( [ -2, 1, 3, 0 ] ) ) ).toEqual( [ { x : -2, y : 1 }, { x : 3, y : 0 } ] );
	} );

	it( 'reads no point from empty data', () =>
	{
		expect( decode( [] ) ).toEqual( [] );
	} );
} );
