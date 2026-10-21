// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { Figure } from '../../src/enums/figure.ts';
import { open } from '../../src/menus/sketch.ts';


describe( 'sketch menu', () =>
{
	beforeEach( () => document.body.replaceChildren() );

	const labels = ( menu : HTMLElement ) => Array.from( menu.children ).map( item => item.textContent );

	it( 'opens at a position with every change for a circle', () =>
	{
		const menu = open( document.body, { x : 10, y : 20 }, Figure.Circle, () => {} );

		expect( [ menu.style.left, menu.style.top ] ).toEqual( [ '10px', '20px' ] );
		expect( labels( menu ) ).toEqual( [ 'Move', 'Delete' ] );
	} );

	it( 'offers only the move for a pin', () =>
	{
		expect( labels( open( document.body, { x : 0, y : 0 }, null, () => {} ) ) ).toEqual( [ 'Move' ] );
	} );

	it( 'runs the picked change and closes', () =>
	{
		const act = vi.fn();
		const menu = open( document.body, { x : 0, y : 0 }, Figure.Circle, act );

		( menu.children[ 1 ] as HTMLButtonElement ).click();

		expect( act ).toHaveBeenCalledWith( 'delete' );
		expect( menu.isConnected ).toBe( false );
	} );
} );
