import { describe, it, expect } from 'vitest';
import { decode } from '../../src/decoders/outline.ts';


describe( 'outline decoder', () =>
{
	it( 'reads the low corner, the high corner, then two numbers per pin', () =>
	{
		expect( decode( new Int32Array( [ -2, -3, 2, 3, -2, 1, 2, 0 ] ) ) ).toEqual( { low : { x : -2, y : -3 }, high : { x : 2, y : 3 }, pins : [ { x : -2, y : 1 }, { x : 2, y : 0 } ] } );
	} );

	it( 'reads an empty outline from empty data', () =>
	{
		expect( decode( new Int32Array() ) ).toEqual( { low : { x : 0, y : 0 }, high : { x : 0, y : 0 }, pins : [] } );
	} );
} );
