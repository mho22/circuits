import { describe, it, expect } from 'vitest';
import { Clock } from '../src/clock.ts';


describe( 'clock', () =>
{
	it( 'turns elapsed seconds into whole steps', () =>
	{
		expect( new Clock( 1000, 100 ).tick( 0.016 ) ).toBe( 16 );
	} );

	it( 'carries the fraction of a step to the next tick', () =>
	{
		const clock = new Clock( 100, 10 );

		expect( [ clock.tick( 0.015 ), clock.tick( 0.015 ), clock.tick( 0.015 ) ] ).toEqual( [ 1, 2, 1 ] );
	} );

	it( 'gives no step for no elapsed time', () =>
	{
		expect( new Clock( 1000, 100 ).tick( 0 ) ).toBe( 0 );
	} );

	it( 'stops at the limit and drops the steps it skipped', () =>
	{
		const clock = new Clock( 1000, 100 );

		expect( clock.tick( 5 ) ).toBe( 100 );
		expect( clock.tick( 0.001 ) ).toBe( 1 );
	} );
} );
