// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { open } from '../../src/menus/selection.ts';


describe( 'selection menu', () =>
{
	beforeEach( () => document.body.replaceChildren() );

	it( 'opens at a position with a move, a make symbol and a delete', () =>
	{
		const menu = open( document.body, { x : 10, y : 20 }, () => {} );

		expect( [ menu.style.left, menu.style.top ] ).toEqual( [ '10px', '20px' ] );
		expect( Array.from( menu.children ).map( item => item.textContent ) ).toEqual( [ 'Move', 'Make symbol', 'Delete' ] );
	} );

	it( 'runs the picked choice and closes', () =>
	{
		const act = vi.fn();
		const menu = open( document.body, { x : 0, y : 0 }, act );

		( menu.children[ 1 ] as HTMLButtonElement ).click();

		expect( act ).toHaveBeenCalledWith( 'symbol' );
		expect( menu.isConnected ).toBe( false );
	} );
} );
