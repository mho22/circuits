import { describe, it, expect } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { hit } from '../src/hit.ts';


describe( 'hit', () =>
{
	it( 'reaches a rectangle on its edges only', () =>
	{
		const shape = { figure : Figure.Rectangle as const, from : { x : 0, y : 0 }, to : { x : 8, y : 6 } };

		expect( [ { x : 4, y : 0 }, { x : 8, y : 3 }, { x : 4, y : 7 }, { x : 4, y : 3 }, { x : 10, y : 3 } ].map( point => hit( shape, point ) ) ).toEqual( [ true, true, true, false, false ] );
	} );

	it( 'reaches a circle near its ring only', () =>
	{
		const shape = { figure : Figure.Circle as const, center : { x : 0, y : 0 }, radius : 4 };

		expect( [ { x : 4, y : 0 }, { x : 0, y : -5 }, { x : 1, y : 1 }, { x : 6, y : 0 } ].map( point => hit( shape, point ) ) ).toEqual( [ true, true, false, false ] );
	} );

	it( 'reaches a line along its length but not past its ends', () =>
	{
		const shape = { figure : Figure.Line as const, from : { x : 0, y : 0 }, to : { x : 4, y : 0 } };

		expect( [ { x : 2, y : 1 }, { x : 4, y : 0 }, { x : -2, y : 0 }, { x : 2, y : 3 } ].map( point => hit( shape, point ) ) ).toEqual( [ true, true, false, false ] );
	} );
} );
