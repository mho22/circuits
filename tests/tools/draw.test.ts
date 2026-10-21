import { describe, it, expect } from 'vitest';
import { Figure } from '../../src/enums/figure.ts';
import { DrawTool } from '../../src/tools/draw.ts';


describe( 'draw tool', () =>
{
	it( 'draws a circle on the half-step dot nearest the press with a radius of whole half steps and a dot on a click', () =>
	{
		const tool = new DrawTool();

		tool.press( { x : 5.4, y : 3.6 } );
		tool.release( { x : 12.2, y : 4.4 } );
		tool.press( { x : 4, y : 4 } );
		tool.release( { x : 4.6, y : 4.4 } );

		expect( tool.shapes ).toEqual( [ { figure : Figure.Circle, center : { x : 6, y : 4 }, radius : 6 }, { figure : Figure.Circle, center : { x : 4, y : 4 }, radius : 1 } ] );
	} );

	it( 'draws a line between the half-step dots nearest the press and the release, in any direction, and skips a click', () =>
	{
		const tool = new DrawTool();

		tool.figure = Figure.Line;
		tool.press( { x : 0.8, y : 0.6 } );
		tool.move( { x : 11.2, y : 5.4 } );

		expect( tool.preview() ).toEqual( { figure : Figure.Line, from : { x : 0, y : 0 }, to : { x : 12, y : 6 } } );

		tool.release( { x : 15.4, y : 1.4 } );
		tool.press( { x : 8, y : 8 } );
		tool.release( { x : 8.6, y : 7.6 } );

		expect( tool.shapes ).toEqual( [ { figure : Figure.Line, from : { x : 0, y : 0 }, to : { x : 16, y : 2 } } ] );
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
			{ figure : Figure.Line, from : { x : 2, y : -2 }, to : { x : 4, y : 2 } },
		];

		tool.undo();

		expect( tool.shapes ).toEqual( [ { figure : Figure.Circle, center : { x : 0, y : 0 }, radius : 2 } ] );
	} );
} );
