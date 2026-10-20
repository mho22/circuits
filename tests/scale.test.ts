import { describe, it, expect } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { measure } from '../src/scale.ts';
import { element } from './stubs/element.ts';


describe( 'scale', () =>
{
	it( 'finds the largest voltage and current in size', () =>
	{
		const elements = [ element( { kind : Kind.Battery, potentials : [ -9, 3 ], currents : [ -0.02, 0.02 ] } ), element( { potentials : [ 5, 5 ] } ) ];
		const branches = [ { from : { x : 0, y : 0 }, to : { x : 1, y : 0 }, voltage : 5, current : 0.01 } ];

		expect( measure( elements, branches ) ).toEqual( { voltage : 9, current : 0.02 } );
	} );

	it( 'finds the largest current among the terminals of a transistor', () =>
	{
		expect( measure( [ element( { kind : Kind.Npn, potentials : [ 0.7, 3, 0 ], currents : [ 2e-5, 2e-3, -2.02e-3 ] } ) ], [] ) ).toEqual( { voltage : 3, current : 2.02e-3 } );
	} );

	it( 'ignores the undefined current of a whole wire', () =>
	{
		expect( measure( [ element( { currents : [ NaN, NaN ] } ) ], [] ) ).toEqual( { voltage : 0, current : 0 } );
	} );

	it( 'measures nothing in an empty circuit', () =>
	{
		expect( measure( [], [] ) ).toEqual( { voltage : 0, current : 0 } );
	} );
} );
