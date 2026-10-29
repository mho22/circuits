import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/clock.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'clock renderer', () =>
{
	function drawn( potential : number, on = true ) : unknown[][]
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Clock, value : 1e6, on, from : { x : 1, y : 1 }, to : { x : 3, y : 1 }, potentials : [ 0, potential ] } ), { voltage : 5, current : 0 } );

		return calls;
	}

	it( 'draws a capsule in the color of its level', () =>
	{
		expect( drawn( 5 ) ).toContainEqual( [ 'set', 'strokeStyle', color( 5, 5 ) ] );
		expect( drawn( 0 ) ).toContainEqual( [ 'set', 'strokeStyle', color( 0, 5 ) ] );
		expect( drawn( 5 ).filter( call => call[ 0 ] === 'roundRect' ) ).toHaveLength( 1 );
	} );

	it( 'writes its frequency next to it', () =>
	{
		expect( drawn( 5 ).filter( call => call[ 0 ] === 'fillText' ).map( call => call[ 1 ] ) ).toEqual( [ '1 MHz' ] );
	} );

	it( 'draws one square pulse inside the capsule', () =>
	{
		const calls = drawn( 5 );
		const start = calls.findIndex( call => call[ 0 ] === 'moveTo' && call[ 1 ] === SPACING - 6 );

		expect( calls[ start ] ).toEqual( [ 'moveTo', SPACING - 6, SPACING + 3 ] );
		expect( calls.slice( start + 1, start + 6 ) ).toEqual( [
			[ 'lineTo', SPACING - 3, SPACING + 3 ],
			[ 'lineTo', SPACING - 3, SPACING - 3 ],
			[ 'lineTo', SPACING + 3, SPACING - 3 ],
			[ 'lineTo', SPACING + 3, SPACING + 3 ],
			[ 'lineTo', SPACING + 6, SPACING + 3 ],
		] );
	} );

	it( 'draws a flat line inside the capsule when it is off', () =>
	{
		const calls = drawn( 0, false );
		const start = calls.findIndex( call => call[ 0 ] === 'moveTo' && call[ 1 ] === SPACING - 6 );

		expect( calls.slice( start, start + 3 ) ).toEqual( [ [ 'moveTo', SPACING - 6, SPACING + 3 ], [ 'lineTo', SPACING + 6, SPACING + 3 ], [ 'stroke' ] ] );
	} );
} );
