import { describe, it, expect } from 'vitest';
import { Figure, NAMES } from '../../src/enums/figure.ts';


describe( 'figure', () =>
{
	it( 'uses the codes of the engine binding', () =>
	{
		expect( [ Figure.Rectangle, Figure.Circle, Figure.Line ] ).toEqual( [ 0, 1, 2 ] );
	} );

	it( 'names every figure', () =>
	{
		expect( Object.values( NAMES ) ).toEqual( [ 'Rectangle', 'Circle', 'Line' ] );
	} );
} );
