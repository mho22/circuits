import { describe, it, expect } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { terminals } from '../src/terminals.ts';


describe( 'terminals', () =>
{
	it( 'gives a two-ended part its two ends', () =>
	{
		expect( terminals( Kind.Resistor, { x : 0, y : 0 }, { x : 4, y : 0 }, false ) ).toEqual( [ { x : 0, y : 0 }, { x : 4, y : 0 } ] );
		expect( terminals( Kind.Diode, { x : 0, y : 0 }, { x : 0, y : 2 }, true ) ).toEqual( [ { x : 0, y : 0 }, { x : 0, y : 2 } ] );
	} );

	it( 'gives an input or an output the end of its lead only', () =>
	{
		expect( terminals( Kind.Input, { x : 0, y : 0 }, { x : 2, y : 0 }, false ) ).toEqual( [ { x : 2, y : 0 }, { x : 2, y : 0 } ] );
		expect( terminals( Kind.Output, { x : 2, y : 0 }, { x : 0, y : 0 }, false ) ).toEqual( [ { x : 0, y : 0 }, { x : 0, y : 0 } ] );
	} );

	it( 'gives a toggle, a clock or a slider the end of its lead only', () =>
	{
		expect( terminals( Kind.Toggle, { x : 0, y : 0 }, { x : 0, y : 2 }, false ) ).toEqual( [ { x : 0, y : 2 }, { x : 0, y : 2 } ] );
		expect( terminals( Kind.Clock, { x : 0, y : 0 }, { x : 2, y : 0 }, false ) ).toEqual( [ { x : 2, y : 0 }, { x : 2, y : 0 } ] );
		expect( terminals( Kind.Slider, { x : 0, y : 0 }, { x : -2, y : 0 }, false ) ).toEqual( [ { x : -2, y : 0 }, { x : -2, y : 0 } ] );
	} );

	it( 'puts the collector left and the emitter right one step past the bar', () =>
	{
		expect( terminals( Kind.Npn, { x : 0, y : 0 }, { x : 2, y : 0 }, false ) ).toEqual( [ { x : 0, y : 0 }, { x : 3, y : -1 }, { x : 3, y : 1 } ] );
		expect( terminals( Kind.Pnp, { x : 0, y : 0 }, { x : 0, y : 2 }, false ) ).toEqual( [ { x : 0, y : 0 }, { x : 1, y : 3 }, { x : -1, y : 3 } ] );
	} );

	it( 'puts the drain left and the source right one step past the bar', () =>
	{
		expect( terminals( Kind.Nmos, { x : 0, y : 0 }, { x : 2, y : 0 }, false ) ).toEqual( [ { x : 0, y : 0 }, { x : 3, y : -1 }, { x : 3, y : 1 } ] );
		expect( terminals( Kind.Pmos, { x : 0, y : 0 }, { x : 0, y : 2 }, false ) ).toEqual( [ { x : 0, y : 0 }, { x : 1, y : 3 }, { x : -1, y : 3 } ] );
	} );

	it( 'swaps the two sides of a mirrored transistor', () =>
	{
		expect( terminals( Kind.Npn, { x : 0, y : 0 }, { x : 2, y : 0 }, true ) ).toEqual( [ { x : 0, y : 0 }, { x : 3, y : 1 }, { x : 3, y : -1 } ] );
	} );
} );
