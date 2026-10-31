import { describe, it, expect } from 'vitest';
import { bus, color } from '../../src/colors/voltage.ts';
import { FROZEN } from '../../src/scale.ts';


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

	it( 'is blue on a frozen scale', () =>
	{
		expect( color( 5, FROZEN.voltage ) ).toBe( 'rgb(24, 48, 128)' );
	} );

	it( 'is gray without a scale or a finite voltage', () =>
	{
		expect( color( 5, 0 ) ).toBe( 'rgb(128, 128, 128)' );
		expect( color( NaN, 5 ) ).toBe( 'rgb(128, 128, 128)' );
	} );
} );


describe( 'bus color', () =>
{
	it( 'is gray when every lane is at zero volts', () =>
	{
		expect( bus( [ 0, 0, 0 ], 5 ) ).toBe( 'rgb(128, 128, 128)' );
	} );

	it( 'is green when a lane carries a positive or a negative voltage', () =>
	{
		expect( bus( [ 0, 5, 0 ], 5 ) ).toBe( 'rgb(0, 255, 0)' );
		expect( bus( [ 0, 0, -5 ], 5 ) ).toBe( 'rgb(0, 255, 0)' );
	} );

	it( 'is gray without a scale or a finite voltage', () =>
	{
		expect( bus( [ 5 ], 0 ) ).toBe( 'rgb(128, 128, 128)' );
		expect( bus( [ NaN ], 5 ) ).toBe( 'rgb(128, 128, 128)' );
	} );

	it( 'is blue on a frozen scale', () =>
	{
		expect( bus( [ 5 ], FROZEN.voltage ) ).toBe( 'rgb(24, 48, 128)' );
	} );
} );
