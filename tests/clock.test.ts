import { describe, it, expect } from 'vitest';
import { Clock } from '../src/clock.ts';


describe( 'clock', () =>
{
	it( 'turns elapsed seconds into whole steps', () =>
	{
		expect( new Clock( 100 ).tick( 0.016, 1000 ) ).toBe( 16 );
	} );

	it( 'carries the fraction of a step to the next tick', () =>
	{
		const clock = new Clock( 10 );

		expect( [ clock.tick( 0.015, 100 ), clock.tick( 0.015, 100 ), clock.tick( 0.015, 100 ) ] ).toEqual( [ 1, 2, 1 ] );
	} );

	it( 'gives no step for no elapsed time', () =>
	{
		expect( new Clock( 100 ).tick( 0, 1000 ) ).toBe( 0 );
	} );

	it( 'follows a rate that changes between ticks', () =>
	{
		const clock = new Clock( 100 );

		expect( [ clock.tick( 0.5, 40 ), clock.tick( 0.5, 2 ) ] ).toEqual( [ 20, 1 ] );
	} );

	it( 'stops at the limit and drops the steps it skipped', () =>
	{
		const clock = new Clock( 100 );

		expect( clock.tick( 5, 1000 ) ).toBe( 100 );
		expect( clock.tick( 0.001, 1000 ) ).toBe( 1 );
	} );
} );
