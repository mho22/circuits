import { describe, it, expect } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { terminals } from '../src/terminals.ts';


describe( 'terminals', () =>
{
	it( 'gives a two-ended part its two ends', () =>
	{
		expect( terminals( Kind.Resistor, { x : 0, y : 0 }, { x : 4, y : 0 } ) ).toEqual( [ { x : 0, y : 0 }, { x : 4, y : 0 } ] );
		expect( terminals( Kind.Diode, { x : 0, y : 0 }, { x : 2, y : 2 } ) ).toEqual( [ { x : 0, y : 0 }, { x : 2, y : 2 } ] );
	} );

	it( 'puts the collector left and the emitter right one step past the bar', () =>
	{
		expect( terminals( Kind.Npn, { x : 0, y : 0 }, { x : 2, y : 0 } ) ).toEqual( [ { x : 0, y : 0 }, { x : 3, y : -1 }, { x : 3, y : 1 } ] );
		expect( terminals( Kind.Pnp, { x : 0, y : 0 }, { x : 0, y : 2 } ) ).toEqual( [ { x : 0, y : 0 }, { x : 1, y : 3 }, { x : -1, y : 3 } ] );
		expect( terminals( Kind.Npn, { x : 0, y : 0 }, { x : 2, y : 2 } ) ).toEqual( [ { x : 0, y : 0 }, { x : 4, y : 2 }, { x : 2, y : 4 } ] );
	} );
} );
