// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { PART_GROUPS, PIN_GROUPS, Toolbar } from '../src/toolbar.ts';


const GROUPS = [ ... PART_GROUPS, ... PIN_GROUPS ];


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
		new Toolbar( parent, GROUPS, () => {} );

		expect( picks().map( pick => pick.textContent ) ).toEqual( [ 'WireW', 'ResistorR', 'DiodeD', 'BatteryB', 'InputI' ] );
	} );

	it( 'lists the parts of a group in its menu', () =>
	{
		new Toolbar( parent, GROUPS, () => {} );

		expect( menus().map( menu => Array.from( menu.children ).map( item => item.textContent ) ) ).toEqual( [
			[ 'WireW', 'SwitchS' ],
			[ 'ResistorR', 'CapacitorC', 'InductorL' ],
			[ 'DiodeD', 'LEDE', 'NPNN', 'PNPP', 'NMOSM', 'PMOSF' ],
			[ 'BatteryB', 'VCCV', 'ToggleT', 'ClockK', 'GroundG' ],
			[ 'InputI', 'OutputO' ],
		] );
	} );

	it( 'names the menu of each group', () =>
	{
		new Toolbar( parent, GROUPS, () => {} );

		expect( opens().map( open => open.getAttribute( 'aria-label' ) ) ).toEqual( [ 'Wiring parts', 'Passive parts', 'Semiconductors parts', 'Sources parts', 'Pins parts' ] );
	} );

	it( 'opens one menu at a time', () =>
	{
		new Toolbar( parent, GROUPS, () => {} );

		opens()[ 0 ].click();

		expect( menus().map( menu => menu.hidden ) ).toEqual( [ false, true, true, true, true ] );

		opens()[ 1 ].click();

		expect( menus().map( menu => menu.hidden ) ).toEqual( [ true, false, true, true, true ] );

		opens()[ 1 ].click();

		expect( menus().map( menu => menu.hidden ) ).toEqual( [ true, true, true, true, true ] );
	} );

	it( 'picks a part from a menu and closes the menu', () =>
	{
		const pick = vi.fn();

		new Toolbar( parent, GROUPS, pick );

		opens()[ 1 ].click();
		( menus()[ 1 ].children[ 1 ] as HTMLButtonElement ).click();

		expect( pick ).toHaveBeenCalledWith( Kind.Capacitor );
		expect( menus()[ 1 ].hidden ).toBe( true );
	} );

	it( 'shows the selected part on its group button and picks it again from there', () =>
	{
		const pick = vi.fn();
		const toolbar = new Toolbar( parent, GROUPS, pick );

		toolbar.select( Kind.Inductor );

		expect( picks().map( pick => [ pick.textContent, pick.classList.contains( 'active' ) ] ) ).toEqual( [ [ 'WireW', false ], [ 'InductorL', true ], [ 'DiodeD', false ], [ 'BatteryB', false ], [ 'InputI', false ] ] );

		picks()[ 1 ].click();

		expect( pick ).toHaveBeenCalledWith( Kind.Inductor );
	} );

	it( 'keeps the last part on its group button and marks no group without a part', () =>
	{
		const toolbar = new Toolbar( parent, GROUPS, () => {} );

		toolbar.select( Kind.Inductor );
		toolbar.select( null );

		expect( picks().map( pick => [ pick.textContent, pick.classList.contains( 'active' ) ] ) ).toEqual( [ [ 'WireW', false ], [ 'InductorL', false ], [ 'DiodeD', false ], [ 'BatteryB', false ], [ 'InputI', false ] ] );
	} );

	it( 'hides the groups that hold none of the allowed parts', () =>
	{
		const toolbar = new Toolbar( parent, GROUPS, () => {} );
		const groups = () => Array.from( parent.querySelectorAll<HTMLElement>( '.group' ) ).map( group => group.hidden );

		expect( groups() ).toEqual( [ false, false, false, false, false ] );

		toolbar.limit( [ Kind.Input, Kind.Output ] );

		expect( groups() ).toEqual( [ true, true, true, true, false ] );

		toolbar.limit( [ Kind.Switch, Kind.Ground ] );

		expect( groups() ).toEqual( [ false, true, true, false, true ] );
	} );

	it( 'closes the menus on a press outside the toolbar or on escape', () =>
	{
		new Toolbar( parent, GROUPS, () => {} );

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
