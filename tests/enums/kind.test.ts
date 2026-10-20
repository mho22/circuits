import { describe, it, expect } from 'vitest';
import { Kind, UNITS } from '../../src/enums/kind.ts';


describe( 'kind', () =>
{
	it( 'uses the codes of the engine binding', () =>
	{
		expect( [ Kind.Wire, Kind.Resistor, Kind.Battery, Kind.Ground ] ).toEqual( [ 0, 1, 2, 3 ] );
	} );

	it( 'gives a unit only to the parts with a value', () =>
	{
		expect( UNITS ).toEqual( { [ Kind.Resistor ] : 'Ω', [ Kind.Battery ] : 'V' } );
	} );
} );
