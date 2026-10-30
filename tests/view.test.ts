import { describe, it, expect } from 'vitest';
import { SPACING } from '../src/grid.ts';
import { View } from '../src/view.ts';


describe( 'view', () =>
{
	function pan( view : View, x : number, y : number ) : void
	{
		view.press( { x : 0, y : 0 } );
		view.move( { x : - x, y : - y } );
		view.release();
	}

	it( 'starts at the origin', () =>
	{
		const view = new View();

		expect( [ view.x, view.y ] ).toEqual( [ 0, 0 ] );
		expect( view.toGrid( SPACING * 3, SPACING * 2 ) ).toEqual( { x : 3, y : 2 } );
	} );

	it( 'turns a screen position into the pixels of the drawing at the zoom of the view', () =>
	{
		const view = new View();

		pan( view, 10, 20 );

		expect( view.toPixels( 5, 5 ) ).toEqual( { x : 15, y : 25 } );

		view.zoom( 1, { x : 0, y : 0 } );

		expect( view.toPixels( 5, 5 ) ).toEqual( { x : 20, y : 30 } );
	} );

	it( 'drags the drawing with the pointer', () =>
	{
		const view = new View();

		view.press( { x : 100, y : 100 } );

		expect( view.move( { x : 60, y : 120 } ) ).toBe( true );
		expect( [ view.x, view.y ] ).toEqual( [ 40, -20 ] );

		view.move( { x : 50, y : 120 } );

		expect( [ view.x, view.y ] ).toEqual( [ 50, -20 ] );
	} );

	it( 'stops dragging on release', () =>
	{
		const view = new View();

		view.press( { x : 0, y : 0 } );
		view.release();

		expect( view.move( { x : 50, y : 50 } ) ).toBe( false );
		expect( [ view.x, view.y ] ).toEqual( [ 0, 0 ] );
	} );

	it( 'shows the origin at a point of the screen', () =>
	{
		const view = new View();

		pan( view, 30, -10 );
		view.center( { x : 100, y : 50 } );

		expect( [ view.x, view.y ] ).toEqual( [ -100, -50 ] );
		expect( view.toGrid( 100, 50 ) ).toEqual( { x : 0, y : 0 } );
	} );

	it( 'turns a screen position into a grid point of the moved drawing', () =>
	{
		const view = new View();

		pan( view, SPACING * 10, - SPACING * 4 );

		expect( view.toGrid( SPACING * 3, SPACING * 2 ) ).toEqual( { x : 13, y : -2 } );
	} );

	it( 'shows the middle of a layer under the cursor and comes back when the layer is left', () =>
	{
		const view = new View();

		pan( view, 30, 10 );
		view.enter( 2, { x : 4, y : 8.5 }, { x : 100, y : 50 } );

		expect( [ view.x, view.y ] ).toEqual( [ SPACING * 4 - 100, SPACING * 8.5 - 50 ] );
		expect( view.toGrid( 100, 50 ) ).toEqual( { x : 4, y : 9 } );

		view.follow( 2 );

		expect( [ view.x, view.y ] ).toEqual( [ SPACING * 4 - 100, SPACING * 8.5 - 50 ] );

		view.enter( 3, { x : 1, y : 1 }, { x : 0, y : 0 } );
		view.follow( 0 );

		expect( [ view.x, view.y ] ).toEqual( [ 30, 10 ] );
	} );

	it( 'shows a point of the grid at a point of the screen', () =>
	{
		const view = new View();

		pan( view, 30, 10 );
		view.show( { x : 4, y : 8.5 }, { x : 100, y : 50 } );

		expect( [ view.x, view.y ] ).toEqual( [ SPACING * 4 - 100, SPACING * 8.5 - 50 ] );
		expect( view.toGrid( 100, 50 ) ).toEqual( { x : 4, y : 9 } );
	} );

	it( 'shows a layer under the cursor again, not where it was left', () =>
	{
		const view = new View();

		view.enter( 2, { x : 4, y : 8 }, { x : 100, y : 50 } );
		pan( view, 30, 10 );
		view.follow( 0 );
		view.enter( 2, { x : 4, y : 8 }, { x : 200, y : 60 } );

		expect( [ view.x, view.y ] ).toEqual( [ SPACING * 4 - 200, SPACING * 8 - 60 ] );
	} );

	it( 'zooms out to one dot for two, four or eight grid steps around a point of the screen', () =>
	{
		const view = new View();
		const point = { x : SPACING * 6, y : SPACING * 3 };

		pan( view, SPACING * 2, SPACING );
		view.zoom( 1, point );

		expect( [ view.level, view.stride ] ).toEqual( [ 1, 2 ] );
		expect( view.toGrid( point.x, point.y ) ).toEqual( { x : 8, y : 4 } );
		expect( view.toGrid( point.x + SPACING, point.y ) ).toEqual( { x : 10, y : 4 } );

		view.zoom( 2, point );

		expect( view.toGrid( point.x, point.y ) ).toEqual( { x : 8, y : 4 } );
		expect( view.toGrid( point.x + SPACING, point.y ) ).toEqual( { x : 12, y : 4 } );

		view.zoom( 3, point );

		expect( view.stride ).toBe( 8 );
		expect( view.toGrid( point.x, point.y ) ).toEqual( { x : 8, y : 4 } );
		expect( view.toGrid( point.x + SPACING, point.y ) ).toEqual( { x : 16, y : 4 } );

		view.zoom( 0, point );

		expect( [ view.x, view.y ] ).toEqual( [ SPACING * 2, SPACING ] );
	} );

	it( 'keeps its zoom out of the four levels', () =>
	{
		const view = new View();

		expect( view.zoom( -1, { x : 100, y : 60 } ) ).toBe( false );
		expect( view.zoom( 4, { x : 100, y : 60 } ) ).toBe( false );
		expect( view.zoom( 0, { x : 100, y : 60 } ) ).toBe( false );
		expect( view.zoom( 1, { x : 100, y : 60 } ) ).toBe( true );
		expect( view.zoom( 0, { x : 100, y : 60 } ) ).toBe( true );

		expect( [ view.level, view.x, view.y ] ).toEqual( [ 0, 0, 0 ] );
	} );

	it( 'shows the origin at a point of the screen when zoomed out', () =>
	{
		const view = new View();

		view.zoom( 2, { x : 0, y : 0 } );
		view.center( { x : 100, y : 50 } );

		expect( view.toGrid( 100, 50 ) ).toEqual( { x : 0, y : 0 } );
		expect( view.toGrid( 100 + SPACING, 50 ) ).toEqual( { x : 4, y : 0 } );
	} );

	it( 'enters a layer at its zoom and comes back to the zoom it left', () =>
	{
		const view = new View();

		view.zoom( 1, { x : 0, y : 0 } );
		view.enter( 2, { x : 4, y : 8 }, { x : 100, y : 50 } );

		expect( view.level ).toBe( 1 );
		expect( view.toGrid( 100, 50 ) ).toEqual( { x : 4, y : 8 } );

		view.zoom( 2, { x : 100, y : 50 } );
		view.follow( 0 );

		expect( [ view.level, view.x, view.y ] ).toEqual( [ 1, 0, 0 ] );
	} );

	it( 'zooms one level for each swipe of the wheel around the cursor', () =>
	{
		const view = new View();
		const point = { x : SPACING * 6, y : SPACING * 3 };

		expect( view.spin( 30, 0, 1000, point ) ).toBe( false );
		expect( view.level ).toBe( 0 );
		expect( view.spin( 30, 0, 1100, point ) ).toBe( true );
		expect( view.spin( 300, 0, 1150, point ) ).toBe( false );

		expect( view.level ).toBe( 1 );
		expect( view.toGrid( point.x, point.y ) ).toEqual( { x : 6, y : 3 } );

		view.spin( -3, 1, 1400, point );

		expect( view.level ).toBe( 0 );

		view.spin( -3, 1, 1700, point );

		expect( [ view.level, view.x, view.y ] ).toEqual( [ 0, 0, 0 ] );
	} );

	it( 'ignores a light touch on the wheel', () =>
	{
		const view = new View();

		[ 4, -2, 8, 6 ].forEach( ( delta, index ) => view.spin( delta, 0, index * 300, { x : 0, y : 0 } ) );

		expect( view.level ).toBe( 0 );
	} );

	it( 'zooms again on a new swipe during the momentum of the last one', () =>
	{
		const view = new View();
		const spin = ( deltas : number[], start : number ) => deltas.forEach( ( delta, index ) => view.spin( delta, 0, start + index * 16, { x : 0, y : 0 } ) );

		spin( [ 20, 40, 60 ], 0 );

		expect( view.level ).toBe( 1 );

		spin( [ 80, 50, 30, 20 ], 48 );

		expect( view.level ).toBe( 1 );

		spin( [ 30, 40 ], 112 );

		expect( view.level ).toBe( 2 );
	} );

	it( 'turns a grid point into its position on the screen at each zoom', () =>
	{
		const view = new View();

		pan( view, 30, 10 );

		expect( view.toScreen( { x : 6, y : 3 } ) ).toEqual( { x : SPACING * 6 - 30, y : SPACING * 3 - 10 } );

		view.zoom( 1, view.toScreen( { x : 6, y : 3 } ) );

		expect( view.toScreen( { x : 6, y : 3 } ) ).toEqual( { x : SPACING * 6 - 30, y : SPACING * 3 - 10 } );
		expect( view.toScreen( { x : 8, y : 3 } ) ).toEqual( { x : SPACING * 7 - 30, y : SPACING * 3 - 10 } );
	} );
} );
