import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/ground.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'ground renderer', () =>
{
	it( 'draws a lead and three shrinking bars below the point', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Ground, from : { x : 1, y : 1 }, to : { x : 1, y : 1 } } ), { voltage : 5, current : 0 } );

		const x = SPACING;
		const y = SPACING;

		expect( calls ).toContainEqual( [ 'set', 'strokeStyle', color( 0, 5 ) ] );
		expect( calls.filter( call => call[ 0 ] === 'moveTo' || call[ 0 ] === 'lineTo' ) ).toEqual( [
			[ 'moveTo', x, y ],
			[ 'lineTo', x, y + 10 ],
			[ 'moveTo', x - 10, y + 10 ],
			[ 'lineTo', x + 10, y + 10 ],
			[ 'moveTo', x - 6, y + 14 ],
			[ 'lineTo', x + 6, y + 14 ],
			[ 'moveTo', x - 2, y + 18 ],
			[ 'lineTo', x + 2, y + 18 ],
		] );
	} );
} );
