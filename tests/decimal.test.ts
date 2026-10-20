import { describe, it, expect } from 'vitest';
import { clean, parse, write } from '../src/decimal.ts';


describe( 'decimal', () =>
{
	it( 'keeps the digits, a leading minus and the first separator', () =>
	{
		expect( clean( '4700' ) ).toBe( '4700' );
		expect( clean( '4,7k' ) ).toBe( '4,7' );
		expect( clean( '-1.5 V' ) ).toBe( '-1.5' );
		expect( clean( '1.2,3.4' ) ).toBe( '1.234' );
		expect( clean( '5-3' ) ).toBe( '53' );
		expect( clean( 'foo' ) ).toBe( '' );
	} );

	it( 'reads a comma or a dot as the decimal separator', () =>
	{
		expect( parse( '4,7' ) ).toBe( 4.7 );
		expect( parse( '4.7' ) ).toBe( 4.7 );
		expect( parse( '4700' ) ).toBe( 4700 );
		expect( parse( '-9' ) ).toBe( -9 );
		expect( parse( ',5' ) ).toBe( 0.5 );
		expect( parse( '5,' ) ).toBe( 5 );
	} );

	it( 'rejects text that is not a number', () =>
	{
		expect( parse( '' ) ).toBeNaN();
		expect( parse( '-' ) ).toBeNaN();
		expect( parse( ',' ) ).toBeNaN();
		expect( parse( '4,7k' ) ).toBeNaN();
		expect( parse( '1,2,3' ) ).toBeNaN();
	} );

	it( 'writes a number with the given separator and no exponent', () =>
	{
		expect( write( 4.7, ',' ) ).toBe( '4,7' );
		expect( write( 4.7, '.' ) ).toBe( '4.7' );
		expect( write( 4700, ',' ) ).toBe( '4700' );
		expect( write( 1e-5 / 1e-6, ',' ) ).toBe( '10' );
		expect( write( 1e-7, '.' ) ).toBe( '0.0000001' );
		expect( write( -9, ',' ) ).toBe( '-9' );
	} );
} );
