// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { open } from '../../src/menus/drawing.ts';


describe( 'drawing menu', () =>
{
	beforeEach( () => document.body.replaceChildren() );

	it( 'opens at a position with a move and a delete', () =>
	{
		const menu = open( document.body, { x : 10, y : 20 }, () => {} );

		expect( [ menu.style.left, menu.style.top ] ).toEqual( [ '10px', '20px' ] );
		expect( Array.from( menu.children ).map( item => item.textContent ) ).toEqual( [ 'Move', 'Delete' ] );
	} );

	it( 'runs the picked edit and closes', () =>
	{
		const act = vi.fn();
		const menu = open( document.body, { x : 0, y : 0 }, act );

		( menu.children[ 1 ] as HTMLButtonElement ).click();

		expect( act ).toHaveBeenCalledWith( 'delete' );
		expect( menu.isConnected ).toBe( false );
	} );
} );
