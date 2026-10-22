// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { Maker } from '../src/maker.ts';


const AREA = { low : { x : 0, y : 0 }, high : { x : 6, y : 4 } };
const SHAPE = { figure : Figure.Circle as const, center : { x : 0, y : 0 }, radius : 2 };
const LAYOUT = [ { x : -2, y : 0 }, { x : 3, y : 1 } ];


describe( 'maker', () =>
{
	let parent : HTMLElement;

	beforeEach( () =>
	{
		document.body.replaceChildren();

		parent = document.createElement( 'div' );

		document.body.append( parent );
	} );

	const bar = () => parent.querySelector<HTMLElement>( '.maker' )!;
	const button = ( text : string ) => Array.from( bar().querySelectorAll( 'button' ) ).find( item => item.textContent === text )!;

	const shown = () => Array.from( bar().querySelectorAll( 'button' ) ).filter( item => ! item.hidden ).map( item => item.textContent );

	it( 'shows its bar with a name, the picker, icon, done and cancel only while open', () =>
	{
		const picker = document.createElement( 'div' );
		const maker = new Maker( parent, picker, () => {}, () => {}, () => {}, () => {} );

		expect( bar().hidden ).toBe( true );

		maker.open( AREA );

		expect( bar().hidden ).toBe( false );
		expect( shown() ).toEqual( [ 'Icon', 'Done', 'Cancel' ] );
		expect( bar().children[ 1 ] ).toBe( picker );
		expect( document.activeElement ).toBe( bar().querySelector( 'input' ) );

		button( 'Cancel' ).click();

		expect( [ bar().hidden, maker.area ] ).toEqual( [ true, null ] );
	} );

	it( 'gives the area, the name, the drawing and the layout on done, then closes', () =>
	{
		const finish = vi.fn();
		const maker = new Maker( parent, document.createElement( 'div' ), finish, () => {}, () => {}, () => {} );

		maker.open( AREA );

		bar().querySelector( 'input' )!.value = ' qux ';
		maker.drawing = [ SHAPE ];
		maker.layout = LAYOUT;
		button( 'Done' ).click();

		expect( finish ).toHaveBeenCalledWith( AREA, 'qux', [ SHAPE ], LAYOUT );
		expect( [ bar().hidden, maker.area, maker.drawing, maker.layout ] ).toEqual( [ true, null, [], [] ] );
	} );

	it( 'asks for a drawing on icon and forgets the drawing and the layout on cancel and on a new open', () =>
	{
		const draw = vi.fn();
		const maker = new Maker( parent, document.createElement( 'div' ), () => {}, draw, () => {}, () => {} );

		maker.open( AREA );
		button( 'Icon' ).click();

		expect( draw ).toHaveBeenCalledOnce();

		maker.drawing = [ SHAPE ];
		maker.layout = LAYOUT;
		button( 'Cancel' ).click();

		expect( [ maker.drawing, maker.layout ] ).toEqual( [ [], [] ] );

		maker.open( AREA );
		maker.drawing = [ SHAPE ];
		maker.layout = LAYOUT;
		maker.open( AREA );

		expect( [ maker.drawing, maker.layout ] ).toEqual( [ [], [] ] );
	} );

	it( 'gives the count of its steps to discard on cancel, then closes, and does nothing when closed', () =>
	{
		const discard = vi.fn();
		const maker = new Maker( parent, document.createElement( 'div' ), () => {}, () => {}, discard, () => {} );

		maker.cancel();

		expect( discard ).not.toHaveBeenCalled();

		maker.open( AREA );
		maker.steps = 2;
		button( 'Cancel' ).click();

		expect( discard ).toHaveBeenCalledWith( 2, null );
		expect( [ bar().hidden, maker.area, maker.steps ] ).toEqual( [ true, null, 0 ] );

		maker.open( AREA );
		maker.steps = 1;
		maker.close();
		maker.open( AREA );

		expect( discard ).toHaveBeenCalledOnce();
		expect( maker.steps ).toBe( 0 );
	} );

	it( 'counts its steps back on an undo and forth on a redo, and forgets the undone steps after a new step or a close', () =>
	{
		const maker = new Maker( parent, document.createElement( 'div' ), () => {}, () => {}, () => {}, () => {} );

		maker.open( AREA );
		maker.record();
		maker.record();

		expect( [ maker.undo(), maker.undo(), maker.undo(), maker.steps ] ).toEqual( [ true, true, false, 0 ] );
		expect( [ maker.redo(), maker.steps ] ).toEqual( [ true, 1 ] );

		maker.record();

		expect( [ maker.redo(), maker.steps ] ).toEqual( [ false, 2 ] );

		maker.undo();
		maker.close();
		maker.open( AREA );

		expect( [ maker.redo(), maker.steps ] ).toEqual( [ false, 0 ] );
	} );

	it( 'edits a layer with its name, its drawing and its layout and shows delete', () =>
	{
		const finish = vi.fn();
		const maker = new Maker( parent, document.createElement( 'div' ), finish, () => {}, () => {}, () => {} );

		maker.edit( 3, { name : 'foo', drawing : [ SHAPE ], labels : [], layout : LAYOUT, delay : 2.5e-9 } );

		expect( [ bar().hidden, maker.area, maker.layer, maker.drawing, maker.layout ] ).toEqual( [ false, null, 3, [ SHAPE ], LAYOUT ] );
		expect( bar().querySelector( 'input' )!.value ).toBe( 'foo' );
		expect( document.activeElement ).not.toBe( bar().querySelector( 'input' ) );
		expect( shown() ).toEqual( [ 'Icon', 'Delete', 'Done', 'Cancel' ] );

		bar().querySelector( 'input' )!.value = 'bar';
		button( 'Done' ).click();

		expect( finish ).toHaveBeenCalledWith( 3, 'bar', [ SHAPE ], LAYOUT );
		expect( [ bar().hidden, maker.layer ] ).toEqual( [ true, null ] );
	} );

	it( 'gives the edited layer on delete and on cancel', () =>
	{
		const discard = vi.fn();
		const remove = vi.fn();
		const maker = new Maker( parent, document.createElement( 'div' ), () => {}, () => {}, discard, remove );

		maker.edit( 2, { name : '', drawing : [], labels : [], layout : [], delay : 1e-9 } );
		button( 'Delete' ).click();

		expect( remove ).toHaveBeenCalledWith( 2 );

		button( 'Cancel' ).click();

		expect( discard ).toHaveBeenCalledWith( 0, 2 );
		expect( [ bar().hidden, maker.layer ] ).toEqual( [ true, null ] );
	} );
} );
