import { describe, it, expect } from 'vitest';
import { decode } from '../../src/decoders/file.ts';
import { encode } from '../../src/encoders/file.ts';
import { Kind } from '../../src/enums/kind.ts';
import { element } from '../stubs/element.ts';


describe( 'file decoder', () =>
{
	it( 'reads six numbers per part: kind, value and ends', () =>
	{
		const text = '{ "parts" : [ { "to" : [ 2, 2 ], "kind" : "battery", "from" : [ 0, 0 ], "volts" : 5 }, { "kind" : "switch", "closed" : true, "from" : [ 2, 2 ], "to" : [ 4, 2 ] } ], "version" : 1 }';

		expect( Array.from( decode( text ) ) ).toEqual( [ Kind.Battery, 5, 0, 0, 2, 2, Kind.Switch, 1, 2, 2, 4, 2 ] );
	} );

	it( 'reads back every kind of part', () =>
	{
		const kinds = [ Kind.Wire, Kind.Resistor, Kind.Battery, Kind.Ground, Kind.Capacitor, Kind.Inductor, Kind.Switch, Kind.Diode, Kind.Npn, Kind.Pnp, Kind.Nmos, Kind.Pmos ];
		const elements = kinds.map( ( kind, index ) => element( { kind, value : kind === Kind.Switch ? 0 : 4.7, from : { x : index, y : 0 }, to : { x : index, y : -2 } } ) );
		const values = ( kind : Kind ) => [ Kind.Resistor, Kind.Battery, Kind.Capacitor, Kind.Inductor ].includes( kind ) ? 4.7 : 0;

		expect( Array.from( decode( encode( elements ) ) ) ).toEqual( elements.flatMap( item => [ item.kind, values( item.kind ), item.from.x, item.from.y, item.to.x, item.to.y ] ) );
	} );

	it( 'rejects a file that is not a circuit', () =>
	{
		for( const text of [
			'',
			'foo',
			'null',
			'[]',
			'{ "parts" : [] }',
			'{ "version" : "1", "parts" : [] }',
			'{ "version" : 0, "parts" : [] }',
			'{ "version" : 1 }',
			'{ "version" : 1, "parts" : [ null ] }',
			'{ "version" : 1, "parts" : [ { "kind" : "foo", "from" : [ 0, 0 ], "to" : [ 2, 0 ] } ] }',
			'{ "version" : 1, "parts" : [ { "kind" : "resistor", "from" : [ 0, 0 ], "to" : [ 2, 0 ] } ] }',
			'{ "version" : 1, "parts" : [ { "kind" : "switch", "closed" : 1, "from" : [ 0, 0 ], "to" : [ 2, 0 ] } ] }',
			'{ "version" : 1, "parts" : [ { "kind" : "wire", "from" : [ 0 ], "to" : [ 2, 0 ] } ] }',
			'{ "version" : 1, "parts" : [ { "kind" : "wire", "from" : [ 0.5, 0 ], "to" : [ 2, 0 ] } ] }',
		] )
		{
			expect( () => decode( text ), text ).toThrow( 'This file is not a circuit.' );
		}
	} );

	it( 'rejects a file from a newer version', () =>
	{
		expect( () => decode( '{ "version" : 2, "parts" : [] }' ) ).toThrow( 'This circuit comes from a newer version of Circuits.' );
	} );
} );
