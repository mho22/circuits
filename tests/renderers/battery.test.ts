import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { draw } from '../../src/renderers/battery.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'battery renderer', () =>
{
	const battery = element( { kind : Kind.Battery, value : 9, from : { x : 0, y : 0 }, to : { x : 4, y : 0 }, potentials : [ 0, 9 ] } );

	it( 'colors each lead with the voltage of its end', () =>
	{
		const { context, calls } = record();

		draw( context, battery, { voltage : 9, current : 0 } );

		const strokes = calls.filter( call => call[ 1 ] === 'strokeStyle' ).map( call => call[ 2 ] );

		expect( strokes.slice( 0, 2 ) ).toEqual( [ color( 0, 9 ), color( 9, 9 ) ] );
	} );

	it( 'draws the short thick plate on the negative side and the long plate on the positive side', () =>
	{
		const { context, calls } = record();

		draw( context, battery, { voltage : 9, current : 0 } );

		expect( calls ).toContainEqual( [ 'moveTo', 36, -6 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 36, 6 ] );
		expect( calls ).toContainEqual( [ 'moveTo', 44, -12 ] );
		expect( calls ).toContainEqual( [ 'lineTo', 44, 12 ] );
		expect( calls.filter( call => call[ 1 ] === 'lineWidth' ).map( call => call[ 2 ] ) ).toEqual( [ 2, 2, 4, 2 ] );
	} );

	it( 'labels the battery with its voltage', () =>
	{
		const { context, calls } = record();

		draw( context, battery, { voltage : 9, current : 0 } );

		expect( calls.some( call => call[ 0 ] === 'fillText' && call[ 1 ] === '9 V' ) ).toBe( true );
	} );
} );
