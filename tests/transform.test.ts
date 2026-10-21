import { describe, it, expect } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { rotate, shift } from '../src/transform.ts';


describe( 'transform', () =>
{
	const circle = { figure : Figure.Circle as const, center : { x : 1, y : 1 }, radius : 2 };
	const rectangle = { figure : Figure.Rectangle as const, from : { x : 0, y : 0 }, to : { x : 4, y : 2 } };
	const line = { figure : Figure.Line as const, from : { x : 0, y : 0 }, to : { x : 4, y : 2 } };

	it( 'shifts every point of a shape by an offset', () =>
	{
		const offset = { x : 2, y : -1 };

		expect( shift( circle, offset ) ).toEqual( { ... circle, center : { x : 3, y : 0 } } );
		expect( shift( line, offset ) ).toEqual( { ... line, from : { x : 2, y : -1 }, to : { x : 6, y : 1 } } );
	} );

	it( 'turns a rectangle a quarter turn around its middle and leaves a circle', () =>
	{
		expect( rotate( rectangle ) ).toEqual( { ... rectangle, from : { x : 3, y : -1 }, to : { x : 1, y : 3 } } );
		expect( rotate( circle ) ).toBe( circle );
	} );
} );
