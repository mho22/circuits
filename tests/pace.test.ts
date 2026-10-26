import { describe, it, expect } from 'vitest';
import { pace } from '../src/pace.ts';


describe( 'pace', () =>
{
	it( 'gives nothing when the circuit runs at real speed or close to it', () =>
	{
		expect( [ pace( 1 ), pace( 0.95 ) ] ).toEqual( [ '', '' ] );
	} );

	it( 'gives the speed with one digit when the circuit runs slower', () =>
	{
		expect( [ pace( 0.94 ), pace( 0.43 ), pace( 0.00027 ) ] ).toEqual( [ ' × 0.9', ' × 0.4', ' × 0.0003' ] );
	} );
} );
