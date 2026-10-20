import { beforeAll, describe, it, expect } from 'vitest';
import { Circuit } from '../../bindings/web/pkg/web.js';
import { decode } from '../../src/decoders/element.ts';
import { erase } from '../../src/tools/eraser.ts';
import { load } from '../stubs/circuit.ts';


beforeAll( load );


describe( 'eraser', () =>
{
	it( 'deletes the topmost part under a point', () =>
	{
		const circuit = new Circuit();

		circuit.add( 0, 0, 0, 0, 4, 0 );
		circuit.add( 3, 0, 2, 0, 2, 0 );

		erase( circuit, { x : 2, y : 0 } );

		expect( decode( circuit.elements() ).map( element => element.id ) ).toEqual( [ 0 ] );
	} );

	it( 'deletes nothing where no part is', () =>
	{
		const circuit = new Circuit();

		circuit.add( 0, 0, 0, 0, 4, 0 );

		erase( circuit, { x : 9, y : 9 } );

		expect( circuit.elements() ).toHaveLength( 10 );
	} );
} );
