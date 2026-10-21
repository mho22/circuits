import { describe, it, expect } from 'vitest';
import { Figure } from '../../src/enums/figure.ts';
import { DrawTool } from '../../src/tools/draw.ts';


describe( 'draw tool', () =>
{
	it( 'draws a rectangle between the grid dots nearest the press and the release and skips a flat one', () =>
	{
		const tool = new DrawTool();

		tool.press( { x : 1, y : 1 } );
		tool.move( { x : 7, y : 9 } );

		expect( tool.preview() ).toEqual( { figure : Figure.Rectangle, from : { x : 0, y : 0 }, to : { x : 8, y : 8 } } );

		tool.release( { x : 9, y : 5 } );
		tool.press( { x : 0, y : 0 } );
		tool.release( { x : 8, y : 1 } );

		expect( tool.shapes ).toEqual( [ { figure : Figure.Rectangle, from : { x : 0, y : 0 }, to : { x : 8, y : 4 } } ] );
		expect( tool.preview() ).toBeNull();
	} );

	it( 'draws a circle on the grid dot nearest the press with a radius of whole grid steps and skips a click', () =>
	{
		const tool = new DrawTool();

		tool.figure = Figure.Circle;
		tool.press( { x : 5, y : 4 } );
		tool.release( { x : 12, y : 9 } );
		tool.press( { x : 4, y : 4 } );
		tool.release( { x : 5, y : 5 } );

		expect( tool.shapes ).toEqual( [ { figure : Figure.Circle, center : { x : 4, y : 4 }, radius : 8 } ] );
	} );

	it( 'draws a line between the grid dots nearest the press and the release, in any direction, and skips a click', () =>
	{
		const tool = new DrawTool();

		tool.figure = Figure.Line;
		tool.press( { x : 1, y : 1 } );
		tool.move( { x : 11, y : 5 } );

		expect( tool.preview() ).toEqual( { figure : Figure.Line, from : { x : 0, y : 0 }, to : { x : 12, y : 4 } } );

		tool.release( { x : 15, y : 1 } );
		tool.press( { x : 8, y : 8 } );
		tool.release( { x : 9, y : 7 } );

		expect( tool.shapes ).toEqual( [ { figure : Figure.Line, from : { x : 0, y : 0 }, to : { x : 16, y : 0 } } ] );
		expect( tool.preview() ).toBeNull();
	} );

	it( 'cancels a shape in progress and tells whether one was in progress', () =>
	{
		const tool = new DrawTool();

		expect( tool.cancel() ).toBe( false );

		tool.press( { x : 0, y : 0 } );
		tool.move( { x : 2, y : 2 } );

		expect( [ tool.cancel(), tool.preview() ] ).toEqual( [ true, null ] );

		tool.release( { x : 2, y : 2 } );

		expect( tool.shapes ).toEqual( [] );
	} );

	it( 'undoes the last shape', () =>
	{
		const tool = new DrawTool();

		tool.shapes = [
			{ figure : Figure.Circle, center : { x : 0, y : 0 }, radius : 2 },
			{ figure : Figure.Rectangle, from : { x : 2, y : -2 }, to : { x : 4, y : 2 } },
		];

		tool.undo();

		expect( tool.shapes ).toEqual( [ { figure : Figure.Circle, center : { x : 0, y : 0 }, radius : 2 } ] );
	} );
} );
