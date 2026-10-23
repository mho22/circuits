import { describe, it, expect } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { flows } from '../src/flows.ts';
import { element } from './stubs/element.ts';


describe( 'flows', () =>
{
	const foo = element( { kind : Kind.Wire, from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, currents : [ NaN ] } );
	const bar = element( { kind : Kind.Resistor, from : { x : 2, y : 0 }, to : { x : 4, y : 0 }, currents : [ 0.005, -0.005 ] } );
	const baz = element( { kind : Kind.Ground, from : { x : 4, y : 0 }, to : { x : 4, y : 0 }, currents : [ 0.005 ] } );
	const branch = { from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, voltage : 5, current : 0.005 };

	it( 'runs along the branches and the leads of each part with their currents', () =>
	{
		expect( flows( [ foo, bar, baz ], [ branch ], false ) ).toEqual( [ branch, { from : { x : 2, y : 0 }, to : { x : 4, y : 0 }, current : 0.005 } ] );
	} );

	it( 'runs along every wire with one same current while the view is frozen', () =>
	{
		expect( flows( [ foo, bar, baz ], [], true ) ).toEqual( [ { from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, current : 1 } ] );
	} );
} );
