import { describe, it, expect } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { hit } from '../src/hit.ts';


describe( 'hit', () =>
{
	it( 'reaches a circle near its ring only', () =>
	{
		const shape = { figure : Figure.Circle as const, center : { x : 0, y : 0 }, radius : 4 };

		expect( [ { x : 4, y : 0 }, { x : 0, y : -5 }, { x : 1, y : 1 }, { x : 6, y : 0 } ].map( point => hit( shape, point ) ) ).toEqual( [ true, true, false, false ] );
	} );

	it( 'reaches a curve near its ring on the side of the point it passes through only', () =>
	{
		const shape = { figure : Figure.Curve as const, from : { x : 0, y : -4 }, through : { x : 4, y : 0 }, to : { x : 0, y : 4 } };

		expect( [ { x : 4, y : 0 }, { x : 3, y : 3 }, { x : -4, y : 0 }, { x : 1, y : 0 } ].map( point => hit( shape, point ) ) ).toEqual( [ true, true, false, false ] );
	} );

	it( 'reaches a text inside its box only', () =>
	{
		const shape = { figure : Figure.Text as const, at : { x : 0, y : 0 }, content : 'foo' };

		expect( [ { x : 0, y : 0 }, { x : 4, y : 1 }, { x : 2, y : -1 }, { x : -1, y : 0 }, { x : 2, y : -2 }, { x : 2, y : 2 } ].map( point => hit( shape, point ) ) ).toEqual( [ true, true, true, false, false, false ] );
	} );

	it( 'reaches a line along its length but not past its ends', () =>
	{
		const shape = { figure : Figure.Line as const, from : { x : 0, y : 0 }, to : { x : 4, y : 0 } };

		expect( [ { x : 2, y : 1 }, { x : 4, y : 0 }, { x : -2, y : 0 }, { x : 2, y : 3 } ].map( point => hit( shape, point ) ) ).toEqual( [ true, true, false, false ] );
	} );
} );
