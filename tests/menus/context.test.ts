// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { open } from '../../src/menus/context.ts';


describe( 'context menu', () =>
{
	beforeEach( () => document.body.replaceChildren() );

	const LABELS = { foo : 'Foo', bar : 'Bar', baz : 'Baz' };

	it( 'lists the labels of the given actions in their order', () =>
	{
		const menu = open( document.body, { x : 10, y : 20 }, LABELS, [ 'baz', 'foo' ], () => {} );

		expect( menu.className ).toBe( 'context' );
		expect( Array.from( menu.children ).map( item => item.textContent ) ).toEqual( [ 'Baz', 'Foo' ] );
	} );

	it( 'runs the picked action and closes', () =>
	{
		const act = vi.fn();
		const menu = open( document.body, { x : 0, y : 0 }, LABELS, [ 'foo', 'bar' ], act );

		( menu.children[ 1 ] as HTMLButtonElement ).click();

		expect( act ).toHaveBeenCalledWith( 'bar' );
		expect( menu.isConnected ).toBe( false );
	} );

	it( 'closes on a press outside or on escape, and stays open on a press inside', () =>
	{
		const first = open( document.body, { x : 0, y : 0 }, LABELS, [ 'foo' ], () => {} );

		first.dispatchEvent( new PointerEvent( 'pointerdown', { bubbles : true } ) );

		expect( first.isConnected ).toBe( true );

		document.body.dispatchEvent( new PointerEvent( 'pointerdown', { bubbles : true } ) );

		expect( first.isConnected ).toBe( false );

		const second = open( document.body, { x : 0, y : 0 }, LABELS, [ 'foo' ], () => {} );

		document.dispatchEvent( new KeyboardEvent( 'keydown', { key : 'Escape' } ) );

		expect( second.isConnected ).toBe( false );
	} );

	it( 'closes when the view zooms', () =>
	{
		const menu = open( document.body, { x : 0, y : 0 }, LABELS, [ 'foo' ], () => {} );

		document.dispatchEvent( new Event( 'zoom' ) );

		expect( menu.isConnected ).toBe( false );
	} );
} );
