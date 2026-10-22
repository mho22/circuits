import { describe, it, expect } from 'vitest';
import { Figure } from '../../src/enums/figure.ts';
import { ShiftTool } from '../../src/tools/shift.ts';


describe( 'shift tool', () =>
{
	const shapes = [ { figure : Figure.Line as const, from : { x : 0, y : 0 }, to : { x : 4, y : 0 } }, { figure : Figure.Circle as const, center : { x : 8, y : 8 }, radius : 2 } ];
	const pins = [ { point : { x : -2, y : 0 }, output : false, label : 'foo' }, { point : { x : 2, y : 0 }, output : true, label : 'bar' } ];
	const area = { low : { x : -2, y : -2 }, high : { x : 6, y : 2 } };

	it( 'leaves the shapes and the pins while it holds nothing', () =>
	{
		const tool = new ShiftTool();

		tool.move( { x : 5, y : 5 } );

		expect( [ tool.shapes( shapes ), tool.pins( pins ), tool.zone() ] ).toEqual( [ shapes, pins, null ] );
	} );

	it( 'moves the held shapes and the area by the half steps of the cursor from the anchor', () =>
	{
		const tool = new ShiftTool();

		tool.start( { shapes : [ 0 ], pins : [] }, area, { x : 2, y : 0 } );
		tool.move( { x : 4, y : -2 } );

		expect( tool.shapes( shapes ) ).toEqual( [ { figure : Figure.Line, from : { x : 2, y : -2 }, to : { x : 6, y : -2 } }, shapes[ 1 ] ] );
		expect( tool.zone() ).toEqual( { low : { x : 0, y : -4 }, high : { x : 8, y : 0 } } );
	} );

	it( 'moves the held shapes and pins by whole grid steps when it holds a pin', () =>
	{
		const tool = new ShiftTool();

		tool.start( { shapes : [ 0 ], pins : [ 1 ] }, area, { x : 0, y : 0 } );
		tool.move( { x : 6, y : 2 } );

		expect( tool.shapes( shapes )[ 0 ] ).toEqual( { figure : Figure.Line, from : { x : 8, y : 4 }, to : { x : 12, y : 4 } } );
		expect( tool.pins( pins ).map( pin => pin.point ) ).toEqual( [ { x : -2, y : 0 }, { x : 4, y : 1 } ] );
	} );

	it( 'never moves a pin onto another pin', () =>
	{
		const tool = new ShiftTool();

		tool.start( { shapes : [], pins : [ 1 ] }, area, { x : 0, y : 0 } );
		tool.move( { x : -16, y : 0 } );

		expect( tool.pins( pins ) ).toBe( pins );
	} );

	it( 'holds nothing after a cancel', () =>
	{
		const tool = new ShiftTool();

		tool.start( { shapes : [ 0 ], pins : [ 0 ] }, area, { x : 0, y : 0 } );
		tool.move( { x : 4, y : 4 } );
		tool.cancel();

		expect( [ tool.grab, tool.shapes( shapes ), tool.pins( pins ), tool.zone() ] ).toEqual( [ null, shapes, pins, null ] );
	} );
} );
