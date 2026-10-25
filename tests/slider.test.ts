import { describe, it, expect } from 'vitest';
import { SPACING } from '../src/grid.ts';
import { covers, level, position, WIDTH } from '../src/slider.ts';


describe( 'slider', () =>
{
	const from = { x : 1, y : 1 };

	it( 'is one dot wider than a toggle', () =>
	{
		expect( WIDTH ).toBe( 21 );
	} );

	it( 'puts the knob at the left end at 0 V, in the middle at 2.5 V and at the right end at 5 V', () =>
	{
		expect( [ 0, 2.5, 5 ].map( volts => position( from, volts ) ) ).toEqual( [ SPACING - 15, SPACING, SPACING + 15 ] );
	} );

	it( 'keeps the knob inside the slider for a voltage out of its range', () =>
	{
		expect( [ -1, 7 ].map( volts => position( from, volts ) ) ).toEqual( [ SPACING - 15, SPACING + 15 ] );
	} );

	it( 'reads the voltage under a point in steps of 0.1 V, from 0 V at the left end to 5 V at the right end', () =>
	{
		expect( [ SPACING - 15, SPACING, SPACING + 3, SPACING + 4, SPACING + 15 ].map( x => level( from, x ) ) ).toEqual( [ 0, 2.5, 3, 3.2, 5 ] );
	} );

	it( 'reads 0 V past the left end and 5 V past the right end', () =>
	{
		expect( [ 0, 100 ].map( x => level( from, x ) ) ).toEqual( [ 0, 5 ] );
	} );

	it( 'covers the points inside its capsule only', () =>
	{
		expect( covers( from, { x : SPACING + 21, y : SPACING + 6 } ) ).toBe( true );
		expect( covers( from, { x : SPACING - 21, y : SPACING - 6 } ) ).toBe( true );
		expect( covers( from, { x : SPACING + 22, y : SPACING } ) ).toBe( false );
		expect( covers( from, { x : SPACING, y : SPACING + 7 } ) ).toBe( false );
	} );
} );
