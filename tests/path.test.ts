import { describe, it, expect } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { path } from '../src/path.ts';


describe( 'path', () =>
{
	it( 'goes from one end of a line to the other and gives no point for a circle or a curve', () =>
	{
		expect( path( { figure : Figure.Line, from : { x : 0, y : 0 }, to : { x : 3, y : 1 } } ) ).toEqual( [ { x : 0, y : 0 }, { x : 3, y : 1 } ] );
		expect( path( { figure : Figure.Circle, center : { x : 0, y : 0 }, radius : 2 } ) ).toEqual( [] );
		expect( path( { figure : Figure.Curve, from : { x : 0, y : 0 }, through : { x : 1, y : 1 }, to : { x : 2, y : 0 } } ) ).toEqual( [] );
	} );
} );
