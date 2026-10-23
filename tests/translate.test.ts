import { describe, it, expect } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { translate } from '../src/translate.ts';


describe( 'translate', () =>
{
	const offset = { x : 2, y : -4 };

	it( 'moves every point of a line, a curve, a circle and a text', () =>
	{
		expect( translate( { figure : Figure.Line, from : { x : 0, y : 0 }, to : { x : 4, y : 0 } }, offset ) ).toEqual( { figure : Figure.Line, from : { x : 2, y : -4 }, to : { x : 6, y : -4 } } );
		expect( translate( { figure : Figure.Curve, from : { x : 0, y : -4 }, through : { x : 4, y : 0 }, to : { x : 0, y : 4 } }, offset ) ).toEqual( { figure : Figure.Curve, from : { x : 2, y : -8 }, through : { x : 6, y : -4 }, to : { x : 2, y : 0 } } );
		expect( translate( { figure : Figure.Circle, center : { x : 1, y : 1 }, radius : 2 }, offset ) ).toEqual( { figure : Figure.Circle, center : { x : 3, y : -3 }, radius : 2 } );
		expect( translate( { figure : Figure.Text, at : { x : 1, y : 1 }, content : 'foo' }, offset ) ).toEqual( { figure : Figure.Text, at : { x : 3, y : -3 }, content : 'foo' } );
	} );
} );
