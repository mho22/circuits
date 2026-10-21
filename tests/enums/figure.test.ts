import { describe, it, expect } from 'vitest';
import { Figure, NAMES } from '../../src/enums/figure.ts';


describe( 'figure', () =>
{
	it( 'uses the codes of the engine binding', () =>
	{
		expect( [ Figure.Circle, Figure.Line, Figure.Curve ] ).toEqual( [ 1, 2, 3 ] );
	} );

	it( 'names every figure', () =>
	{
		expect( Object.values( NAMES ) ).toEqual( [ 'Circle', 'Line', 'Curve' ] );
	} );
} );
