import { describe, it, expect } from 'vitest';
import { arc } from '../src/arc.ts';


describe( 'arc', () =>
{
	it( 'finds the circle through three points and the way from the first to the last through the middle one', () =>
	{
		expect( arc( { x : 0, y : -2 }, { x : 2, y : 0 }, { x : 0, y : 2 } ) ).toEqual( { center : { x : 0, y : 0 }, radius : 2, start : - Math.PI / 2, end : Math.PI / 2, counterclockwise : false } );
		expect( arc( { x : 2, y : -1 }, { x : 0, y : 1 }, { x : 2, y : 3 } ) ).toEqual( { center : { x : 2, y : 1 }, radius : 2, start : - Math.PI / 2, end : Math.PI / 2, counterclockwise : true } );
	} );

	it( 'finds no circle through three points on one line', () =>
	{
		expect( arc( { x : 0, y : 0 }, { x : 1, y : 1 }, { x : 2, y : 2 } ) ).toBeNull();
		expect( arc( { x : 0, y : 0 }, { x : 1, y : 1 }, { x : 0, y : 0 } ) ).toBeNull();
	} );
} );
