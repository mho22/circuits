import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { draw } from '../../src/renderers/transistor.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'transistor renderer', () =>
{
	const npn = element( { kind : Kind.Npn, from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, potentials : [ 0.7, 3, 0 ], currents : [ 2e-5, 2e-3, -2.02e-3 ] } );

	function drawn( kind : Kind ) : unknown[][]
	{
		const { context, calls } = record();

		draw( context, { ... npn, kind }, { voltage : 5, current : 0 } );

		return calls;
	}

	it( 'colors the base, the collector and the emitter leads with their voltages', () =>
	{
		const strokes = drawn( Kind.Npn ).filter( call => call[ 1 ] === 'strokeStyle' ).map( call => call[ 2 ] );

		expect( strokes.slice( 0, 3 ) ).toEqual( [ color( 0.7, 5 ), color( 3, 5 ), color( 0, 5 ) ] );
	} );

	it( 'draws the base up to the bar and the collector and the emitter out to their terminals', () =>
	{
		const calls = drawn( Kind.Npn );

		expect( calls ).toContainEqual( [ 'moveTo', 0, 0 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 40, 0 ] );
		expect( calls ).toContainEqual( [ 'moveTo', 40, -5 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 60, -20 ] );
		expect( calls ).toContainEqual( [ 'moveTo', 40, 5 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 60, 20 ] );
		expect( calls ).toContainEqual( [ 'moveTo', 40, -10 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 40, 10 ] );
	} );

	it( 'draws the collector on the right and the emitter on the left of a mirrored transistor', () =>
	{
		const { context, calls } = record();

		draw( context, { ... npn, mirrored : true }, { voltage : 5, current : 0 } );

		const lead = ( potential : number ) => { const index = calls.findIndex( call => call[ 1 ] === 'strokeStyle' && call[ 2 ] === color( potential, 5 ) ); return calls.slice( index + 3, index + 5 ); };

		expect( lead( 3 ) ).toEqual( [ [ 'moveTo', 40, 5 ], [ 'lineTo', 60, 20 ] ] );
		expect( lead( 0 ) ).toEqual( [ [ 'moveTo', 40, -5 ], [ 'lineTo', 60, -20 ] ] );
	} );

	it( 'points the arrow of an npn out of the emitter and the arrow of a pnp into it', () =>
	{
		const tip = ( kind : Kind ) => drawn( kind ).filter( call => call[ 0 ] === 'moveTo' ).at( -1 )!.slice( 1 ).map( value => Math.round( value as number * 10 ) / 10 );

		expect( tip( Kind.Npn ) ).toEqual( [ 54, 15.5 ] );
		expect( tip( Kind.Pnp ) ).toEqual( [ 46, 9.5 ] );
	} );
} );
