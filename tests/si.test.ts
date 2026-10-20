import { describe, it, expect } from 'vitest';
import { format } from '../src/si.ts';


describe( 'si', () =>
{
	it( 'formats a value with the nearest prefix', () =>
	{
		expect( format( 4700, 'Ω' ) ).toBe( '4.7 kΩ' );
		expect( format( 0.005, 'A' ) ).toBe( '5 mA' );
		expect( format( 0.00000227, 'A' ) ).toBe( '2.27 µA' );
		expect( format( 2200000, 'Ω' ) ).toBe( '2.2 MΩ' );
		expect( format( 5, 'V' ) ).toBe( '5 V' );
		expect( format( -9, 'V' ) ).toBe( '-9 V' );
	} );

	it( 'keeps three significant digits', () =>
	{
		expect( format( 0.0072727, 'A' ) ).toBe( '7.27 mA' );
		expect( format( 123456, 'Ω' ) ).toBe( '123 kΩ' );
	} );

	it( 'moves to the next prefix when rounding reaches it', () =>
	{
		expect( format( 999.96, 'V' ) ).toBe( '1 kV' );
	} );

	it( 'uses the smallest prefix for tiny values', () =>
	{
		expect( format( 1e-15, 'A' ) ).toBe( '0.001 pA' );
	} );

	it( 'formats zero and values that are not finite', () =>
	{
		expect( format( 0, 'V' ) ).toBe( '0 V' );
		expect( format( NaN, 'A' ) ).toBe( '— A' );
		expect( format( Infinity, 'A' ) ).toBe( '— A' );
	} );
} );
