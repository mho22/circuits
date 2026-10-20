import { describe, it, expect } from 'vitest';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/wire.ts';
import { record } from '../stubs/context.ts';


describe( 'wire renderer', () =>
{
	it( 'strokes each segment in screen coordinates', () =>
	{
		const { context, calls } = record();

		draw( context, [ 0, 0, 2, 0, 2, 0, 3, 1 ] );

		expect( calls.filter( call => call[ 0 ] === 'moveTo' || call[ 0 ] === 'lineTo' ) ).toEqual( [
			[ 'moveTo', 0, 0 ],
			[ 'lineTo', SPACING * 2, 0 ],
			[ 'moveTo', SPACING * 2, 0 ],
			[ 'lineTo', SPACING * 3, SPACING ],
		] );

		expect( calls.at( -2 ) ).toEqual( [ 'stroke' ] );
	} );

	it( 'draws with the given alpha and restores it', () =>
	{
		const { context, calls } = record();

		draw( context, [], 0.5 );

		expect( calls.at( 0 ) ).toEqual( [ 'set', 'globalAlpha', 0.5 ] );

		expect( calls.at( -1 ) ).toEqual( [ 'set', 'globalAlpha', 1 ] );
	} );
} );
