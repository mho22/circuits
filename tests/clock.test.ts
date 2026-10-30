import { describe, it, expect } from 'vitest';
import { Clock } from '../src/clock.ts';


function grown( steps : number ) : Clock
{
	const clock = new Clock( 8 );

	for( let limit = 1; limit < steps; limit *= 2 ) clock.measure( clock.tick( 1, 2 * steps ), 0 );

	return clock;
}


describe( 'clock', () =>
{
	it( 'turns elapsed seconds into whole steps', () =>
	{
		expect( grown( 100 ).tick( 0.016, 1000 ) ).toBe( 16 );
	} );

	it( 'carries the fraction of a step to the next tick', () =>
	{
		const clock = grown( 10 );

		expect( [ clock.tick( 0.015, 100 ), clock.tick( 0.015, 100 ), clock.tick( 0.015, 100 ) ] ).toEqual( [ 1, 2, 1 ] );
	} );

	it( 'gives no step for no elapsed time', () =>
	{
		expect( new Clock( 8 ).tick( 0, 1000 ) ).toBe( 0 );
	} );

	it( 'follows a rate that changes between ticks', () =>
	{
		const clock = grown( 100 );

		expect( [ clock.tick( 0.5, 40 ), clock.tick( 0.5, 2 ) ] ).toEqual( [ 20, 1 ] );
	} );

	it( 'starts with one step and doubles its limit while the steps take no time', () =>
	{
		const clock = new Clock( 8 );

		expect( [ 0, 0, 0, 0 ].map( milliseconds => { const steps = clock.tick( 1, 1000 ); clock.measure( steps, milliseconds ); return steps; } ) ).toEqual( [ 1, 2, 4, 8 ] );
	} );

	it( 'limits the steps to those that fit in its budget at the measured cost', () =>
	{
		const clock = grown( 1000 );

		clock.measure( 1000, 40 );

		expect( clock.tick( 1, 1000 ) ).toBe( 200 );
	} );

	it( 'lowers its limit when fewer steps than the limit take longer than the budget', () =>
	{
		const clock = grown( 1000 );

		clock.measure( 10, 16 );

		expect( clock.tick( 1, 1000 ) ).toBe( 5 );
	} );

	it( 'starts from the cost of the last frame, not from one step', () =>
	{
		const clock = new Clock( 8 );

		clock.measure( clock.tick( 1, 1000 ), 0.01 );

		expect( clock.tick( 1, 1000 ) ).toBe( 800 );
	} );

	it( 'follows the cost of the last frame after one slow frame', () =>
	{
		const clock = grown( 10 );

		clock.measure( 3, 6 );
		clock.measure( clock.tick( 1, 1000 ), 0.5 );

		expect( clock.tick( 1, 1000 ) ).toBe( 64 );
	} );

	it( 'keeps its limit when fewer steps than the limit fit in the budget', () =>
	{
		const clock = grown( 100 );

		clock.measure( 10, 0 );

		expect( clock.tick( 1, 1000 ) ).toBe( 128 );
	} );

	it( 'drops the steps beyond the limit and keeps no fraction', () =>
	{
		const clock = new Clock( 8 );

		expect( [ clock.tick( 5.5, 1 ), clock.tick( 0.5, 1 ) ] ).toEqual( [ 1, 0 ] );
	} );

	it( 'follows the share of the owed steps that it runs, smoothly', () =>
	{
		const clock = new Clock( 8 );

		expect( clock.speed ).toBe( 1 );

		clock.tick( 1, 10 );

		expect( clock.speed ).toBeCloseTo( 0.91 );

		clock.tick( 0, 10 );

		expect( clock.speed ).toBeCloseTo( 0.91 );
	} );
} );
