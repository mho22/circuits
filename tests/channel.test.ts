import { describe, it, expect } from 'vitest';
import { channel } from '../src/channel.ts';


describe( 'channel', () =>
{
	it( 'gives the drain end and the source end of the channel at the bar', () =>
	{
		expect( channel( { x : 0, y : 0 }, { x : 2, y : 0 } ) ).toEqual( [ { x : 2, y : -1 }, { x : 2, y : 1 } ] );
		expect( channel( { x : 0, y : 0 }, { x : 0, y : 2 } ) ).toEqual( [ { x : 1, y : 2 }, { x : -1, y : 2 } ] );
		expect( channel( { x : 0, y : 0 }, { x : 2, y : 2 } ) ).toEqual( [ { x : 3, y : 1 }, { x : 1, y : 3 } ] );
	} );
} );
