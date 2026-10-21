import { beforeAll, describe, it, expect } from 'vitest';
import { Circuit } from '../../bindings/web/pkg/web.js';
import { decode } from '../../src/decoders/element.ts';
import { Kind } from '../../src/enums/kind.ts';
import { MoveTool } from '../../src/tools/move.ts';
import { load } from '../stubs/circuit.ts';


beforeAll( load );


function resistor() : Circuit
{
	const circuit = new Circuit();

	circuit.add( Kind.Resistor, 1000, 0, 0, 2, 0 );

	return circuit;
}


describe( 'move tool', () =>
{
	it( 'previews the part shifted by the cursor from the anchor', () =>
	{
		const circuit = resistor();
		const tool = new MoveTool( circuit );
		const [ part ] = decode( circuit.elements() );

		tool.start( [ part ], { x : 1, y : 0 }, null );
		tool.move( { x : 4, y : 2 } );

		expect( tool.preview() ).toEqual( [ { ... part, from : { x : 3, y : 2 }, to : { x : 5, y : 2 }, potentials : [ 0, 0 ] } ] );
		expect( tool.zone() ).toBeNull();
	} );

	it( 'moves the part on drop and stops', () =>
	{
		const circuit = resistor();
		const tool = new MoveTool( circuit );

		tool.start( decode( circuit.elements() ), { x : 1, y : 0 }, null );
		tool.move( { x : 1, y : 3 } );
		tool.drop();

		expect( decode( circuit.elements() ).map( item => [ item.from, item.to ] ) ).toEqual( [ [ { x : 0, y : 3 }, { x : 2, y : 3 } ] ] );
		expect( tool.elements ).toEqual( [] );
		expect( tool.preview() ).toEqual( [] );
	} );

	it( 'previews a selection and its area shifted by the cursor from the anchor', () =>
	{
		const circuit = resistor();
		const tool = new MoveTool( circuit );

		circuit.add( Kind.Wire, 0, 2, 0, 2, 2 );
		tool.start( decode( circuit.elements() ), { x : 0, y : 0 }, { low : { x : 0, y : 0 }, high : { x : 2, y : 2 } } );
		tool.move( { x : 3, y : 1 } );

		expect( tool.preview().map( item => [ item.from, item.to ] ) ).toEqual( [ [ { x : 3, y : 1 }, { x : 5, y : 1 } ], [ { x : 5, y : 1 }, { x : 5, y : 3 } ] ] );
		expect( tool.zone() ).toEqual( { low : { x : 3, y : 1 }, high : { x : 5, y : 3 } } );
	} );

	it( 'moves every part inside the area in one undo step on drop', () =>
	{
		const circuit = resistor();
		const tool = new MoveTool( circuit );

		circuit.add( Kind.Wire, 0, 2, 0, 2, 2 );
		tool.start( decode( circuit.elements() ), { x : 0, y : 0 }, { low : { x : 0, y : 0 }, high : { x : 2, y : 2 } } );
		tool.move( { x : 0, y : 4 } );
		tool.drop();

		expect( decode( circuit.elements() ).map( item => [ item.from, item.to ] ) ).toEqual( [ [ { x : 0, y : 4 }, { x : 2, y : 4 } ], [ { x : 2, y : 4 }, { x : 2, y : 6 } ] ] );
		expect( tool.zone() ).toBeNull();

		circuit.undo();

		expect( decode( circuit.elements() ).map( item => item.from ) ).toEqual( [ { x : 0, y : 0 }, { x : 2, y : 0 } ] );
	} );

	it( 'leaves the part in place when the drop is at the anchor', () =>
	{
		const circuit = resistor();
		const tool = new MoveTool( circuit );

		tool.start( decode( circuit.elements() ), { x : 1, y : 0 }, null );
		tool.drop();

		expect( circuit.undo() ).toBe( 0 );
		expect( circuit.undo() ).toBeUndefined();
	} );

	it( 'leaves the part in place on cancel', () =>
	{
		const circuit = resistor();
		const tool = new MoveTool( circuit );

		tool.start( decode( circuit.elements() ), { x : 1, y : 0 }, { low : { x : 0, y : 0 }, high : { x : 2, y : 0 } } );
		tool.move( { x : 5, y : 5 } );
		tool.cancel();
		tool.drop();

		expect( decode( circuit.elements() )[ 0 ].from ).toEqual( { x : 0, y : 0 } );
	} );
} );
