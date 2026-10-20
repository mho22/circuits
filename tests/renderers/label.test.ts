import { describe, it, expect } from 'vitest';
import { Kind } from '../../src/enums/kind.ts';
import { SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/label.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'label renderer', () =>
{
	it( 'writes the value above a horizontal part', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Resistor, value : 4700, from : { x : 0, y : 2 }, to : { x : 4, y : 2 } } ) );

		expect( calls ).toContainEqual( [ 'set', 'textAlign', 'center' ] );
		expect( calls ).toContainEqual( [ 'set', 'textBaseline', 'bottom' ] );
		expect( calls ).toContainEqual( [ 'fillText', '4.7 kΩ', SPACING * 2, SPACING * 2 - 14 ] );
	} );

	it( 'writes the value beside a vertical part', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Battery, value : 5, from : { x : 2, y : 4 }, to : { x : 2, y : 0 } } ) );

		expect( calls ).toContainEqual( [ 'set', 'textAlign', 'right' ] );
		expect( calls ).toContainEqual( [ 'set', 'textBaseline', 'middle' ] );
		expect( calls ).toContainEqual( [ 'fillText', '5 V', SPACING * 2 - 14, SPACING * 2 ] );
	} );

	it( 'writes the value above a part on one point', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Supply, value : 5, from : { x : 2, y : 2 }, to : { x : 2, y : 2 } } ) );

		expect( calls ).toContainEqual( [ 'set', 'textAlign', 'center' ] );
		expect( calls ).toContainEqual( [ 'set', 'textBaseline', 'bottom' ] );
		expect( calls ).toContainEqual( [ 'fillText', '5 V', SPACING * 2, SPACING * 2 - 14 ] );
	} );

	it( 'writes nothing for a part without a value', () =>
	{
		const { context, calls } = record();

		draw( context, element() );

		expect( calls ).toEqual( [] );
	} );
} );
