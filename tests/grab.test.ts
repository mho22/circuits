import { describe, it, expect } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { grab } from '../src/grab.ts';


describe( 'grab', () =>
{
	const area = { low : { x : -8, y : -8 }, high : { x : 8, y : 8 } };

	it( 'grabs the shapes with every bound inside the area or on its edge', () =>
	{
		const shapes = [
			{ figure : Figure.Line as const, from : { x : -8, y : 0 }, to : { x : 8, y : 0 } },
			{ figure : Figure.Line as const, from : { x : 0, y : 0 }, to : { x : 10, y : 0 } },
			{ figure : Figure.Circle as const, center : { x : 0, y : 0 }, radius : 8 },
			{ figure : Figure.Circle as const, center : { x : 2, y : 0 }, radius : 8 },
		];

		expect( grab( shapes, [], area ).shapes ).toEqual( [ 0, 2 ] );
	} );

	it( 'grabs the pins whose grid step is inside the area', () =>
	{
		const pins = [ { point : { x : -2, y : 0 }, output : false, label : 'foo' }, { point : { x : 3, y : 0 }, output : true, label : 'bar' } ];

		expect( grab( [], pins, area ) ).toEqual( { shapes : [], pins : [ 0 ] } );
	} );
} );
