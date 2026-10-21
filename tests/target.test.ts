import { describe, it, expect } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { target } from '../src/target.ts';


describe( 'target', () =>
{
	const shapes = [
		{ figure : Figure.Line as const, from : { x : -8, y : 0 }, to : { x : 8, y : 0 } },
		{ figure : Figure.Circle as const, center : { x : 4, y : 0 }, radius : 4 },
	];
	const pins = [ { point : { x : -2, y : 0 }, output : false, label : 'foo' } ];

	it( 'finds a pin within one fine point of its place before any shape', () =>
	{
		expect( target( shapes, pins, { x : -7, y : 1 } ) ).toEqual( { pin : true, index : 0 } );
	} );

	it( 'finds the topmost shape under a point', () =>
	{
		expect( target( shapes, pins, { x : 0, y : 0 } ) ).toEqual( { pin : false, index : 1 } );
		expect( target( shapes, pins, { x : -4, y : 1 } ) ).toEqual( { pin : false, index : 0 } );
	} );

	it( 'finds nothing away from the pins and the shapes', () =>
	{
		expect( target( shapes, pins, { x : 0, y : 6 } ) ).toBeNull();
	} );
} );
