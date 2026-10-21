// @vitest-environment happy-dom

import { beforeEach, describe, it, expect, vi } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { Palette } from '../src/palette.ts';


describe( 'palette', () =>
{
	let nav : HTMLElement;

	beforeEach( () =>
	{
		document.body.replaceChildren();
		document.body.className = '';

		nav = document.createElement( 'nav' );

		document.body.append( nav );
	} );

	const group = () => nav.querySelector<HTMLElement>( '.palette' )!;
	const bar = () => document.querySelector<HTMLElement>( '.sketch' )!;
	const buttons = () => Array.from( group().querySelectorAll( 'button' ) );

	it( 'shows the figure buttons and the bar only while open and marks the page as sketching', () =>
	{
		const palette = new Palette( nav, document.body, () => {}, () => {} );

		expect( [ group().hidden, bar().hidden ] ).toEqual( [ true, true ] );
		expect( buttons().map( button => button.textContent ) ).toEqual( [ 'LineL', 'CircleC', 'CurveU' ] );

		palette.open();

		expect( [ group().hidden, bar().hidden, document.body.classList.contains( 'sketching' ) ] ).toEqual( [ false, false, true ] );

		palette.close();

		expect( [ group().hidden, bar().hidden, document.body.classList.contains( 'sketching' ) ] ).toEqual( [ true, true, false ] );
	} );

	it( 'picks a figure, marks the selected one and gives its hint', () =>
	{
		const pick = vi.fn();
		const palette = new Palette( nav, document.body, pick, () => {} );

		buttons()[ 0 ].click();

		expect( pick ).toHaveBeenCalledWith( Figure.Line );

		palette.select( Figure.Circle );

		expect( buttons().map( button => button.classList.contains( 'active' ) ) ).toEqual( [ false, true, false ] );
		expect( bar().querySelector( 'span' )!.textContent ).toBe( 'Press on the center and drag to the radius, or click for a dot. Right-click a line or a curve to delete it, a circle or a pin to change it.' );
	} );

	it( 'calls done from its bar', () =>
	{
		const done = vi.fn();

		new Palette( nav, document.body, () => {}, done );

		bar().querySelector( 'button' )!.click();

		expect( done ).toHaveBeenCalledOnce();
	} );
} );
