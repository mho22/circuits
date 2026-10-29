import { beforeAll, describe, it, expect } from 'vitest';
import { Circuit } from '../../bindings/web/pkg/web.js';
import { decode } from '../../src/decoders/element.ts';
import { Kind } from '../../src/enums/kind.ts';
import { toggle } from '../../src/tools/toggle.ts';
import { load } from '../stubs/circuit.ts';


beforeAll( load );


describe( 'toggle', () =>
{
	it( 'closes an open switch under a point, then opens it', () =>
	{
		const circuit = new Circuit();

		circuit.add( Kind.Switch, 0, 0, 0, 4, 0 );

		expect( toggle( circuit, { x : 2, y : 0 } ) ).toBe( true );
		expect( decode( circuit.elements() )[ 0 ].value ).toBe( 1 );

		toggle( circuit, { x : 4, y : 0 } );

		expect( decode( circuit.elements() )[ 0 ].value ).toBe( 0 );
	} );

	it( 'finds the switch under a wire that ends on it', () =>
	{
		const circuit = new Circuit();

		circuit.add( Kind.Switch, 0, 0, 0, 4, 0 );
		circuit.add( Kind.Wire, 0, 4, 0, 4, 4 );

		toggle( circuit, { x : 4, y : 0 } );

		expect( decode( circuit.elements() )[ 0 ].value ).toBe( 1 );
	} );

	it( 'sets a free input to 1, then back to 0, from its square or its lead', () =>
	{
		const circuit = new Circuit();

		circuit.add( Kind.Input, 0, 0, 0, 2, 0 );

		expect( toggle( circuit, { x : 0, y : 0 } ) ).toBe( true );
		expect( decode( circuit.elements() )[ 0 ] ).toMatchObject( { value : 1, potentials : [ 5, 5 ] } );

		toggle( circuit, { x : 1, y : 0 } );

		expect( decode( circuit.elements() )[ 0 ].value ).toBe( 0 );
	} );

	it( 'switches a toggle to 0, then back to 1, from its slot or its lead', () =>
	{
		const circuit = new Circuit();

		circuit.add( Kind.Toggle, 1, 2, 2, 4, 2 );

		expect( toggle( circuit, { x : 2, y : 2 } ) ).toBe( true );
		expect( decode( circuit.elements() )[ 0 ] ).toMatchObject( { value : 0, potentials : [ 0, 0 ] } );

		toggle( circuit, { x : 3, y : 2 } );

		expect( decode( circuit.elements() )[ 0 ] ).toMatchObject( { value : 1, potentials : [ 0, 5 ] } );
	} );

	it( 'switches a clock off, then back on', () =>
	{
		const circuit = new Circuit();

		circuit.add( Kind.Clock, 1e6, 0, 0, 2, 0 );

		expect( toggle( circuit, { x : 1, y : 0 } ) ).toBe( true );
		expect( decode( circuit.elements() )[ 0 ].on ).toBe( false );

		toggle( circuit, { x : 1, y : 0 } );

		expect( decode( circuit.elements() )[ 0 ].on ).toBe( true );
	} );

	it( 'leaves a toggle on a wire that another part drives', () =>
	{
		const circuit = new Circuit();

		circuit.add( Kind.Toggle, 0, 2, 2, 4, 2 );
		circuit.add( Kind.Supply, 5, 4, 2, 4, 2 );

		expect( Array.from( circuit.listening() ) ).toEqual( [ 0 ] );
		expect( toggle( circuit, { x : 2, y : 2 } ) ).toBe( false );
		expect( decode( circuit.elements() )[ 0 ] ).toMatchObject( { value : 0, potentials : [ 5, 5 ] } );
	} );

	it( 'leaves an output', () =>
	{
		const circuit = new Circuit();

		circuit.add( Kind.Output, 0, 0, 0, 2, 0 );

		expect( toggle( circuit, { x : 0, y : 0 } ) ).toBe( false );
		expect( decode( circuit.elements() )[ 0 ].value ).toBe( 0 );
	} );

	it( 'switches an input inside a symbol even when a wire touches it outside', () =>
	{
		const circuit = new Circuit();
		const data = [ 0, Kind.Symbol, 1, 0, 5, 5, 5, 5, 0, Kind.Wire, 0, 0, 1, 5, 3, 5, 1, Kind.Input, 0, 0, -1, 0, 0, 0, 1, Kind.Output, 0, 0, 5, 0, 4, 0 ];

		circuit.load( new Float64Array( data ), [], new Uint32Array(), [], new Int32Array(), new Int32Array() );

		while( circuit.proceed() );
		circuit.enter( 0 );

		expect( toggle( circuit, { x : -1, y : 0 } ) ).toBe( true );
		expect( decode( circuit.elements() )[ 0 ] ).toMatchObject( { value : 1, potentials : [ 5, 5 ] } );
	} );

	it( 'changes nothing where no switch is', () =>
	{
		const circuit = new Circuit();

		circuit.add( Kind.Resistor, 1000, 0, 0, 4, 0 );

		expect( toggle( circuit, { x : 2, y : 0 } ) ).toBe( false );
		expect( decode( circuit.elements() )[ 0 ].value ).toBe( 1000 );
	} );
} );
