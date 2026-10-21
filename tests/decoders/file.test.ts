import { describe, it, expect } from 'vitest';
import { decode } from '../../src/decoders/file.ts';
import { encode } from '../../src/encoders/file.ts';
import { Figure } from '../../src/enums/figure.ts';
import { Kind } from '../../src/enums/kind.ts';
import { element } from '../stubs/element.ts';


describe( 'file decoder', () =>
{
	it( 'reads eight numbers per part: layer, kind, value, rotation and ends', () =>
	{
		const text = '{ "parts" : [ { "to" : [ 2, 2 ], "kind" : "battery", "from" : [ 0, 0 ], "volts" : 5 }, { "kind" : "switch", "closed" : true, "from" : [ 2, 2 ], "to" : [ 4, 2 ] } ], "version" : 1 }';

		expect( Array.from( decode( text ).parts ) ).toEqual( [ 0, Kind.Battery, 5, 0, 0, 0, 2, 2, 0, Kind.Switch, 1, 0, 2, 2, 4, 2 ] );
	} );

	it( 'reads back every kind of part', () =>
	{
		const kinds = [ Kind.Wire, Kind.Resistor, Kind.Battery, Kind.Ground, Kind.Capacitor, Kind.Inductor, Kind.Switch, Kind.Diode, Kind.Npn, Kind.Pnp, Kind.Nmos, Kind.Pmos, Kind.Supply, Kind.Input, Kind.Output ];
		const elements = kinds.map( ( kind, index ) => element( { kind, value : [ Kind.Switch, Kind.Input, Kind.Output ].includes( kind ) ? 0 : 4.7, from : { x : index, y : 0 }, to : { x : index, y : -2 } } ) );
		const values = ( kind : Kind ) => [ Kind.Resistor, Kind.Battery, Kind.Capacitor, Kind.Inductor, Kind.Supply ].includes( kind ) ? 4.7 : 0;

		expect( Array.from( decode( encode( elements ) ).parts ) ).toEqual( elements.flatMap( item => [ 0, item.kind, values( item.kind ), 0, item.from.x, item.from.y, item.to.x, item.to.y ] ) );
	} );

	it( 'reads a mirrored transistor as the value 1', () =>
	{
		const text = '{ "version" : 1, "parts" : [ { "kind" : "pnp", "mirrored" : true, "from" : [ 0, 0 ], "to" : [ 2, 0 ] }, { "kind" : "resistor", "ohms" : 10, "mirrored" : true, "from" : [ 0, 2 ], "to" : [ 2, 2 ] } ] }';

		expect( Array.from( decode( text ).parts ) ).toEqual( [ 0, Kind.Pnp, 1, 0, 0, 0, 2, 0, 0, Kind.Resistor, 10, 0, 0, 2, 2, 2 ] );
	} );

	it( 'reads back a symbol and the parts of its layer', () =>
	{
		const symbol = element( { kind : Kind.Symbol, value : 3, rotation : 2, from : { x : 5, y : 5 }, to : { x : 5, y : 5 } } );
		const pin = element( { kind : Kind.Output, from : { x : 1, y : 0 }, to : { x : 0, y : 0 } } );
		const wire = element( { from : { x : 0, y : 0 }, to : { x : 2, y : 0 } } );

		expect( Array.from( decode( encode( [ symbol ], [ { number : 3, definition : { name : '', drawing : [], labels : [], layout : [], delay : 1e-9 }, parts : [ pin, wire ] } ] ) ).parts ) ).toEqual( [ 0, Kind.Symbol, 3, 2, 5, 5, 5, 5, 3, Kind.Output, 0, 0, 1, 0, 0, 0, 3, Kind.Wire, 0, 0, 0, 0, 2, 0 ] );
	} );

	it( 'reads a symbol without a rotation as not turned', () =>
	{
		const text = '{ "version" : 1, "parts" : [ { "kind" : "symbol", "layer" : 1, "from" : [ 0, 0 ], "to" : [ 0, 0 ] } ], "symbols" : [ { "layer" : 1, "parts" : [] } ] }';

		expect( Array.from( decode( text ).parts ) ).toEqual( [ 0, Kind.Symbol, 1, 0, 0, 0, 0, 0 ] );
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
			'{ "version" : 1, "parts" : [ { "kind" : "npn", "mirrored" : 1, "from" : [ 0, 0 ], "to" : [ 2, 0 ] } ] }',
			'{ "version" : 1, "parts" : [ { "kind" : "symbol", "layer" : 0, "from" : [ 0, 0 ], "to" : [ 0, 0 ] } ] }',
			'{ "version" : 1, "parts" : [ { "kind" : "symbol", "layer" : 1.5, "from" : [ 0, 0 ], "to" : [ 0, 0 ] } ] }',
			'{ "version" : 1, "parts" : [ { "kind" : "symbol", "layer" : 1, "rotation" : 4, "from" : [ 0, 0 ], "to" : [ 0, 0 ] } ] }',
			'{ "version" : 1, "parts" : [ { "kind" : "input", "level" : 2, "from" : [ 0, 0 ], "to" : [ 1, 0 ] } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "layout" : {}, "parts" : [] } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "layout" : [ [ 0 ] ], "parts" : [] } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : {} }',
			'{ "version" : 1, "parts" : [], "symbols" : [ null ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 0, "parts" : [] } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1 } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "name" : 1, "parts" : [] } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "drawing" : {}, "parts" : [] } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "drawing" : [ null ], "parts" : [] } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "drawing" : [ { "shape" : "foo" } ], "parts" : [] } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "drawing" : [ { "shape" : "rectangle", "from" : [ 0, 0 ] } ], "parts" : [] } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "drawing" : [ { "shape" : "circle", "center" : [ 0, 0 ], "radius" : 1.5 } ], "parts" : [] } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "drawing" : [ { "shape" : "line", "from" : [ 0, 0 ] } ], "parts" : [] } ] }',
			'{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "drawing" : [ { "shape" : "polyline", "points" : [ [ 0, 0 ], [ 1, 1 ] ] } ], "parts" : [] } ] }',
			'{ "version" : 1, "parts" : [ { "kind" : "pin", "label" : 1, "from" : [ 0, 0 ], "to" : [ 0, 0 ] } ] }',
			'{ "version" : 1, "parts" : [ { "kind" : "output", "label" : 1, "from" : [ 0, 0 ], "to" : [ 1, 0 ] } ] }',
		] )
		{
			expect( () => decode( text ), text ).toThrow( 'This file is not a circuit.' );
		}
	} );

	it( 'rejects a file from a newer version', () =>
	{
		expect( () => decode( '{ "version" : 2, "parts" : [] }' ) ).toThrow( 'This circuit comes from a newer version of Circuits.' );
	} );

	it( 'reads back the name, the drawing and the layout of each layer and the label of each input and output', () =>
	{
		const symbol = element( { kind : Kind.Symbol, value : 1, from : { x : 5, y : 5 }, to : { x : 5, y : 5 } } );
		const pin = element( { kind : Kind.Output, label : 'bar', from : { x : 1, y : 0 }, to : { x : 0, y : 0 } } );
		const drawing = [
			{ figure : Figure.Circle as const, center : { x : 0, y : 0 }, radius : 2 },
			{ figure : Figure.Line as const, from : { x : -1, y : 0 }, to : { x : 1, y : 1 } },
		];
		const contents = decode( encode( [ symbol ], [ { number : 1, definition : { name : 'foo', drawing, labels : [], layout : [ { x : 2, y : -1 } ], delay : 1e-9 }, parts : [ pin, element() ] }, { number : 2, definition : { name : 'baz', drawing : [], labels : [], layout : [], delay : 2e-9 }, parts : [] } ] ) );

		expect( Array.from( contents.parts ) ).toEqual( [ 0, Kind.Symbol, 1, 0, 5, 5, 5, 5, 1, Kind.Output, 0, 0, 1, 0, 0, 0, 1, Kind.Wire, 0, 0, 0, 0, 2, 0 ] );
		expect( contents.labels ).toEqual( [ '', 'bar', '' ] );
		expect( Array.from( contents.layers ) ).toEqual( [ 1, 2 ] );
		expect( contents.names ).toEqual( [ 'foo', 'baz' ] );
		expect( Array.from( contents.drawings ) ).toEqual( [ 9, 1, 0, 0, 2, 2, -1, 0, 1, 1, 0 ] );
		expect( Array.from( contents.layouts ) ).toEqual( [ 1, 2, -1, 0 ] );
		expect( Array.from( contents.delays ) ).toEqual( [ 1e-9, 2e-9 ] );
	} );

	it( 'reads a rectangle of an older file as its four sides', () =>
	{
		const contents = decode( '{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "drawing" : [ { "shape" : "rectangle", "from" : [ -4, -2 ], "to" : [ 4, 2 ] } ], "parts" : [] } ] }' );

		expect( Array.from( contents.drawings ) ).toEqual( [ 20, 2, -4, -2, 4, -2, 2, 4, -2, 4, 2, 2, 4, 2, -4, 2, 2, -4, 2, -4, -2 ] );
	} );

	it( 'reads the level of an input as its value', () =>
	{
		const input = element( { kind : Kind.Input, value : 1, from : { x : 0, y : 0 }, to : { x : 1, y : 0 } } );

		expect( Array.from( decode( encode( [ input ] ) ).parts ) ).toEqual( [ 0, Kind.Input, 1, 0, 0, 0, 1, 0 ] );
	} );

	it( 'reads the frequency of a clock as its value', () =>
	{
		const clock = element( { kind : Kind.Clock, value : 6e7, from : { x : 0, y : 0 }, to : { x : 2, y : 0 } } );

		expect( Array.from( decode( encode( [ clock ] ) ).parts ) ).toEqual( [ 0, Kind.Clock, 6e7, 0, 0, 0, 2, 0 ] );
	} );

	it( 'reads the level of a toggle as its value and an led', () =>
	{
		const toggle = element( { kind : Kind.Toggle, value : 1, from : { x : 0, y : 0 }, to : { x : 1, y : 0 } } );
		const led = element( { kind : Kind.Led, from : { x : 0, y : 2 }, to : { x : 2, y : 2 } } );

		expect( Array.from( decode( encode( [ toggle, led ] ) ).parts ) ).toEqual( [ 0, Kind.Toggle, 1, 0, 0, 0, 1, 0, 0, Kind.Led, 0, 0, 0, 2, 2, 2 ] );
	} );

	it( 'reads an input without a label and a level, and a layer without a name, a drawing and a layout', () =>
	{
		const contents = decode( '{ "version" : 1, "parts" : [], "symbols" : [ { "layer" : 1, "parts" : [ { "kind" : "input", "from" : [ 0, 0 ], "to" : [ 1, 0 ] } ] } ] }' );

		expect( [ contents.parts[ 2 ], contents.labels, Array.from( contents.layers ), contents.names, Array.from( contents.drawings ), Array.from( contents.layouts ) ] ).toEqual( [ 0, [ '' ], [ 1 ], [ '' ], [ 0 ], [ 0 ] ] );
	} );
} );
