import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { draw } from '../../src/renderers/switch.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'switch renderer', () =>
{
	function lines( value : number ) : unknown[][]
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Switch, value, from : { x : 0, y : 0 }, to : { x : 4, y : 0 }, potentials : [ 5, 0 ] } ), { voltage : 5, current : 0 } );

		return calls.filter( call => call[ 0 ] === 'moveTo' || call[ 0 ] === 'lineTo' );
	}

	it( 'draws a lead from each end to the body', () =>
	{
		expect( lines( 0 ).slice( 0, 4 ) ).toEqual( [ [ 'moveTo', 0, 0 ], [ 'lineTo', 25, 0 ], [ 'moveTo', 55, 0 ], [ 'lineTo', 80, 0 ] ] );
	} );

	it( 'draws a closed lever onto the contact', () =>
	{
		expect( lines( 1 ).slice( 4 ) ).toEqual( [ [ 'moveTo', 25, 0 ], [ 'lineTo', 55, 0 ] ] );
	} );

	it( 'draws an open lever turned away from the contact', () =>
	{
		const [ , [ , x, y ] ] = lines( 0 ).slice( 4 ) as [ unknown, [ string, number, number ] ];

		expect( x ).toBeCloseTo( 25 + 30 * Math.cos( Math.PI / 6 ) );
		expect( y ).toBeCloseTo( - 30 * Math.sin( Math.PI / 6 ) );
	} );

	it( 'colors the lever with the voltage of its pivot', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Switch, from : { x : 0, y : 0 }, to : { x : 4, y : 0 }, potentials : [ 5, 0 ] } ), { voltage : 5, current : 0 } );

		expect( calls.filter( call => call[ 1 ] === 'strokeStyle' ).map( call => call[ 2 ] ) ).toEqual( [ color( 5, 5 ), color( 0, 5 ), color( 5, 5 ) ] );
	} );
} );
