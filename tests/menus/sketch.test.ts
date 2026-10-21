// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { open } from '../../src/menus/sketch.ts';


describe( 'sketch menu', () =>
{
	beforeEach( () => document.body.replaceChildren() );

	it( 'opens at a position with only the move', () =>
	{
		const menu = open( document.body, { x : 10, y : 20 }, () => {} );

		expect( [ menu.style.left, menu.style.top ] ).toEqual( [ '10px', '20px' ] );
		expect( Array.from( menu.children ).map( item => item.textContent ) ).toEqual( [ 'Move' ] );
	} );

	it( 'runs the move and closes', () =>
	{
		const act = vi.fn();
		const menu = open( document.body, { x : 0, y : 0 }, act );

		( menu.children[ 0 ] as HTMLButtonElement ).click();

		expect( act ).toHaveBeenCalledWith( 'move' );
		expect( menu.isConnected ).toBe( false );
	} );
} );
