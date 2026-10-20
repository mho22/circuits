import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';


describe( 'voltage color', () =>
{
	it( 'is gray at zero volts', () =>
	{
		expect( color( 0, 5 ) ).toBe( 'rgb(128, 128, 128)' );
	} );

	it( 'is green at the largest positive voltage', () =>
	{
		expect( color( 5, 5 ) ).toBe( 'rgb(0, 255, 0)' );
	} );

	it( 'is red at the largest negative voltage', () =>
	{
		expect( color( -5, 5 ) ).toBe( 'rgb(255, 0, 0)' );
	} );

	it( 'blends between gray and the end colors', () =>
	{
		expect( color( 2.5, 5 ) ).toBe( 'rgb(64, 192, 64)' );
	} );

	it( 'clamps a voltage beyond the scale', () =>
	{
		expect( color( 50, 5 ) ).toBe( 'rgb(0, 255, 0)' );
	} );

	it( 'is gray without a scale or a finite voltage', () =>
	{
		expect( color( 5, 0 ) ).toBe( 'rgb(128, 128, 128)' );
		expect( color( NaN, 5 ) ).toBe( 'rgb(128, 128, 128)' );
	} );
} );
