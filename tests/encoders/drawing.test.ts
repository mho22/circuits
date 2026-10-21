import { describe, it, expect } from 'vitest';
import { encode } from '../../src/encoders/drawing.ts';
import { Figure } from '../../src/enums/figure.ts';


describe( 'drawing encoder', () =>
{
	it( 'writes the figure then its numbers for each shape', () =>
	{
		const shapes = [
			{ figure : Figure.Circle as const, center : { x : 0, y : 1 }, radius : 2 },
			{ figure : Figure.Line as const, from : { x : -1, y : 0 }, to : { x : 1, y : 1 } },
			{ figure : Figure.Curve as const, from : { x : 0, y : -4 }, through : { x : 4, y : 0 }, to : { x : 0, y : 4 } },
		];

		expect( Array.from( encode( shapes ) ) ).toEqual( [ 1, 0, 1, 2, 2, -1, 0, 1, 1, 3, 0, -4, 4, 0, 0, 4 ] );
	} );

	it( 'writes nothing for an empty drawing', () =>
	{
		expect( encode( [] ) ).toEqual( new Int32Array() );
	} );
} );
