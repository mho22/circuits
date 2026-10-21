import { describe, it, expect } from 'vitest';
import { Color, NAMES, NEXT } from '../../src/enums/color.ts';


describe( 'color', () =>
{
	it( 'uses the values of the engine binding', () =>
	{
		expect( [ Color.Red, Color.Green, Color.Blue ] ).toEqual( [ 0, 1, 2 ] );
	} );

	it( 'names every color', () =>
	{
		expect( Object.values( NAMES ) ).toEqual( [ 'Red', 'Green', 'Blue' ] );
	} );

	it( 'goes from red to green to blue and back to red', () =>
	{
		expect( [ NEXT[ Color.Red ], NEXT[ Color.Green ], NEXT[ Color.Blue ] ] ).toEqual( [ Color.Green, Color.Blue, Color.Red ] );
	} );
} );
