import { describe, it, expect } from 'vitest';
import { pace } from '../src/pace.ts';


describe( 'pace', () =>
{
	it( 'gives nothing when the circuit has no clock', () =>
	{
		expect( pace( undefined, 0.5 ) ).toBe( '' );
	} );

	it( 'gives nothing when the circuit runs at real speed or close to it', () =>
	{
		expect( [ pace( 64, 1 ), pace( 64, 0.95 ) ] ).toEqual( [ '', '' ] );
	} );

	it( 'gives the frequency the clock really reaches when the circuit runs slower', () =>
	{
		expect( [ pace( 64, 0.94 ), pace( 64, 0.6612 ), pace( 1e9, 0.00027 ) ] ).toEqual( [ '60 Hz', '42 Hz', '270 kHz' ] );
	} );

	it( 'gives the frequency without decimals', () =>
	{
		expect( [ pace( 64, 0.68 ), pace( 2e6, 0.7 ) ] ).toEqual( [ '44 Hz', '1 MHz' ] );
	} );
} );
