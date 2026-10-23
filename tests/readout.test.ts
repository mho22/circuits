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
		expect( read( element( { kind : Kind.Resistor, value : 2200, potentials : [ 5, 0 ], currents : [ 0.00227, -0.00227 ] } ), undefined ) ).toBe( 'Resistor 2.2 kΩ · 5 V across · 2.27 mA' );
	} );

	it( 'describes a symbol with its layer and the voltage of each pin', () =>
	{
		expect( read( element( { kind : Kind.Symbol, value : 2, potentials : [ 5, 0, 2.5 ], currents : [ 0.001, -0.001, 0 ] } ), undefined ) ).toBe( 'Symbol 2 · 5 V · 0 V · 2.5 V' );
	} );

	it( 'describes an input with its bit and its voltage', () =>
	{
		expect( read( element( { kind : Kind.Input, potentials : [ 3.3, 3.3 ] } ), undefined ) ).toBe( 'Input · 1 · 3.3 V' );
	} );

	it( 'describes a toggle with its level, its voltage and its current', () =>
	{
		expect( read( element( { kind : Kind.Toggle, value : 1, potentials : [ 0, 5 ], currents : [ 0.005, -0.005 ] } ), undefined ) ).toBe( 'Toggle · 1 · 5 V · 5 mA' );
		expect( read( element( { kind : Kind.Toggle, value : 1, potentials : [ 0, 0 ], currents : [ NaN, NaN ], listening : true } ), undefined ) ).toBe( 'Toggle · 0 · 0 V' );
		expect( read( element( { kind : Kind.Clock, value : 1e7, potentials : [ 0, 5 ], currents : [ 0, 0 ] } ), undefined ) ).toBe( 'Clock 10 MHz · 1 · 5 V · 0 A' );
	} );

	it( 'describes an led with its forward drop and its current', () =>
	{
		expect( read( element( { kind : Kind.Led, potentials : [ 5, 3.1 ], currents : [ 0.01, -0.01 ] } ), undefined ) ).toBe( 'LED · 1.9 V across · 10 mA' );
	} );

	it( 'describes a named symbol with its name', () =>
	{
		expect( read( element( { kind : Kind.Symbol, value : 2, potentials : [ 5 ], currents : [ 0 ], definition : { name : 'foo', drawing : [], labels : [], layout : [], delay : 3e-9 } } ), undefined ) ).toBe( 'foo · 3 ns · 5 V' );
		expect( read( element( { kind : Kind.Symbol, value : 2, potentials : [ 5 ], currents : [ 0 ], definition : { name : 'foo', drawing : [], labels : [], layout : [], delay : NaN } } ), undefined ) ).toBe( 'foo · 5 V' );
	} );

	it( 'describes the pin of a symbol under a point with its label, its bit and its voltage', () =>
	{
		const outline = { low : { x : -2, y : -2 }, high : { x : 2, y : 2 }, pins : [ { x : -2, y : 0 }, { x : 2, y : 0 } ] };
		const definition = { name : 'foo', drawing : [], labels : [ 'bar', '' ], layout : [], delay : 3e-9 };
		const symbol = element( { kind : Kind.Symbol, value : 2, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline, definition, potentials : [ 5, 0 ], currents : [ 0, 0 ] } );

		expect( read( symbol, undefined, { x : 3, y : 5 } ) ).toBe( 'foo · bar · 1 · 5 V' );
		expect( read( symbol, undefined, { x : 7, y : 5 } ) ).toBe( 'foo · 0 · 0 V' );
		expect( read( symbol, undefined, { x : 5, y : 5 } ) ).toBe( 'foo · 3 ns · 5 V · 0 V' );
	} );

	it( 'puts the label of a symbol after its name and before the label of each of its pins', () =>
	{
		const outline = { low : { x : -2, y : -2 }, high : { x : 2, y : 2 }, pins : [ { x : -2, y : 0 }, { x : 2, y : 0 } ] };
		const definition = { name : 'foo', drawing : [], labels : [ 'bar', '' ], layout : [], delay : 3e-9 };
		const symbol = element( { kind : Kind.Symbol, value : 2, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline, definition, label : 'qux-', potentials : [ 5, 0 ], currents : [ 0, 0 ] } );

		expect( read( symbol, undefined, { x : 3, y : 5 } ) ).toBe( 'foo · qux-bar · 1 · 5 V' );
		expect( read( symbol, undefined, { x : 7, y : 5 } ) ).toBe( 'foo · 0 · 0 V' );
		expect( read( symbol, undefined, { x : 5, y : 5 } ) ).toBe( 'foo · qux- · 3 ns · 5 V · 0 V' );
	} );

	it( 'describes an output with its label', () =>
	{
		expect( read( element( { kind : Kind.Output, label : 'bar', potentials : [ 0, 0 ] } ), undefined ) ).toBe( 'Output bar · 0 · 0 V' );
	} );

	it( 'describes a battery', () =>
	{
		expect( read( element( { kind : Kind.Battery, value : 9, potentials : [ 0, 9 ], currents : [ -0.5, 0.5 ] } ), undefined ) ).toBe( 'Battery 9 V · 9 V across · 500 mA' );
	} );

	it( 'describes a capacitor and an inductor with their values', () =>
	{
		expect( read( element( { kind : Kind.Capacitor, value : 1e-5, potentials : [ 3, 0 ], currents : [ 0.002, -0.002 ] } ), undefined ) ).toBe( 'Capacitor 10 µF · 3 V across · 2 mA' );
		expect( read( element( { kind : Kind.Inductor, value : 1, potentials : [ 0, 1 ], currents : [ -0.004, 0.004 ] } ), undefined ) ).toBe( 'Inductor 1 H · 1 V across · 4 mA' );
	} );

	it( 'describes a switch with its state', () =>
	{
		expect( read( element( { kind : Kind.Switch, value : 0, potentials : [ 5, 0 ], currents : [ 0, 0 ] } ), undefined ) ).toBe( 'Switch open · 5 V across · 0 A' );
		expect( read( element( { kind : Kind.Switch, value : 1, potentials : [ 0, 0 ], currents : [ 0.005, -0.005 ] } ), undefined ) ).toBe( 'Switch closed · 0 V across · 5 mA' );
	} );

	it( 'describes a diode with its drop', () =>
	{
		expect( read( element( { kind : Kind.Diode, potentials : [ 5, 4.35 ], currents : [ 0.00435, -0.00435 ] } ), undefined ) ).toBe( 'Diode · 650 mV across · 4.35 mA' );
	} );

	it( 'describes a transistor with its junction voltages and its base and collector currents', () =>
	{
		expect( read( element( { kind : Kind.Npn, potentials : [ 0.7, 3, 0 ], currents : [ 2e-5, 2e-3, -2.02e-3 ] } ), undefined ) ).toBe( 'NPN · Vbe 700 mV · Vce 3 V · Ib 20 µA · Ic 2 mA' );
		expect( read( element( { kind : Kind.Pnp, potentials : [ -0.7, -3, 0 ], currents : [ -2e-5, -2e-3, 2.02e-3 ] } ), undefined ) ).toBe( 'PNP · Vbe 700 mV · Vce 3 V · Ib 20 µA · Ic 2 mA' );
	} );

	it( 'describes a mosfet with its gate and drain voltages and its drain current', () =>
	{
		expect( read( element( { kind : Kind.Nmos, potentials : [ 2, 2.5, 0 ], currents : [ 0, 2.5e-3, -2.5e-3 ] } ), undefined ) ).toBe( 'NMOS · Vgs 2 V · Vds 2.5 V · Id 2.5 mA' );
		expect( read( element( { kind : Kind.Pmos, potentials : [ 3, 2.5, 5 ], currents : [ 0, -2.5e-3, 2.5e-3 ] } ), undefined ) ).toBe( 'PMOS · Vgs 2 V · Vds 2.5 V · Id 2.5 mA' );
	} );

	it( 'describes a vcc with its voltage and its current', () =>
	{
		expect( read( element( { kind : Kind.Supply, value : 5, potentials : [ 0, 5 ], currents : [ 0.005, -0.005 ] } ), undefined ) ).toBe( 'VCC 5 V · 5 mA' );
	} );

	it( 'describes a ground', () =>
	{
		expect( read( element( { kind : Kind.Ground, currents : [ -0.02, 0.02 ] } ), undefined ) ).toBe( 'Ground · 0 V · 20 mA' );
	} );
} );
