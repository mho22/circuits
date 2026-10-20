import { describe, it, expect } from 'vitest';
import { format, parse } from '../src/si.ts';


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

	it( 'parses a number with an optional prefix and unit', () =>
	{
		expect( parse( '4.7k' ) ).toBe( 4700 );
		expect( parse( '4.7 kΩ' ) ).toBe( 4700 );
		expect( parse( '2.2M' ) ).toBe( 2200000 );
		expect( parse( '220' ) ).toBe( 220 );
		expect( parse( ' 9 V ' ) ).toBe( 9 );
		expect( parse( '-1.5' ) ).toBe( -1.5 );
		expect( parse( '.5' ) ).toBe( 0.5 );
		expect( parse( '10m' ) ).toBe( 0.01 );
		expect( parse( '3u' ) ).toBe( 0.000003 );
		expect( parse( '3µ' ) ).toBe( 0.000003 );
		expect( parse( '1K' ) ).toBe( 1000 );
	} );

	it( 'rejects text that is not a value', () =>
	{
		expect( parse( '' ) ).toBeNaN();
		expect( parse( 'foo' ) ).toBeNaN();
		expect( parse( '1.2.3' ) ).toBeNaN();
		expect( parse( 'k5' ) ).toBeNaN();
	} );
} );
