import { describe, it, expect } from 'vitest';
import { brightness, CHANNELS, DARK, fill, LIT } from '../../src/colors/light.ts';
import { Color } from '../../src/enums/color.ts';


describe( 'light color', () =>
{
	it( 'gives no brightness below 0.1 mA forward', () =>
	{
		for( const current of [ -1e-3, 5e-5, NaN ] ) expect( brightness( current ) ).toBe( 0 );
	} );

	it( 'gives a brightness in proportion to the current up to 5 mA', () =>
	{
		expect( brightness( 0.0025 ) ).toBe( 0.5 );
		expect( brightness( 0.05 ) ).toBe( 1 );
	} );

	it( 'gives the channels of each color', () =>
	{
		expect( [ Color.Red, Color.Green, Color.Blue ].map( color => `rgb(${ CHANNELS[ color ].join( ', ' ) })` ) ).toEqual( [ Color.Red, Color.Green, Color.Blue ].map( color => `rgb(${ LIT[ color ] })` ) );
	} );

	it( 'fills with the sums of the channels', () =>
	{
		expect( fill( ... CHANNELS[ Color.Green ] as [ number, number, number ] ) ).toBe( `rgb(${ LIT[ Color.Green ] })` );
		expect( fill( 175.5, 152, 255 ) ).toBe( 'rgb(176, 152, 255)' );
	} );

	it( 'keeps each channel at 255 or below', () =>
	{
		expect( fill( 303, 268, 144 ) ).toBe( 'rgb(255, 255, 144)' );
	} );

	it( 'keeps each channel at the dark gray or above', () =>
	{
		expect( fill( 10.2, 1.92, 1.92 ) ).toBe( 'rgb(26, 26, 26)' );
		expect( fill( 24, 64, 127.5 ) ).toBe( 'rgb(26, 64, 128)' );
		expect( DARK ).toBe( '#1a1a1a' );
	} );
} );
