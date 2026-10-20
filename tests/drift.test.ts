import { describe, it, expect } from 'vitest';
import { Drift } from '../src/drift.ts';


describe( 'drift', () =>
{
	const foo = { from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, current : 1 };
	const bar = { from : { x : 0, y : 2 }, to : { x : 2, y : 2 }, current : -0.5 };

	it( 'moves each path in proportion to its current', () =>
	{
		const drift = new Drift();

		expect( drift.advance( [ foo, bar ], 1, 0.05 ) ).toEqual( [ 4, -2 ] );
	} );

	it( 'adds the new distance to the distance of the frame before', () =>
	{
		const drift = new Drift();

		drift.advance( [ foo ], 1, 0.05 );

		expect( drift.advance( [ { ...foo, current : 0.5 } ], 1, 0.05 ) ).toEqual( [ 6 ] );
	} );

	it( 'restarts a path that was gone for one frame', () =>
	{
		const drift = new Drift();

		drift.advance( [ foo ], 1, 0.05 );
		drift.advance( [ bar ], 1, 0.05 );

		expect( drift.advance( [ foo ], 1, 0.05 ) ).toEqual( [ 4 ] );
	} );

	it( 'keeps still without a current or a scale', () =>
	{
		const drift = new Drift();

		expect( drift.advance( [ { ...foo, current : NaN } ], 1, 0.05 ) ).toEqual( [ 0 ] );
		expect( drift.advance( [ foo ], 0, 0.05 ) ).toEqual( [ 0 ] );
	} );
} );
