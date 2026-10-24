import { describe, it, expect } from 'vitest';
import { brightness } from '../../src/colors/light.ts';


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
} );
