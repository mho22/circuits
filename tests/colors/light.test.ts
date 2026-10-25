import { describe, it, expect } from 'vitest';
import { brightness, LIT, mix } from '../../src/colors/light.ts';
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

	it( 'mixes one color into itself', () =>
	{
		expect( mix( [ [ Color.Green, 1 ] ] ) ).toBe( LIT[ Color.Green ] );
	} );

	it( 'adds the colors channel by channel up to 255', () =>
	{
		expect( mix( [ [ Color.Red, 1 ], [ Color.Green, 1 ] ] ) ).toBe( '255, 255, 144' );
		expect( mix( [ [ Color.Red, 1 ], [ Color.Green, 1 ], [ Color.Blue, 1 ] ] ) ).toBe( '255, 255, 255' );
	} );

	it( 'scales each color by its brightness before the sum', () =>
	{
		expect( mix( [ [ Color.Red, 0.5 ], [ Color.Blue, 1 ] ] ) ).toBe( '176, 152, 255' );
	} );

	it( 'keeps each channel at the dark gray or above', () =>
	{
		expect( mix( [ [ Color.Red, 0.04 ] ] ) ).toBe( '26, 26, 26' );
		expect( mix( [ [ Color.Blue, 0.5 ] ] ) ).toBe( '26, 64, 128' );
	} );
} );
