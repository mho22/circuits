import { describe, it, expect } from 'vitest';
import { crowded } from '../src/crowd.ts';
import { Color } from '../src/enums/color.ts';


describe( 'crowd', () =>
{
	const point = { x : 1, y : 0 };
	const input = { point, output : false, label : 'foo' };
	const light = ( color : Color ) => ( { point, output : false, label : '', light : color } );

	it( 'leaves a point with a single pin free', () =>
	{
		expect( crowded( [ input ], point ) ).toBe( false );
		expect( crowded( [ light( Color.Red ) ], point ) ).toBe( false );
	} );

	it( 'lets up to three lights share a point', () =>
	{
		expect( crowded( [ light( Color.Red ), light( Color.Green ) ], point ) ).toBe( false );
		expect( crowded( [ light( Color.Red ), light( Color.Green ), light( Color.Blue ) ], point ) ).toBe( false );
		expect( crowded( [ light( Color.Red ), light( Color.Green ), light( Color.Blue ), light( Color.Red ) ], point ) ).toBe( true );
	} );

	it( 'lets inputs share a point with inputs and outputs with outputs into a stack of any size', () =>
	{
		expect( crowded( Array( 8 ).fill( input ), point ) ).toBe( false );
		expect( crowded( Array( 8 ).fill( { ... input, output : true } ), point ) ).toBe( false );
	} );

	it( 'never lets an input share a point with an output or a light', () =>
	{
		expect( crowded( [ input, light( Color.Red ) ], point ) ).toBe( true );
		expect( crowded( [ input, { ... input, output : true } ], point ) ).toBe( true );
		expect( crowded( [ input, input, { ... input, output : true } ], point ) ).toBe( true );
	} );

	it( 'counts only the pins on the point', () =>
	{
		expect( crowded( [ input, { ... input, point : { x : 2, y : 0 } } ], point ) ).toBe( false );
	} );
} );
