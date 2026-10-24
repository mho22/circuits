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
		expect( mix( [ Color.Green ] ) ).toBe( LIT[ Color.Green ] );
	} );

	it( 'adds the colors channel by channel up to 255', () =>
	{
		expect( mix( [ Color.Red, Color.Green ] ) ).toBe( '255, 255, 144' );
		expect( mix( [ Color.Red, Color.Green, Color.Blue ] ) ).toBe( '255, 255, 255' );
	} );
} );
