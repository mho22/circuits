import { describe, it, expect } from 'vitest';
import { SPACING } from '../src/grid.ts';
import { View } from '../src/view.ts';


describe( 'view', () =>
{
	it( 'starts at the origin', () =>
	{
		const view = new View();

		expect( [ view.x, view.y ] ).toEqual( [ 0, 0 ] );
		expect( view.toGrid( SPACING * 3, SPACING * 2 ) ).toEqual( { x : 3, y : 2 } );
	} );

	it( 'scrolls by pixels or by lines', () =>
	{
		const view = new View();

		view.scroll( 30, -10, 0 );

		expect( [ view.x, view.y ] ).toEqual( [ 30, -10 ] );

		view.scroll( 1, 2, 1 );

		expect( [ view.x, view.y ] ).toEqual( [ 30 + SPACING, -10 + SPACING * 2 ] );
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

	it( 'goes back to the origin', () =>
	{
		const view = new View();

		view.scroll( 30, -10, 0 );
		view.center();

		expect( [ view.x, view.y ] ).toEqual( [ 0, 0 ] );
	} );

	it( 'turns a screen position into a grid point of the moved drawing', () =>
	{
		const view = new View();

		view.scroll( SPACING * 10, - SPACING * 4, 0 );

		expect( view.toGrid( SPACING * 3, SPACING * 2 ) ).toEqual( { x : 13, y : -2 } );
	} );

	it( 'shows the middle of a layer under the cursor and comes back when the layer is left', () =>
	{
		const view = new View();

		view.scroll( 30, 10, 0 );
		view.enter( 2, { x : 4, y : 8.5 }, { x : 100, y : 50 } );

		expect( [ view.x, view.y ] ).toEqual( [ SPACING * 4 - 100, SPACING * 8.5 - 50 ] );
		expect( view.toGrid( 100, 50 ) ).toEqual( { x : 4, y : 9 } );

		view.follow( 2 );

		expect( [ view.x, view.y ] ).toEqual( [ SPACING * 4 - 100, SPACING * 8.5 - 50 ] );

		view.enter( 3, { x : 1, y : 1 }, { x : 0, y : 0 } );
		view.follow( 0 );

		expect( [ view.x, view.y ] ).toEqual( [ 30, 10 ] );
	} );

	it( 'shows a layer under the cursor again, not where it was left', () =>
	{
		const view = new View();

		view.enter( 2, { x : 4, y : 8 }, { x : 100, y : 50 } );
		view.scroll( 30, 10, 0 );
		view.follow( 0 );
		view.enter( 2, { x : 4, y : 8 }, { x : 200, y : 60 } );

		expect( [ view.x, view.y ] ).toEqual( [ SPACING * 4 - 200, SPACING * 8 - 60 ] );
	} );
} );
