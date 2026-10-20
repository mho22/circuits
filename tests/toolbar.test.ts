// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { Toolbar } from '../src/toolbar.ts';


describe( 'toolbar', () =>
{
	let parent : HTMLElement;

	beforeEach( () =>
	{
		document.body.replaceChildren();

		parent = document.createElement( 'nav' );

		document.body.append( parent );
	} );

	const picks = () => Array.from( parent.querySelectorAll<HTMLButtonElement>( '.pick' ) );
	const menus = () => Array.from( parent.querySelectorAll<HTMLElement>( '.menu' ) );
	const opens = () => Array.from( parent.querySelectorAll<HTMLButtonElement>( '.open' ) );

	it( 'shows one button per group with the first part and its letter', () =>
	{
		new Toolbar( parent, () => {} );

		expect( picks().map( pick => pick.textContent ) ).toEqual( [ 'WireW', 'ResistorR', 'BatteryB', 'DiodeD' ] );
	} );

	it( 'lists the parts of a group in its menu', () =>
	{
		new Toolbar( parent, () => {} );

		expect( menus().map( menu => Array.from( menu.children ).map( item => item.textContent ) ) ).toEqual( [
			[ 'WireW', 'GroundG', 'SwitchS' ],
			[ 'ResistorR', 'CapacitorC', 'InductorL' ],
			[ 'DiodeD', 'NPNN', 'PNPP', 'NMOSM', 'PMOSF' ],
		] );
	} );

	it( 'gives no menu to a group with one part', () =>
	{
		new Toolbar( parent, () => {} );

		expect( opens().map( open => open.getAttribute( 'aria-label' ) ) ).toEqual( [ 'Wiring parts', 'Passive parts', 'Semiconductors parts' ] );
	} );

	it( 'opens one menu at a time', () =>
	{
		new Toolbar( parent, () => {} );

		opens()[ 0 ].click();

		expect( menus().map( menu => menu.hidden ) ).toEqual( [ false, true, true ] );

		opens()[ 1 ].click();

		expect( menus().map( menu => menu.hidden ) ).toEqual( [ true, false, true ] );

		opens()[ 1 ].click();

		expect( menus().map( menu => menu.hidden ) ).toEqual( [ true, true, true ] );
	} );

	it( 'picks a part from a menu and closes the menu', () =>
	{
		const pick = vi.fn();

		new Toolbar( parent, pick );

		opens()[ 1 ].click();
		( menus()[ 1 ].children[ 1 ] as HTMLButtonElement ).click();

		expect( pick ).toHaveBeenCalledWith( Kind.Capacitor );
		expect( menus()[ 1 ].hidden ).toBe( true );
	} );

	it( 'shows the selected part on its group button and picks it again from there', () =>
	{
		const pick = vi.fn();
		const toolbar = new Toolbar( parent, pick );

		toolbar.select( Kind.Inductor );

		expect( picks().map( pick => [ pick.textContent, pick.classList.contains( 'active' ) ] ) ).toEqual( [ [ 'WireW', false ], [ 'InductorL', true ], [ 'BatteryB', false ], [ 'DiodeD', false ] ] );

		picks()[ 1 ].click();

		expect( pick ).toHaveBeenCalledWith( Kind.Inductor );
	} );

	it( 'closes the menus on a press outside the toolbar or on escape', () =>
	{
		new Toolbar( parent, () => {} );

		opens()[ 0 ].click();
		document.body.dispatchEvent( new PointerEvent( 'pointerdown', { bubbles : true } ) );

		expect( menus()[ 0 ].hidden ).toBe( true );

		opens()[ 0 ].click();
		menus()[ 0 ].dispatchEvent( new PointerEvent( 'pointerdown', { bubbles : true } ) );

		expect( menus()[ 0 ].hidden ).toBe( false );

		document.dispatchEvent( new KeyboardEvent( 'keydown', { key : 'Escape' } ) );

		expect( menus()[ 0 ].hidden ).toBe( true );
	} );
} );
