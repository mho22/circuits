import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Kind } from '../../src/enums/kind.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/supply.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'supply renderer', () =>
{
	it( 'draws a lead and a bar above the point, with the voltage over the bar', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Supply, value : 5, from : { x : 1, y : 1 }, to : { x : 1, y : 1 }, potentials : [ 0, 5 ] } ), { voltage : 5, current : 0 } );

		const x = SPACING;
		const y = SPACING;

		expect( calls ).toContainEqual( [ 'set', 'strokeStyle', color( 5, 5 ) ] );
		expect( calls.filter( call => call[ 0 ] === 'moveTo' || call[ 0 ] === 'lineTo' ) ).toEqual( [
			[ 'moveTo', x, y ],
			[ 'lineTo', x, y - 10 ],
			[ 'moveTo', x - 8, y - 10 ],
			[ 'lineTo', x + 8, y - 10 ],
		] );
		expect( calls ).toContainEqual( [ 'fillText', '5 V', x, y - 14 ] );
	} );
} );
