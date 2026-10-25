import { beforeAll, describe, it, expect } from 'vitest';
import { Circuit } from '../../bindings/web/pkg/web.js';
import { decode } from '../../src/decoders/element.ts';
import { Kind } from '../../src/enums/kind.ts';
import { SlideTool } from '../../src/tools/slide.ts';
import { load } from '../stubs/circuit.ts';


beforeAll( load );


describe( 'slide', () =>
{
	function slider() : Circuit
	{
		const circuit = new Circuit();

		circuit.add( Kind.Slider, 0, 0, 0, 2, 0 );

		return circuit;
	}

	it( 'sets the voltage of the slider under a point, then follows the point while it moves', () =>
	{
		const circuit = slider();
		const tool = new SlideTool( circuit );

		expect( tool.press( decode( circuit.elements() ), { x : 15, y : 0 } ) ).toBe( true );
		expect( decode( circuit.elements() )[ 0 ].value ).toBe( 5 );
		expect( tool.move( { x : 3, y : 40 } ) ).toBe( true );
		expect( decode( circuit.elements() )[ 0 ].value ).toBe( 3 );
		expect( tool.release() ).toBe( true );
		expect( tool.move( { x : -15, y : 0 } ) ).toBe( false );
		expect( decode( circuit.elements() )[ 0 ].value ).toBe( 3 );
	} );

	it( 'undoes a whole slide in one step', () =>
	{
		const circuit = slider();
		const tool = new SlideTool( circuit );

		tool.press( decode( circuit.elements() ), { x : 0, y : 0 } );
		tool.move( { x : 6, y : 0 } );
		tool.move( { x : 15, y : 0 } );
		tool.release();
		circuit.undo();

		expect( decode( circuit.elements() ).map( item => item.value ) ).toEqual( [ 0 ] );
	} );

	it( 'adds no undo step when the voltage stays the same', () =>
	{
		const circuit = slider();
		const tool = new SlideTool( circuit );

		tool.press( decode( circuit.elements() ), { x : -15, y : 0 } );
		tool.release();
		circuit.undo();

		expect( decode( circuit.elements() ) ).toEqual( [] );
	} );

	it( 'ignores a point off every slider and a slider that reads its wire', () =>
	{
		const circuit = slider();
		const tool = new SlideTool( circuit );
		const elements = decode( circuit.elements() );

		expect( tool.press( elements, { x : 30, y : 0 } ) ).toBe( false );
		expect( tool.press( elements.map( item => ( { ... item, listening : true } ) ), { x : 15, y : 0 } ) ).toBe( false );
		expect( tool.release() ).toBe( false );
		expect( decode( circuit.elements() )[ 0 ].value ).toBe( 0 );
	} );
} );
