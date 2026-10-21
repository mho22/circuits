import { describe, it, expect } from 'vitest';
import { Figure } from '../../src/enums/figure.ts';
import { CarryTool } from '../../src/tools/carry.ts';


describe( 'carry tool', () =>
{
	const shapes = [
		{ figure : Figure.Circle as const, center : { x : 0, y : 0 }, radius : 2 },
		{ figure : Figure.Line as const, from : { x : 0, y : 0 }, to : { x : 4, y : 0 } },
	];
	const pins = [ { point : { x : -2, y : 0 }, output : false, label : 'foo' }, { point : { x : 2, y : 0 }, output : true, label : 'bar' } ];

	it( 'leaves the shapes and the pins while it carries nothing', () =>
	{
		const tool = new CarryTool();

		tool.move( { x : 5, y : 5 } );

		expect( [ tool.shapes( shapes ), tool.pins( pins ) ] ).toEqual( [ shapes, pins ] );
	} );

	it( 'shifts the carried shape by the distance from its anchor in whole grid steps and leaves the pins', () =>
	{
		const tool = new CarryTool();

		tool.start( { pin : false, index : 1 }, { x : 1, y : 0 } );
		tool.move( { x : 4, y : 7 } );

		expect( tool.shapes( shapes ) ).toEqual( [ shapes[ 0 ], { ... shapes[ 1 ], from : { x : 4, y : 8 }, to : { x : 8, y : 8 } } ] );
		expect( tool.pins( pins ) ).toBe( pins );
	} );

	it( 'moves the carried pin to the nearest grid step, never onto another pin', () =>
	{
		const tool = new CarryTool();

		tool.start( { pin : true, index : 0 }, { x : -8, y : 0 } );
		tool.move( { x : -9, y : -7 } );

		expect( tool.pins( pins ).map( pin => pin.point ) ).toEqual( [ { x : -2, y : -2 }, { x : 2, y : 0 } ] );
		expect( tool.shapes( shapes ) ).toBe( shapes );

		tool.move( { x : 8, y : 0 } );

		expect( tool.pins( pins ) ).toBe( pins );
	} );

	it( 'carries nothing after a cancel', () =>
	{
		const tool = new CarryTool();

		tool.start( { pin : false, index : 0 }, { x : 0, y : 0 } );
		tool.move( { x : 4, y : 4 } );
		tool.cancel();

		expect( [ tool.target, tool.shapes( shapes ) ] ).toEqual( [ null, shapes ] );
	} );
} );
