import { readFileSync } from 'fs';
import { beforeAll, describe, it, expect } from 'vitest';
import { Circuit, initSync } from '../../bindings/web/pkg/web.js';
import { WireTool } from '../../src/tools/wire.ts';


beforeAll( () =>
{
	initSync( { module : readFileSync( new URL( '../../bindings/web/pkg/web_bg.wasm', import.meta.url ) ) } );
} );


describe( 'wire tool', () =>
{
	it( 'previews a wire snapped to the nearest straight angle', () =>
	{
		const tool = new WireTool( new Circuit() );

		tool.press( { x : 0, y : 0 } );
		tool.move( { x : 4, y : 5 } );

		expect( tool.preview() ).toEqual( [ 0, 0, 5, 5 ] );
	} );

	it( 'adds the previewed wire on release', () =>
	{
		const circuit = new Circuit();
		const tool = new WireTool( circuit );

		tool.press( { x : 0, y : 0 } );
		tool.move( { x : 6, y : 1 } );
		tool.release();

		expect( Array.from( circuit.wires() ) ).toEqual( [ 0, 0, 6, 0 ] );

		expect( tool.preview() ).toEqual( [] );
	} );

	it( 'adds nothing for a click without a drag', () =>
	{
		const circuit = new Circuit();
		const tool = new WireTool( circuit );

		tool.press( { x : 2, y : 2 } );
		tool.release();

		expect( circuit.wires() ).toHaveLength( 0 );
	} );

	it( 'ignores moves and releases without a press', () =>
	{
		const circuit = new Circuit();
		const tool = new WireTool( circuit );

		tool.move( { x : 3, y : 3 } );
		tool.release();

		expect( tool.preview() ).toEqual( [] );

		expect( circuit.wires() ).toHaveLength( 0 );
	} );

	it( 'erases the wire under a point', () =>
	{
		const circuit = new Circuit();
		const tool = new WireTool( circuit );

		circuit.addWire( 0, 0, 4, 0 );

		tool.erase( { x : 9, y : 9 } );

		expect( circuit.wires() ).toHaveLength( 4 );

		tool.erase( { x : 2, y : 0 } );

		expect( circuit.wires() ).toHaveLength( 0 );
	} );
} );
