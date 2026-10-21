import { describe, it, expect } from 'vitest';
import { bit } from '../src/bit.ts';


describe( 'bit', () =>
{
	it( 'reads 1 above 2.5 V and 0 at or below it', () =>
	{
		expect( [ 5, 2.6, 2.5, 0, -5 ].map( bit ) ).toEqual( [ 1, 1, 0, 0, 0 ] );
	} );
} );
