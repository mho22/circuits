// @vitest-environment happy-dom

import { beforeEach, describe, it, expect } from 'vitest';
import { Trail } from '../src/trail.ts';


describe( 'trail', () =>
{
	let parent : HTMLElement;

	beforeEach( () =>
	{
		document.body.replaceChildren();

		parent = document.createElement( 'nav' );

		document.body.append( parent );
	} );

	const element = () => parent.querySelector<HTMLElement>( '.trail' )!;

	it( 'stays hidden on the main page', () =>
	{
		const trail = new Trail( parent );

		expect( element().hidden ).toBe( true );

		trail.update( [] );

		expect( element().hidden ).toBe( true );
	} );

	it( 'shows the names of the symbols it is in, from the outer one to the inner one, with the outer ones apart', () =>
	{
		const trail = new Trail( parent );

		trail.update( [ 'foo', 'bar', 'baz' ] );

		expect( [ element().hidden, element().textContent ] ).toEqual( [ false, 'foo > bar > baz' ] );
		expect( element().querySelector( 'span' )!.textContent ).toBe( 'foo > bar > ' );
		expect( element().lastChild!.textContent ).toBe( 'baz' );

		trail.update( [ 'foo', 'bar' ] );

		expect( element().textContent ).toBe( 'foo > bar' );
		expect( element().querySelector( 'span' )!.textContent ).toBe( 'foo > ' );

		trail.update( [ 'foo' ] );

		expect( element().querySelector( 'span' )!.textContent ).toBe( '' );
		expect( element().textContent ).toBe( 'foo' );

		trail.update( [] );

		expect( [ element().hidden, element().textContent ] ).toEqual( [ true, '' ] );
	} );

	it( 'scrolls to the inner name when the names are longer than its space', () =>
	{
		const trail = new Trail( parent );

		Object.defineProperty( element(), 'scrollWidth', { value : 120 } );

		trail.update( [ 'foo', 'bar' ] );

		expect( element().scrollLeft ).toBe( 120 );
	} );
} );
