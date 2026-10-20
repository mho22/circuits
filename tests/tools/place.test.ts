import { beforeAll, describe, it, expect } from 'vitest';
import { Circuit } from '../../bindings/web/pkg/web.js';
import { decode } from '../../src/decoders/element.ts';
import { Kind } from '../../src/enums/kind.ts';
import { PlaceTool } from '../../src/tools/place.ts';
import { load } from '../stubs/circuit.ts';


beforeAll( load );


describe( 'place tool', () =>
{
	it( 'previews a wire snapped to the nearest straight angle', () =>
	{
		const tool = new PlaceTool( new Circuit() );

		tool.press( { x : 0, y : 0 } );
		tool.move( { x : 4, y : 5 } );

		expect( tool.preview() ).toEqual( { id : -1, kind : Kind.Wire, from : { x : 0, y : 0 }, to : { x : 5, y : 5 }, value : 0, potentials : [ 0, 0 ], currents : [ NaN, NaN ] } );
	} );

	it( 'previews a transistor with one voltage and one current per terminal', () =>
	{
		const tool = new PlaceTool( new Circuit() );

		tool.kind = Kind.Npn;
		tool.press( { x : 0, y : 0 } );
		tool.move( { x : 2, y : 0 } );

		expect( tool.preview() ).toEqual( { id : -1, kind : Kind.Npn, from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, value : 0, potentials : [ 0, 0, 0 ], currents : [ NaN, NaN, NaN ] } );
	} );

	it( 'adds the previewed part with its default value on release', () =>
	{
		const circuit = new Circuit();
		const tool = new PlaceTool( circuit );

		tool.kind = Kind.Resistor;
		tool.press( { x : 0, y : 0 } );
		tool.move( { x : 6, y : 1 } );
		tool.release();

		for( const kind of [ Kind.Battery, Kind.Capacitor, Kind.Inductor, Kind.Switch, Kind.Diode ] )
		{
			tool.kind = kind;
			tool.press( { x : 0, y : 0 } );
			tool.move( { x : 0, y : 3 } );
			tool.release();
		}

		expect( decode( circuit.elements() ).map( element => [ element.kind, element.value, element.to ] ) ).toEqual( [
			[ Kind.Resistor, 1000, { x : 6, y : 0 } ],
			[ Kind.Battery, 5, { x : 0, y : 3 } ],
			[ Kind.Capacitor, 1e-5, { x : 0, y : 3 } ],
			[ Kind.Inductor, 1, { x : 0, y : 3 } ],
			[ Kind.Switch, 0, { x : 0, y : 3 } ],
			[ Kind.Diode, 0, { x : 0, y : 3 } ],
		] );

		expect( tool.preview() ).toBeNull();
	} );

	it( 'places a ground with a click and ignores the drag', () =>
	{
		const circuit = new Circuit();
		const tool = new PlaceTool( circuit );

		tool.kind = Kind.Ground;
		tool.press( { x : 2, y : 2 } );
		tool.move( { x : 5, y : 2 } );

		expect( tool.preview()?.to ).toEqual( { x : 2, y : 2 } );

		tool.release();

		expect( decode( circuit.elements() ).map( element => [ element.kind, element.from, element.to ] ) ).toEqual( [ [ Kind.Ground, { x : 2, y : 2 }, { x : 2, y : 2 } ] ] );
	} );

	it( 'adds nothing for a click without a drag', () =>
	{
		const circuit = new Circuit();
		const tool = new PlaceTool( circuit );

		tool.press( { x : 2, y : 2 } );
		tool.release();

		expect( circuit.elements() ).toHaveLength( 0 );
	} );

	it( 'ignores moves and releases without a press', () =>
	{
		const circuit = new Circuit();
		const tool = new PlaceTool( circuit );

		tool.move( { x : 3, y : 3 } );
		tool.release();

		expect( tool.preview() ).toBeNull();
		expect( circuit.elements() ).toHaveLength( 0 );
	} );
} );
