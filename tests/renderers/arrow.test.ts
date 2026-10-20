import { describe, it, expect } from 'vitest';
import { draw } from '../../src/renderers/arrow.ts';
import { record } from '../stubs/context.ts';


describe( 'arrow renderer', () =>
{
	it( 'fills a triangle that points along its direction', () =>
	{
		const { context, calls } = record();

		draw( context, 'foo', 10, 20, 1, 0 );

		expect( calls ).toEqual( [
			[ 'set', 'fillStyle', 'foo' ],
			[ 'beginPath' ],
			[ 'moveTo', 15, 20 ],
			[ 'lineTo', 5, 25 ],
			[ 'lineTo', 5, 15 ],
			[ 'closePath' ],
			[ 'fill' ],
		] );
	} );
} );
