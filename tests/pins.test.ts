import { describe, it, expect } from 'vitest';
import { pins } from '../src/pins.ts';


describe( 'pins', () =>
{
	it( 'places each pin of an outline around a center after the turn', () =>
	{
		const outline = { low : { x : -2, y : -2 }, high : { x : 2, y : 2 }, pins : [ { x : -2, y : 0 }, { x : 2, y : 0 } ] };

		expect( pins( { x : 5, y : 5 }, outline, 1 ) ).toEqual( [ { x : 5, y : 3 }, { x : 5, y : 7 } ] );
	} );
} );
