import { describe, it, expect } from 'vitest';
import { channel } from '../src/channel.ts';


describe( 'channel', () =>
{
	it( 'gives the drain end and the source end of the channel at the bar', () =>
	{
		expect( channel( { x : 0, y : 0 }, { x : 2, y : 0 }, false ) ).toEqual( [ { x : 2, y : -1 }, { x : 2, y : 1 } ] );
		expect( channel( { x : 0, y : 0 }, { x : 0, y : 2 }, false ) ).toEqual( [ { x : 1, y : 2 }, { x : -1, y : 2 } ] );
	} );

	it( 'swaps the two ends of a mirrored channel', () =>
	{
		expect( channel( { x : 0, y : 0 }, { x : 2, y : 0 }, true ) ).toEqual( [ { x : 2, y : 1 }, { x : 2, y : -1 } ] );
	} );
} );
