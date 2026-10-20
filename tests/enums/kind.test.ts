import { describe, it, expect } from 'vitest';
import { FIELDS, INPUTS, Kind, NAMES, POINTS, UNITS } from '../../src/enums/kind.ts';


describe( 'kind', () =>
{
	it( 'uses the codes of the engine binding', () =>
	{
		expect( [ Kind.Wire, Kind.Resistor, Kind.Battery, Kind.Ground, Kind.Capacitor, Kind.Inductor, Kind.Switch, Kind.Diode, Kind.Npn, Kind.Pnp, Kind.Nmos, Kind.Pmos, Kind.Supply ] ).toEqual( [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 ] );
	} );

	it( 'names every part', () =>
	{
		expect( Object.values( NAMES ) ).toEqual( [ 'Wire', 'Resistor', 'Battery', 'Ground', 'Capacitor', 'Inductor', 'Switch', 'Diode', 'NPN', 'PNP', 'NMOS', 'PMOS', 'VCC' ] );
	} );

	it( 'lists the parts that sit on one point', () =>
	{
		expect( POINTS ).toEqual( [ Kind.Ground, Kind.Supply ] );
	} );

	it( 'names the value of each part with a value in a file', () =>
	{
		expect( FIELDS ).toEqual( { [ Kind.Resistor ] : 'ohms', [ Kind.Battery ] : 'volts', [ Kind.Capacitor ] : 'farads', [ Kind.Inductor ] : 'henries', [ Kind.Supply ] : 'volts' } );
	} );

	it( 'gives a unit only to the parts with a value', () =>
	{
		expect( UNITS ).toEqual( { [ Kind.Resistor ] : 'Ω', [ Kind.Battery ] : 'V', [ Kind.Capacitor ] : 'F', [ Kind.Inductor ] : 'H', [ Kind.Supply ] : 'V' } );
	} );

	it( 'gives an input unit and its scale to the parts with a value', () =>
	{
		expect( INPUTS ).toEqual( { [ Kind.Resistor ] : [ 'Ω', 1 ], [ Kind.Battery ] : [ 'V', 1 ], [ Kind.Capacitor ] : [ 'µF', 1e-6 ], [ Kind.Inductor ] : [ 'mH', 1e-3 ], [ Kind.Supply ] : [ 'V', 1 ] } );
	} );
} );
