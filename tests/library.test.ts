// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { Library } from '../src/library.ts';


describe( 'library', () =>
{
	let parent : HTMLElement;

	beforeEach( () =>
	{
		document.body.replaceChildren();

		parent = document.createElement( 'nav' );

		document.body.append( parent );
	} );

	const group = () => parent.querySelector<HTMLElement>( '.group' )!;
	const pick = () => parent.querySelector<HTMLButtonElement>( '.pick' )!;
	const open = () => parent.querySelector<HTMLButtonElement>( '.open' )!;
	const menu = () => parent.querySelector<HTMLElement>( '.menu' )!;

	it( 'stays hidden until a symbol exists', () =>
	{
		const library = new Library( parent, () => {} );

		expect( group().hidden ).toBe( true );

		library.update( [ [ 1, '' ], [ 3, '' ] ] );

		expect( group().hidden ).toBe( false );
		expect( pick().textContent ).toBe( 'Symbol 1' );
		expect( Array.from( menu().children ).map( item => item.textContent ) ).toEqual( [ 'Symbol 1', 'Symbol 3' ] );
		expect( open().getAttribute( 'aria-label' ) ).toBe( 'Symbols' );

		library.update( [] );

		expect( group().hidden ).toBe( true );
	} );

	it( 'picks a symbol from its menu, closes the menu, and picks it again from the button', () =>
	{
		const picked = vi.fn();
		const library = new Library( parent, picked );

		library.update( [ [ 1, '' ], [ 3, '' ] ] );
		open().click();

		expect( menu().hidden ).toBe( false );

		( menu().children[ 1 ] as HTMLButtonElement ).click();

		expect( picked ).toHaveBeenLastCalledWith( 3 );
		expect( menu().hidden ).toBe( true );

		library.select( 3 );
		pick().click();

		expect( pick().textContent ).toBe( 'Symbol 3' );
		expect( pick().classList.contains( 'active' ) ).toBe( true );
		expect( picked ).toHaveBeenLastCalledWith( 3 );

		library.select( null );

		expect( pick().classList.contains( 'active' ) ).toBe( false );
	} );

	it( 'shows the first symbol again when the shown one is gone', () =>
	{
		const library = new Library( parent, () => {} );

		library.update( [ [ 1, '' ], [ 3, '' ] ] );
		library.select( 3 );
		library.update( [ [ 1, '' ] ] );

		expect( pick().textContent ).toBe( 'Symbol 1' );
	} );

	it( 'closes its menu on a press outside or on escape', () =>
	{
		const library = new Library( parent, () => {} );

		library.update( [ [ 1, '' ] ] );
		open().click();
		document.body.dispatchEvent( new PointerEvent( 'pointerdown', { bubbles : true } ) );

		expect( menu().hidden ).toBe( true );

		open().click();
		document.dispatchEvent( new KeyboardEvent( 'keydown', { key : 'Escape' } ) );

		expect( menu().hidden ).toBe( true );
	} );

	it( 'shows the name of a named symbol and renames it when its name changes', () =>
	{
		const library = new Library( parent, () => {} );

		library.update( [ [ 1, 'foo' ], [ 3, '' ] ] );

		expect( pick().textContent ).toBe( 'foo' );
		expect( Array.from( menu().children ).map( item => item.textContent ) ).toEqual( [ 'foo', 'Symbol 3' ] );

		library.update( [ [ 1, 'bar' ], [ 3, '' ] ] );

		expect( pick().textContent ).toBe( 'bar' );
	} );

	it( 'lists the categories with an arrow before the symbols, each in order', () =>
	{
		const library = new Library( parent, () => {} );

		library.update( [ [ 1, 'qux' ], [ 2, 'foo/baz' ], [ 3, 'bar/corge' ], [ 4, '' ], [ 5, 'foo/bar/garply' ] ] );

		expect( Array.from( menu().querySelectorAll( ':scope > .folder > button, :scope > button' ) ).map( item => item.textContent ) ).toEqual( [ 'bar▸', 'foo▸', 'qux', 'Symbol 4' ] );
		expect( Array.from( menu().querySelectorAll( '.folder > .menu' ) ).map( item => item.textContent ) ).toEqual( [ 'corge', 'bar▸garplybaz', 'garply' ] );
		expect( Array.from( menu().querySelectorAll<HTMLElement>( '.menu' ) ).every( item => item.hidden ) ).toBe( true );
		expect( pick().textContent ).toBe( 'qux' );
	} );

	it( 'sorts the symbols by their numbers', () =>
	{
		const library = new Library( parent, () => {} );

		library.update( [ [ 10, '' ], [ 9, '' ] ] );

		expect( Array.from( menu().children ).map( item => item.textContent ) ).toEqual( [ 'Symbol 9', 'Symbol 10' ] );
	} );

	it( 'picks a symbol inside a category and closes the categories when the menu opens again', () =>
	{
		const picked = vi.fn();
		const library = new Library( parent, picked );

		library.update( [ [ 1, 'foo/bar' ], [ 2, 'foo/qux' ] ] );
		open().click();

		const inside = menu().querySelector<HTMLElement>( '.folder > .menu' )!;
		const button = ( text : string ) => Array.from( menu().querySelectorAll( 'button' ) ).find( item => item.textContent === text )!;

		button( 'foo▸' ).dispatchEvent( new PointerEvent( 'pointerenter' ) );
		button( 'qux' ).click();

		expect( picked ).toHaveBeenLastCalledWith( 2 );
		expect( menu().hidden ).toBe( true );

		open().click();

		expect( inside.hidden ).toBe( true );
	} );

	it( 'opens a category on a hover, keeps its open categories when the hover comes back, and closes them on a hover of another item', () =>
	{
		const library = new Library( parent, () => {} );

		library.update( [ [ 1, 'foo/bar/baz' ], [ 2, 'foo/qux' ], [ 3, 'corge/garply' ], [ 4, 'waldo' ] ] );
		open().click();

		const inside = ( text : string ) => Array.from( menu().querySelectorAll<HTMLElement>( '.folder' ) ).find( item => item.firstElementChild!.textContent === `${ text }▸` )!.querySelector<HTMLElement>( ':scope > .menu' )!;
		const hover = ( text : string ) => Array.from( menu().querySelectorAll( 'button' ) ).find( item => item.textContent === text )!.dispatchEvent( new PointerEvent( 'pointerenter' ) );

		hover( 'foo▸' );
		hover( 'bar▸' );

		expect( [ inside( 'foo' ).hidden, inside( 'bar' ).hidden, inside( 'corge' ).hidden ] ).toEqual( [ false, false, true ] );

		hover( 'foo▸' );

		expect( [ inside( 'foo' ).hidden, inside( 'bar' ).hidden ] ).toEqual( [ false, false ] );

		hover( 'qux' );

		expect( [ inside( 'foo' ).hidden, inside( 'bar' ).hidden ] ).toEqual( [ false, true ] );

		hover( 'corge▸' );

		expect( [ inside( 'foo' ).hidden, inside( 'corge' ).hidden ] ).toEqual( [ true, false ] );

		hover( 'waldo' );

		expect( inside( 'corge' ).hidden ).toBe( true );
	} );
} );
