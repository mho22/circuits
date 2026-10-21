import { describe, it, expect } from 'vitest';
import { Figure } from '../../src/enums/figure.ts';
import { draw } from '../../src/renderers/drawing.ts';
import { record } from '../stubs/context.ts';


describe( 'drawing renderer', () =>
{
	const center = { x : 100, y : 50 };

	it( 'strokes every shape in one path around the center at the size of a unit', () =>
	{
		const { context, calls } = record();
		const shapes = [
			{ figure : Figure.Rectangle as const, from : { x : -1, y : -1 }, to : { x : 1, y : 2 } },
			{ figure : Figure.Circle as const, center : { x : 2, y : 0 }, radius : 1 },
		];

		draw( context, shapes, center, 0, 10 );

		expect( calls ).toEqual( [
			[ 'set', 'strokeStyle', '#9e9e9e' ],
			[ 'set', 'lineWidth', 2 ],
			[ 'beginPath' ],
			[ 'moveTo', 90, 40 ],
			[ 'lineTo', 110, 40 ],
			[ 'lineTo', 110, 70 ],
			[ 'lineTo', 90, 70 ],
			[ 'lineTo', 90, 40 ],
			[ 'moveTo', 130, 50 ],
			[ 'arc', 120, 50, 10, 0, 2 * Math.PI ],
			[ 'stroke' ],
		] );
	} );

	it( 'turns each point with the rotation', () =>
	{
		const { context, calls } = record();

		draw( context, [ { figure : Figure.Line, from : { x : 0, y : 0 }, to : { x : 2, y : 0 } } ], center, 1, 10 );

		expect( calls.filter( call => [ 'moveTo', 'lineTo' ].includes( call[ 0 ] as string ) ) ).toEqual( [ [ 'moveTo', 100, 50 ], [ 'lineTo', 100, 70 ] ] );
	} );
} );
