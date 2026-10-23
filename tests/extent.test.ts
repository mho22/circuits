import { describe, it, expect } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { extent } from '../src/extent.ts';


describe( 'extent', () =>
{
	it( 'gives nothing without a shape and a pin', () =>
	{
		expect( extent( [], [] ) ).toBeNull();
	} );

	it( 'covers the shapes in whole grid steps and the pins with one step around them', () =>
	{
		const shapes = [ { figure : Figure.Circle as const, center : { x : 0, y : 0 }, radius : 6 } ];
		const pins = [ { point : { x : -4, y : 0 }, output : false, label : 'foo' } ];

		expect( extent( shapes, pins ) ).toEqual( { low : { x : -5, y : -3 }, high : { x : 3, y : 3 } } );
	} );
} );
