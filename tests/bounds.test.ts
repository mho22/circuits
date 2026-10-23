import { describe, it, expect } from 'vitest';
import { bounds } from '../src/bounds.ts';
import { Figure } from '../src/enums/figure.ts';


describe( 'bounds', () =>
{
	it( 'bounds a line by its ends', () =>
	{
		expect( bounds( { figure : Figure.Line, from : { x : 0, y : 0 }, to : { x : 4, y : -2 } } ) ).toEqual( [ { x : 0, y : 0 }, { x : 4, y : -2 } ] );
	} );

	it( 'bounds a circle by the corners of the square around it', () =>
	{
		expect( bounds( { figure : Figure.Circle, center : { x : 1, y : 2 }, radius : 3 } ) ).toEqual( [ { x : -2, y : -1 }, { x : 4, y : 5 } ] );
	} );

	it( 'bounds a curve by its three points and by the extremes of its circle on the side it bulges', () =>
	{
		expect( bounds( { figure : Figure.Curve, from : { x : 0, y : -4 }, through : { x : 4, y : 0 }, to : { x : -4, y : 0 } } ) ).toEqual( [ { x : 0, y : -4 }, { x : 4, y : 0 }, { x : -4, y : 0 }, { x : 4, y : 0 }, { x : 0, y : 4 } ] );
		expect( bounds( { figure : Figure.Curve, from : { x : 0, y : -4 }, through : { x : 4, y : 0 }, to : { x : 0, y : 4 } } ) ).toEqual( [ { x : 0, y : -4 }, { x : 4, y : 0 }, { x : 0, y : 4 }, { x : 4, y : 0 } ] );
	} );

	it( 'bounds a text by a box that rises from its point and grows with each character', () =>
	{
		expect( bounds( { figure : Figure.Text, at : { x : 2, y : 4 }, content : 'f' } ) ).toEqual( [ { x : 2, y : 2 }, { x : 4, y : 5 } ] );
		expect( bounds( { figure : Figure.Text, at : { x : 2, y : 4 }, content : 'foo' } ) ).toEqual( [ { x : 2, y : 2 }, { x : 7, y : 5 } ] );
		expect( bounds( { figure : Figure.Text, at : { x : 2, y : 4 }, content : 'foobar' } ) ).toEqual( [ { x : 2, y : 2 }, { x : 11, y : 5 } ] );
		expect( bounds( { figure : Figure.Text, at : { x : 2, y : 4 }, content : 'éé' } ) ).toEqual( [ { x : 2, y : 2 }, { x : 5, y : 5 } ] );
	} );

	it( 'rounds the extremes of a curve to the fine grid', () =>
	{
		expect( bounds( { figure : Figure.Curve, from : { x : -4, y : -12 }, through : { x : 4, y : -2 }, to : { x : -4, y : 4 } } ) ).toEqual( [ { x : -4, y : -12 }, { x : 4, y : -2 }, { x : -4, y : 4 }, { x : 4, y : -4 } ] );
	} );
} );
