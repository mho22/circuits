import { describe, it, expect } from 'vitest';
import { Color } from '../../src/enums/color.ts';
import { CarryTool } from '../../src/tools/carry.ts';


describe( 'carry tool', () =>
{
	const pins = [ { point : { x : -2, y : 0 }, output : false, label : 'foo' }, { point : { x : 2, y : 0 }, output : true, label : 'bar' } ];

	it( 'leaves the pins while it carries nothing', () =>
	{
		const tool = new CarryTool();

		tool.move( { x : 5, y : 5 } );

		expect( tool.pins( pins ) ).toBe( pins );
	} );

	it( 'moves the carried pin to the nearest grid step, never onto another pin', () =>
	{
		const tool = new CarryTool();

		tool.start( 0, { x : -8, y : 0 } );
		tool.move( { x : -9, y : -7 } );

		expect( tool.pins( pins ).map( pin => pin.point ) ).toEqual( [ { x : -2, y : -2 }, { x : 2, y : 0 } ] );

		tool.move( { x : 8, y : 0 } );

		expect( tool.pins( pins ) ).toBe( pins );
	} );

	it( 'moves a light onto other lights, up to three on a point', () =>
	{
		const tool = new CarryTool();
		const at = ( x : number, light : Color ) => ( { point : { x, y : 0 }, output : false, label : '', light } );
		const full = [ at( 1, Color.Red ), at( 1, Color.Green ), at( 1, Color.Blue ), at( 0, Color.Red ) ];
		const room = [ at( 1, Color.Red ), at( 1, Color.Green ), at( 2, Color.Blue ), at( 0, Color.Red ) ];

		tool.start( 3, { x : 0, y : 0 } );
		tool.move( { x : 4, y : 0 } );

		expect( tool.pins( full ) ).toBe( full );
		expect( tool.pins( room ).map( pin => pin.point.x ) ).toEqual( [ 1, 1, 2, 1 ] );
	} );

	it( 'never moves a light onto an input or an output', () =>
	{
		const tool = new CarryTool();
		const light = { point : { x : 0, y : 0 }, output : false, label : '', light : Color.Red };

		tool.start( 2, { x : 0, y : 0 } );
		tool.move( { x : -8, y : 0 } );

		expect( tool.pins( [ ... pins, light ] ).map( pin => pin.point ) ).toEqual( [ { x : -2, y : 0 }, { x : 2, y : 0 }, { x : 0, y : 0 } ] );
	} );

	it( 'carries nothing after a cancel', () =>
	{
		const tool = new CarryTool();

		tool.start( 0, { x : 0, y : 0 } );
		tool.move( { x : 4, y : 4 } );
		tool.cancel();

		expect( [ tool.index, tool.pins( pins ) ] ).toEqual( [ null, pins ] );
	} );
} );
