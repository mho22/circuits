// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { Kind } from '../../src/enums/kind.ts';
import { open } from '../../src/menus/part.ts';


describe( 'part menu', () =>
{
	beforeEach( () => document.body.replaceChildren() );

	const labels = ( menu : HTMLElement ) => Array.from( menu.children ).map( item => item.textContent );

	it( 'opens at a position with every action for a part with two ends', () =>
	{
		const menu = open( document.body, { x : 10, y : 20 }, Kind.Resistor, () => {} );

		expect( menu.parentElement ).toBe( document.body );
		expect( [ menu.style.left, menu.style.top ] ).toEqual( [ '10px', '20px' ] );
		expect( labels( menu ) ).toEqual( [ 'Move', 'Rotate', 'Mirror horizontally', 'Mirror vertically', 'Delete' ] );
	} );

	it( 'offers only a move and a delete for a ground or a vcc', () =>
	{
		expect( labels( open( document.body, { x : 0, y : 0 }, Kind.Ground, () => {} ) ) ).toEqual( [ 'Move', 'Delete' ] );
		expect( labels( open( document.body, { x : 0, y : 0 }, Kind.Supply, () => {} ) ) ).toEqual( [ 'Move', 'Delete' ] );
	} );

	it( 'runs the picked action and closes', () =>
	{
		const act = vi.fn();
		const menu = open( document.body, { x : 0, y : 0 }, Kind.Npn, act );

		( menu.children[ 3 ] as HTMLButtonElement ).click();

		expect( act ).toHaveBeenCalledWith( 'vertical' );
		expect( menu.isConnected ).toBe( false );
	} );

	it( 'closes on a press outside or on escape, and stays open on a press inside', () =>
	{
		const first = open( document.body, { x : 0, y : 0 }, Kind.Diode, () => {} );

		first.dispatchEvent( new PointerEvent( 'pointerdown', { bubbles : true } ) );

		expect( first.isConnected ).toBe( true );

		document.body.dispatchEvent( new PointerEvent( 'pointerdown', { bubbles : true } ) );

		expect( first.isConnected ).toBe( false );

		const second = open( document.body, { x : 0, y : 0 }, Kind.Diode, () => {} );

		document.dispatchEvent( new KeyboardEvent( 'keydown', { key : 'Escape' } ) );

		expect( second.isConnected ).toBe( false );
	} );
} );
