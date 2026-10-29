import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { Hold } from '../src/hold.ts';


describe( 'hold', () =>
{
	beforeEach( () => vi.useFakeTimers() );

	afterEach( () => vi.useRealTimers() );

	it( 'runs an action after 500 ms', () =>
	{
		const action = vi.fn();

		new Hold().wait( action );

		vi.advanceTimersByTime( 499 );

		expect( action ).not.toHaveBeenCalled();

		vi.advanceTimersByTime( 1 );

		expect( action ).toHaveBeenCalledOnce();
	} );

	it( 'runs only the last of two actions that come within 500 ms', () =>
	{
		const hold = new Hold();
		const first = vi.fn();
		const second = vi.fn();

		hold.wait( first );
		vi.advanceTimersByTime( 200 );
		hold.wait( second );
		vi.advanceTimersByTime( 500 );

		expect( first ).not.toHaveBeenCalled();
		expect( second ).toHaveBeenCalledOnce();
	} );

	it( 'runs no action after a cancel', () =>
	{
		const hold = new Hold();
		const action = vi.fn();

		hold.wait( action );
		hold.cancel();
		vi.advanceTimersByTime( 500 );

		expect( action ).not.toHaveBeenCalled();
	} );

	it( 'accepts a cancel with no action that waits', () =>
	{
		expect( () => new Hold().cancel() ).not.toThrow();
	} );
} );
