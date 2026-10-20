import { describe, it, expect } from 'vitest';
import { draw } from '../../src/renderers/line.ts';
import { record } from '../stubs/context.ts';


describe( 'line renderer', () =>
{
	it( 'strokes one line with its style and width', () =>
	{
		const { context, calls } = record();

		draw( context, '#ffffff', 3, 1, 2, 3, 4 );

		expect( calls ).toEqual( [
			[ 'set', 'strokeStyle', '#ffffff' ],
			[ 'set', 'lineWidth', 3 ],
			[ 'beginPath' ],
			[ 'moveTo', 1, 2 ],
			[ 'lineTo', 3, 4 ],
			[ 'stroke' ],
		] );
	} );
} );
