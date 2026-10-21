import { describe, it, expect } from 'vitest';
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

	it( 'carries nothing after a cancel', () =>
	{
		const tool = new CarryTool();

		tool.start( 0, { x : 0, y : 0 } );
		tool.move( { x : 4, y : 4 } );
		tool.cancel();

		expect( [ tool.index, tool.pins( pins ) ] ).toEqual( [ null, pins ] );
	} );
} );
