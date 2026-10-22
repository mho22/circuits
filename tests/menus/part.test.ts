// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { Kind } from '../../src/enums/kind.ts';
import { open } from '../../src/menus/part.ts';
import { element } from '../stubs/element.ts';


describe( 'part menu', () =>
{
	beforeEach( () => document.body.replaceChildren() );

	const labels = ( menu : HTMLElement ) => Array.from( menu.children ).map( item => item.textContent );

	it( 'opens at a position with every action for a part with two ends', () =>
	{
		const menu = open( document.body, { x : 10, y : 20 }, element( { kind : Kind.Resistor } ), () => {} );

		expect( menu.parentElement ).toBe( document.body );
		expect( [ menu.style.left, menu.style.top ] ).toEqual( [ '10px', '20px' ] );
		expect( labels( menu ) ).toEqual( [ 'Move', 'Rotate', 'Mirror horizontally', 'Mirror vertically', 'Copy', 'Delete' ] );
	} );

	it( 'offers only a move, a copy and a delete for a ground or a vcc', () =>
	{
		expect( labels( open( document.body, { x : 0, y : 0 }, element( { kind : Kind.Ground } ), () => {} ) ) ).toEqual( [ 'Move', 'Copy', 'Delete' ] );
		expect( labels( open( document.body, { x : 0, y : 0 }, element( { kind : Kind.Supply } ), () => {} ) ) ).toEqual( [ 'Move', 'Copy', 'Delete' ] );
	} );

	it( 'offers a move, a rotate, a copy and a delete for a symbol, and every action for an input or an output', () =>
	{
		expect( labels( open( document.body, { x : 0, y : 0 }, element( { kind : Kind.Symbol } ), () => {} ) ) ).toEqual( [ 'Move', 'Rotate', 'Copy', 'Delete' ] );
		expect( labels( open( document.body, { x : 0, y : 0 }, element( { kind : Kind.Input } ), () => {} ) ) ).toEqual( [ 'Move', 'Rotate', 'Mirror horizontally', 'Mirror vertically', 'Copy', 'Delete' ] );
		expect( labels( open( document.body, { x : 0, y : 0 }, element( { kind : Kind.Output } ), () => {} ) ) ).toEqual( [ 'Move', 'Rotate', 'Mirror horizontally', 'Mirror vertically', 'Copy', 'Delete' ] );
	} );

	it( 'offers to change an led to the next color, from red to green to blue and back to red', () =>
	{
		const menu = ( value : number ) => labels( open( document.body, { x : 0, y : 0 }, element( { kind : Kind.Led, value } ), () => {} ) );

		expect( menu( 0 ) ).toEqual( [ 'Move', 'Rotate', 'Mirror horizontally', 'Mirror vertically', 'Change in green', 'Copy', 'Delete' ] );
		expect( [ menu( 1 )[ 4 ], menu( 2 )[ 4 ] ] ).toEqual( [ 'Change in blue', 'Change in red' ] );
	} );

	it( 'runs the picked action and closes', () =>
	{
		const act = vi.fn();
		const menu = open( document.body, { x : 0, y : 0 }, element( { kind : Kind.Npn } ), act );

		( menu.children[ 3 ] as HTMLButtonElement ).click();

		expect( act ).toHaveBeenCalledWith( 'vertical' );
		expect( menu.isConnected ).toBe( false );
	} );
} );
