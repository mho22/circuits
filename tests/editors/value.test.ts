// @vitest-environment happy-dom

import { describe, it, expect, vi } from 'vitest';
import { edit } from '../../src/editors/value.ts';


function press( input : HTMLInputElement, key : string ) : void
{
	input.dispatchEvent( new KeyboardEvent( 'keydown', { key } ) );
}

function type( input : HTMLInputElement, text : string ) : void
{
	input.value = text;
	input.dispatchEvent( new Event( 'input' ) );
}


describe( 'value editor', () =>
{
	it( 'opens a focused input with the number and its unit at a position', () =>
	{
		const input = edit( document.body, { x : 10, y : 20 }, '4700', 'Ω', () => {} );
		const field = input.parentElement!;

		expect( field.parentElement ).toBe( document.body );
		expect( document.activeElement ).toBe( input );
		expect( input.value ).toBe( '4700' );
		expect( input.inputMode ).toBe( 'decimal' );
		expect( field.lastElementChild!.textContent ).toBe( 'Ω' );
		expect( [ field.style.left, field.style.top ] ).toEqual( [ '10px', '20px' ] );

		field.remove();
	} );

	it( 'keeps only a number in the input', () =>
	{
		const input = edit( document.body, { x : 0, y : 0 }, '', 'V', () => {} );

		type( input, '4,7k' );

		expect( input.value ).toBe( '4,7' );

		type( input, '-9 V' );

		expect( input.value ).toBe( '-9' );

		input.parentElement!.remove();
	} );

	it( 'commits the text and closes on Enter', () =>
	{
		const commit = vi.fn();
		const input = edit( document.body, { x : 0, y : 0 }, '1000', 'Ω', commit );

		type( input, '2,2' );
		press( input, 'Enter' );

		expect( commit ).toHaveBeenCalledExactlyOnceWith( '2,2' );
		expect( input.parentElement!.isConnected ).toBe( false );
	} );

	it( 'closes without a commit on Escape', () =>
	{
		const commit = vi.fn();
		const input = edit( document.body, { x : 0, y : 0 }, '5', 'V', commit );

		press( input, 'Escape' );

		expect( commit ).not.toHaveBeenCalled();
		expect( input.parentElement!.isConnected ).toBe( false );
	} );

	it( 'closes without a commit when it loses focus', () =>
	{
		const commit = vi.fn();
		const input = edit( document.body, { x : 0, y : 0 }, '5', 'V', commit );

		input.blur();

		expect( commit ).not.toHaveBeenCalled();
		expect( input.parentElement!.isConnected ).toBe( false );
	} );

	it( 'ignores other keys', () =>
	{
		const commit = vi.fn();
		const input = edit( document.body, { x : 0, y : 0 }, '5', 'V', commit );

		press( input, 'a' );

		expect( commit ).not.toHaveBeenCalled();
		expect( input.parentElement!.isConnected ).toBe( true );

		input.parentElement!.remove();
	} );
} );
