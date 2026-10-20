import { describe, it, expect } from 'vitest';
import { encode } from '../../src/encoders/file.ts';
import { Kind } from '../../src/enums/kind.ts';
import { element } from '../stubs/element.ts';


describe( 'file encoder', () =>
{
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

	it( 'writes an empty drawing', () =>
	{
		expect( encode( [] ) ).toBe( '{\n\t"version" : 1,\n\t"parts" : []\n}\n' );
	} );
} );
