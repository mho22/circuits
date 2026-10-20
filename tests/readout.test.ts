import { describe, it, expect } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { describe as read } from '../src/readout.ts';
import { element } from './stubs/element.ts';


describe( 'readout', () =>
{
	it( 'describes a wire with its voltage and the current of the hovered branch', () =>
	{
		const branch = { from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, voltage : 5, current : -0.01 };

		expect( read( element( { potentials : [ 5, 5 ] } ), branch ) ).toBe( 'Wire · 5 V · 10 mA' );
	} );

	it( 'describes a wire without a branch', () =>
	{
		expect( read( element(), undefined ) ).toBe( 'Wire · 0 V · — A' );
	} );

	it( 'describes a resistor with its value, the voltage across it and its current', () =>
	{
		expect( read( element( { kind : Kind.Resistor, value : 2200, potentials : [ 5, 0 ], current : 0.00227 } ), undefined ) ).toBe( 'Resistor 2.2 kΩ · 5 V across · 2.27 mA' );
	} );

	it( 'describes a battery', () =>
	{
		expect( read( element( { kind : Kind.Battery, value : 9, potentials : [ 0, 9 ], current : -0.5 } ), undefined ) ).toBe( 'Battery 9 V · 9 V across · 500 mA' );
	} );

	it( 'describes a ground', () =>
	{
		expect( read( element( { kind : Kind.Ground, current : -0.02 } ), undefined ) ).toBe( 'Ground · 0 V · 20 mA' );
	} );
} );
