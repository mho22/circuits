import { describe, it, expect } from 'vitest';
import { side } from '../src/side.ts';


describe( 'side', () =>
{
	it( 'tells on which side of a line a point is, or that it is on the line', () =>
	{
		const from = { x : 0, y : 0 };
		const to = { x : 4, y : 0 };

		expect( [ { x : 2, y : 3 }, { x : 2, y : -3 }, { x : 6, y : 0 } ].map( point => side( point, from, to ) ) ).toEqual( [ 1, -1, 0 ] );
	} );
} );
