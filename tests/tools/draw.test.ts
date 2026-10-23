import { describe, it, expect } from 'vitest';
import { Figure } from '../../src/enums/figure.ts';
import { DrawTool } from '../../src/tools/draw.ts';


describe( 'draw tool', () =>
{
	it( 'draws a circle on the half-step dot nearest the press with a radius of whole half steps and a dot on a click', () =>
	{
		const tool = new DrawTool();

		tool.figure = Figure.Circle;
		tool.press( { x : 5.4, y : 3.6 } );
		tool.release( { x : 12.2, y : 4.4 } );
		tool.press( { x : 4, y : 4 } );
		tool.release( { x : 4.6, y : 4.4 } );

		expect( tool.shapes ).toEqual( [ { figure : Figure.Circle, center : { x : 6, y : 4 }, radius : 6 }, { figure : Figure.Circle, center : { x : 4, y : 4 }, radius : 1 } ] );
	} );

	it( 'draws a line between the half-step dots nearest the press and the release, in any direction, and skips a click', () =>
	{
		const tool = new DrawTool();

		tool.press( { x : 0.8, y : 0.6 } );
		tool.move( { x : 11.2, y : 5.4 } );

		expect( tool.preview() ).toEqual( { figure : Figure.Line, from : { x : 0, y : 0 }, to : { x : 12, y : 6 } } );

		tool.release( { x : 15.4, y : 1.4 } );
		tool.press( { x : 8, y : 8 } );
		tool.release( { x : 8.6, y : 7.6 } );

		expect( tool.shapes ).toEqual( [ { figure : Figure.Line, from : { x : 0, y : 0 }, to : { x : 16, y : 2 } } ] );
		expect( tool.preview() ).toBeNull();
	} );

	it( 'draws a curve from a drag between its ends and a click on the dot it passes through, and waits while that dot is on the line', () =>
	{
		const tool = new DrawTool();

		tool.figure = Figure.Curve;
		tool.press( { x : 0, y : -4 } );
		tool.move( { x : 0, y : 4 } );

		expect( tool.preview() ).toEqual( { figure : Figure.Line, from : { x : 0, y : -4 }, to : { x : 0, y : 4 } } );

		tool.release( { x : 0, y : 4 } );
		tool.move( { x : 4.4, y : 0.6 } );

		expect( [ tool.shapes, tool.preview() ] ).toEqual( [ [], { figure : Figure.Curve, from : { x : 0, y : -4 }, through : { x : 4, y : 0 }, to : { x : 0, y : 4 } } ] );

		tool.press( { x : 0, y : 0 } );
		tool.release( { x : 0, y : 0 } );

		expect( tool.shapes ).toEqual( [] );

		tool.press( { x : 4, y : 0 } );
		tool.release( { x : 4, y : 0 } );

		expect( tool.shapes ).toEqual( [ { figure : Figure.Curve, from : { x : 0, y : -4 }, through : { x : 4, y : 0 }, to : { x : 0, y : 4 } } ] );
		expect( tool.preview() ).toBeNull();
	} );

	it( 'gives the half-step dot of the press for a text, draws no preview, and writes the trimmed text there', () =>
	{
		const tool = new DrawTool();

		tool.figure = Figure.Text;
		tool.press( { x : 5.4, y : 3.6 } );
		tool.move( { x : 12, y : 4 } );

		expect( tool.preview() ).toBeNull();
		expect( tool.release( { x : 12, y : 4 } ) ).toEqual( { x : 6, y : 4 } );
		expect( tool.release( { x : 12, y : 4 } ) ).toBeNull();

		tool.write( { x : 6, y : 4 }, '  ' );
		tool.write( { x : 6, y : 4 }, ' foo ' );

		expect( tool.shapes ).toEqual( [ { figure : Figure.Text, at : { x : 6, y : 4 }, content : 'foo' } ] );

		tool.undo();

		expect( tool.shapes ).toEqual( [] );
	} );

	it( 'cancels a curve waiting for the dot it passes through', () =>
	{
		const tool = new DrawTool();

		tool.figure = Figure.Curve;
		tool.press( { x : 0, y : 0 } );
		tool.release( { x : 4, y : 0 } );

		expect( [ tool.cancel(), tool.cancel() ] ).toEqual( [ true, false ] );

		tool.release( { x : 2, y : 2 } );

		expect( tool.shapes ).toEqual( [] );
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

		tool.load( [
			{ figure : Figure.Circle, center : { x : 0, y : 0 }, radius : 2 },
			{ figure : Figure.Line, from : { x : 2, y : -2 }, to : { x : 4, y : 2 } },
		], [] );

		tool.undo();

		expect( tool.shapes ).toEqual( [ { figure : Figure.Circle, center : { x : 0, y : 0 }, radius : 2 } ] );
	} );

	it( 'undoes and redoes a change of the shapes and the pins in one step', () =>
	{
		const tool = new DrawTool();
		const foo = { figure : Figure.Line as const, from : { x : 0, y : 0 }, to : { x : 4, y : 0 } };
		const bar = { figure : Figure.Line as const, from : { x : 0, y : 4 }, to : { x : 4, y : 4 } };

		const baz = [ { point : { x : -2, y : 0 }, output : false, label : 'baz' } ];
		const qux = [ { point : { x : -2, y : 2 }, output : false, label : 'baz' } ];

		tool.load( [ foo, bar ], baz );
		tool.change( [ bar ], qux );
		tool.undo();

		expect( [ tool.shapes, tool.pins ] ).toEqual( [ [ foo, bar ], baz ] );

		tool.redo();

		expect( [ tool.shapes, tool.pins ] ).toEqual( [ [ bar ], qux ] );
	} );

	it( 'redoes the undone shapes in order, and forgets them after a new shape or a load', () =>
	{
		const tool = new DrawTool();
		const foo = { figure : Figure.Line as const, from : { x : 0, y : 0 }, to : { x : 4, y : 0 } };
		const bar = { figure : Figure.Line as const, from : { x : 0, y : 4 }, to : { x : 4, y : 4 } };

		tool.load( [ foo, bar ], [] );
		tool.undo();
		tool.undo();
		tool.undo();
		tool.redo();

		expect( tool.shapes ).toEqual( [ foo ] );

		tool.redo();
		tool.redo();

		expect( tool.shapes ).toEqual( [ foo, bar ] );

		tool.undo();
		tool.press( { x : 0, y : 8 } );
		tool.release( { x : 4, y : 8 } );
		tool.redo();

		expect( tool.shapes ).toEqual( [ foo, { figure : Figure.Line, from : { x : 0, y : 8 }, to : { x : 4, y : 8 } } ] );

		tool.undo();
		tool.load( [ bar ], [] );
		tool.redo();

		expect( tool.shapes ).toEqual( [ bar ] );
	} );
} );
