import { describe, it, expect } from 'vitest';
import { turn } from '../src/turn.ts';


describe( 'turn', () =>
{
	it( 'turns a point a quarter clockwise per step on the screen', () =>
	{
		expect( [ 0, 1, 2, 3, 4 ].map( rotation => turn( { x : 2, y : 1 }, rotation ) ) ).toEqual( [ { x : 2, y : 1 }, { x : -1, y : 2 }, { x : -2, y : -1 }, { x : 1, y : -2 }, { x : 2, y : 1 } ] );
	} );
} );
