import { describe, it, expect } from 'vitest';
import { decode } from '../../src/decoders/reading.ts';


describe( 'reading decoder', () =>
{
	it( 'reads the count, the voltages and the currents of each part', () =>
	{
		const data = new Float64Array( [ 2, 5, 0, 0.01, -0.01, 3, 0.7, 3, 0, 2e-5, 2e-3, -2.02e-3 ] );

		expect( decode( data ) ).toEqual( [
			{ potentials : [ 5, 0 ], currents : [ 0.01, -0.01 ] },
			{ potentials : [ 0.7, 3, 0 ], currents : [ 2e-5, 2e-3, -2.02e-3 ] },
		] );
	} );

	it( 'reads a part without terminal and no part from empty data', () =>
	{
		expect( decode( new Float64Array( [ 0 ] ) ) ).toEqual( [ { potentials : [], currents : [] } ] );
		expect( decode( new Float64Array() ) ).toEqual( [] );
	} );
} );
