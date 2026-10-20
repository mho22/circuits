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

		toggle( circuit, { x : 2, y : 0 } );

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

	it( 'changes nothing where no switch is', () =>
	{
		const circuit = new Circuit();

		circuit.add( Kind.Resistor, 1000, 0, 0, 4, 0 );

		toggle( circuit, { x : 2, y : 0 } );

		expect( decode( circuit.elements() )[ 0 ].value ).toBe( 1000 );
	} );
} );
