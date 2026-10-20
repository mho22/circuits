import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/inductor.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'inductor renderer', () =>
{
	const inductor = element( { kind : Kind.Inductor, value : 1, from : { x : 0, y : 0 }, to : { x : 4, y : 0 }, potentials : [ 5, 0 ] } );

	it( 'colors the inductor from the voltage at one end to the voltage at the other', () =>
	{
		const { context, calls } = record();

		draw( context, inductor, { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'createLinearGradient', 0, 0, SPACING * 4, 0 ] );
		expect( calls ).toContainEqual( [ 'addColorStop', 0, color( 5, 5 ) ] );
		expect( calls ).toContainEqual( [ 'addColorStop', 1, color( 0, 5 ) ] );
	} );

	it( 'draws leads and four loops in the middle', () =>
	{
		const { context, calls } = record();

		draw( context, inductor, { voltage : 5, current : 0 } );

		expect( calls ).toContainEqual( [ 'lineTo', 20, 0 ] );
		expect( calls.filter( call => call[ 0 ] === 'arc' ) ).toEqual( [ 25, 35, 45, 55 ].map( x => [ 'arc', x, 0, 5, Math.PI, 0 ] ) );
		expect( calls ).toContainEqual( [ 'lineTo', SPACING * 4, 0 ] );
	} );

	it( 'shrinks the loops to fit a short inductor', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Inductor, value : 1, to : { x : 1, y : 0 } } ), { voltage : 0, current : 0 } );

		expect( calls.filter( call => call[ 0 ] === 'arc' ).map( call => call[ 3 ] ) ).toEqual( [ 2.5, 2.5, 2.5, 2.5 ] );
	} );

	it( 'labels the inductor with its value', () =>
	{
		const { context, calls } = record();

		draw( context, inductor, { voltage : 5, current : 0 } );

		expect( calls.some( call => call[ 0 ] === 'fillText' && call[ 1 ] === '1 H' ) ).toBe( true );
	} );
} );
