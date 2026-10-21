import { describe, it, expect } from 'vitest';
import { encode } from '../../src/encoders/file.ts';
import { Figure } from '../../src/enums/figure.ts';
import { Kind } from '../../src/enums/kind.ts';
import { element } from '../stubs/element.ts';


describe( 'file encoder', () =>
{
	const BLANK = { name : '', drawing : [], labels : [], layout : [], delay : 1e-9 };

	it( 'writes one part per line with its kind, its value and its ends', () =>
	{
		const elements = [
			element( { kind : Kind.Resistor, value : 1000, from : { x : 0, y : 0 }, to : { x : 4, y : 0 } } ),
			element( { kind : Kind.Capacitor, value : 1e-5, from : { x : 4, y : 0 }, to : { x : 4, y : 2 } } ),
			element( { kind : Kind.Switch, value : 1, from : { x : 4, y : 0 }, to : { x : 4, y : -2 } } ),
			element( { kind : Kind.Npn, from : { x : 0, y : 0 }, to : { x : 2, y : 0 } } ),
		];

		expect( encode( elements ) ).toBe( [
			'{',
			'\t"version" : 1,',
			'\t"parts" : [',
			'\t\t{ "kind" : "resistor", "ohms" : 1000, "from" : [ 0, 0 ], "to" : [ 4, 0 ] },',
			'\t\t{ "kind" : "capacitor", "farads" : 0.00001, "from" : [ 4, 0 ], "to" : [ 4, 2 ] },',
			'\t\t{ "kind" : "switch", "closed" : true, "from" : [ 4, 0 ], "to" : [ 4, -2 ] },',
			'\t\t{ "kind" : "npn", "from" : [ 0, 0 ], "to" : [ 2, 0 ] }',
			'\t]',
			'}',
			'',
		].join( '\n' ) );
	} );

	it( 'writes an open switch', () =>
	{
		expect( encode( [ element( { kind : Kind.Switch, value : 0 } ) ] ) ).toContain( '"closed" : false' );
	} );

	it( 'writes a mirrored transistor', () =>
	{
		expect( encode( [ element( { kind : Kind.Nmos, value : 1, mirrored : true } ) ] ) ).toContain( '{ "kind" : "nmos", "mirrored" : true, "from"' );
	} );

	it( 'writes a symbol with its layer and its rotation, and the parts of each layer after the drawing', () =>
	{
		const symbol = element( { kind : Kind.Symbol, value : 1, rotation : 1, from : { x : 5, y : 5 }, to : { x : 5, y : 5 } } );
		const pin = element( { kind : Kind.Output, from : { x : 1, y : 0 }, to : { x : 0, y : 0 } } );

		expect( encode( [ symbol ], [ { number : 1, definition : BLANK, parts : [ pin, element() ] }, { number : 2, definition : BLANK, parts : [] } ] ) ).toBe( [
			'{',
			'\t"version" : 1,',
			'\t"parts" : [',
			'\t\t{ "kind" : "symbol", "layer" : 1, "rotation" : 1, "from" : [ 5, 5 ], "to" : [ 5, 5 ] }',
			'\t],',
			'\t"symbols" : [',
			'\t\t{',
			'\t\t\t"layer" : 1,',
			'\t\t\t"parts" : [',
			'\t\t\t\t{ "kind" : "output", "from" : [ 1, 0 ], "to" : [ 0, 0 ] },',
			'\t\t\t\t{ "kind" : "wire", "from" : [ 0, 0 ], "to" : [ 2, 0 ] }',
			'\t\t\t]',
			'\t\t},',
			'\t\t{',
			'\t\t\t"layer" : 2,',
			'\t\t\t"parts" : []',
			'\t\t}',
			'\t]',
			'}',
			'',
		].join( '\n' ) );
	} );

	it( 'writes an empty drawing', () =>
	{
		expect( encode( [] ) ).toBe( '{\n\t"version" : 1,\n\t"parts" : []\n}\n' );
	} );

	it( 'writes the level of an input only when it is 1', () =>
	{
		const low = element( { kind : Kind.Input, from : { x : 0, y : 0 }, to : { x : 1, y : 0 } } );
		const high = element( { kind : Kind.Input, value : 1, from : { x : 0, y : 2 }, to : { x : 1, y : 2 } } );

		expect( encode( [ low, high ] ) ).toContain( '{ "kind" : "input", "from" : [ 0, 0 ], "to" : [ 1, 0 ] },\n\t\t{ "kind" : "input", "level" : 1, "from" : [ 0, 2 ], "to" : [ 1, 2 ] }' );
	} );

	it( 'writes the frequency of a clock', () =>
	{
		expect( encode( [ element( { kind : Kind.Clock, value : 1e7, from : { x : 0, y : 0 }, to : { x : 2, y : 0 } } ) ] ) ).toContain( '{ "kind" : "clock", "hertz" : 10000000, "from" : [ 0, 0 ], "to" : [ 2, 0 ] }' );
	} );

	it( 'writes the level of a toggle only when it is 1', () =>
	{
		const low = element( { kind : Kind.Toggle, from : { x : 0, y : 0 }, to : { x : 1, y : 0 } } );
		const high = element( { kind : Kind.Toggle, value : 1, from : { x : 2, y : 0 }, to : { x : 3, y : 0 } } );

		expect( encode( [ low, high ] ) ).toContain( '{ "kind" : "toggle", "from" : [ 0, 0 ], "to" : [ 1, 0 ] },\n\t\t{ "kind" : "toggle", "level" : 1, "from" : [ 2, 0 ], "to" : [ 3, 0 ] }' );
	} );

	it( 'writes the color of an led only when it is not red', () =>
	{
		const red = element( { kind : Kind.Led, from : { x : 0, y : 0 }, to : { x : 2, y : 0 } } );
		const blue = element( { kind : Kind.Led, value : 2, from : { x : 0, y : 2 }, to : { x : 2, y : 2 } } );

		expect( encode( [ red, blue ] ) ).toContain( '{ "kind" : "led", "from" : [ 0, 0 ], "to" : [ 2, 0 ] },\n\t\t{ "kind" : "led", "color" : "blue", "from" : [ 0, 2 ], "to" : [ 2, 2 ] }' );
	} );

	it( 'writes the delay of a layer only when it is not one nanosecond', () =>
	{
		expect( encode( [], [ { number : 1, definition : { ... BLANK, delay : 3e-9 }, parts : [] } ] ) ).toContain( '\t\t\t"layer" : 1,\n\t\t\t"delay" : 3e-9,\n' );
		expect( encode( [], [ { number : 1, definition : BLANK, parts : [] } ] ) ).not.toContain( 'delay' );
	} );

	it( 'writes the name, the drawing and the layout of a layer and the label of an input or an output', () =>
	{
		const pin = element( { kind : Kind.Input, label : 'b"ar', from : { x : 0, y : 0 }, to : { x : 1, y : 0 } } );
		const drawing = [
			{ figure : Figure.Circle as const, center : { x : 0, y : 1 }, radius : 2 },
			{ figure : Figure.Line as const, from : { x : -1, y : 0 }, to : { x : 1, y : 1 } },
			{ figure : Figure.Curve as const, from : { x : 0, y : -4 }, through : { x : 4, y : 0 }, to : { x : 0, y : 4 } },
		];
		const text = encode( [], [ { number : 1, definition : { name : 'foo', drawing, labels : [], layout : [ { x : -3, y : 0 }, { x : 2, y : 1 } ], delay : 1e-9 }, parts : [ pin ] } ] );
		const shapes = '{ "shape" : "circle", "center" : [ 0, 1 ], "radius" : 2 }, { "shape" : "line", "from" : [ -1, 0 ], "to" : [ 1, 1 ] }, { "shape" : "curve", "from" : [ 0, -4 ], "through" : [ 4, 0 ], "to" : [ 0, 4 ] }';

		expect( text ).toContain( `\t\t\t"layer" : 1,\n\t\t\t"name" : "foo",\n\t\t\t"drawing" : [ ${ shapes } ],\n\t\t\t"layout" : [ [ -3, 0 ], [ 2, 1 ] ],\n\t\t\t"parts" : [` );
		expect( text ).toContain( '{ "kind" : "input", "label" : "b\\"ar", "from" : [ 0, 0 ], "to" : [ 1, 0 ] }' );
	} );
} );
