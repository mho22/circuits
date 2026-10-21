// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { Figure } from '../../src/enums/figure.ts';
import { open } from '../../src/menus/sketch.ts';


describe( 'sketch menu', () =>
{
	beforeEach( () => document.body.replaceChildren() );

	const labels = ( menu : HTMLElement ) => Array.from( menu.children ).map( item => item.textContent );

	it( 'opens at a position with every change for a rectangle', () =>
	{
		const menu = open( document.body, { x : 10, y : 20 }, Figure.Rectangle, () => {} );

		expect( [ menu.style.left, menu.style.top ] ).toEqual( [ '10px', '20px' ] );
		expect( labels( menu ) ).toEqual( [ 'Move', 'Rotate', 'Delete' ] );
	} );

	it( 'offers only the changes that alter a circle or a pin', () =>
	{
		expect( labels( open( document.body, { x : 0, y : 0 }, Figure.Circle, () => {} ) ) ).toEqual( [ 'Move', 'Delete' ] );
		expect( labels( open( document.body, { x : 0, y : 0 }, null, () => {} ) ) ).toEqual( [ 'Move' ] );
	} );

	it( 'runs the picked change and closes', () =>
	{
		const act = vi.fn();
		const menu = open( document.body, { x : 0, y : 0 }, Figure.Rectangle, act );

		( menu.children[ 1 ] as HTMLButtonElement ).click();

		expect( act ).toHaveBeenCalledWith( 'rotate' );
		expect( menu.isConnected ).toBe( false );
	} );
} );
