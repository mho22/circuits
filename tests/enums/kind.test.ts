import { describe, it, expect } from 'vitest';
import { INPUTS, Kind, NAMES, UNITS } from '../../src/enums/kind.ts';


describe( 'kind', () =>
{
	it( 'uses the codes of the engine binding', () =>
	{
		expect( [ Kind.Wire, Kind.Resistor, Kind.Battery, Kind.Ground, Kind.Capacitor, Kind.Inductor, Kind.Switch ] ).toEqual( [ 0, 1, 2, 3, 4, 5, 6 ] );
	} );

	it( 'names every part', () =>
	{
		expect( Object.values( NAMES ) ).toEqual( [ 'Wire', 'Resistor', 'Battery', 'Ground', 'Capacitor', 'Inductor', 'Switch' ] );
	} );

	it( 'gives a unit only to the parts with a value', () =>
	{
		expect( UNITS ).toEqual( { [ Kind.Resistor ] : 'Ω', [ Kind.Battery ] : 'V', [ Kind.Capacitor ] : 'F', [ Kind.Inductor ] : 'H' } );
	} );

	it( 'gives an input unit and its scale to the parts with a value', () =>
	{
		expect( INPUTS ).toEqual( { [ Kind.Resistor ] : [ 'Ω', 1 ], [ Kind.Battery ] : [ 'V', 1 ], [ Kind.Capacitor ] : [ 'µF', 1e-6 ], [ Kind.Inductor ] : [ 'mH', 1e-3 ] } );
	} );
} );
