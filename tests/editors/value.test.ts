// @vitest-environment happy-dom

import { describe, it, expect, vi } from 'vitest';
import { edit } from '../../src/editors/value.ts';


function press( input : HTMLInputElement, key : string ) : void
{
	input.dispatchEvent( new KeyboardEvent( 'keydown', { key } ) );
}


describe( 'value editor', () =>
{
	it( 'opens a focused input with the text at a position', () =>
	{
		const input = edit( document.body, { x : 10, y : 20 }, '1 kΩ', () => {} );

		expect( input.parentElement ).toBe( document.body );
		expect( document.activeElement ).toBe( input );
		expect( input.value ).toBe( '1 kΩ' );
		expect( [ input.style.left, input.style.top ] ).toEqual( [ '10px', '20px' ] );

		input.remove();
	} );

	it( 'commits the text and closes on Enter', () =>
	{
		const commit = vi.fn();
		const input = edit( document.body, { x : 0, y : 0 }, '1 kΩ', commit );

		input.value = '4.7k';

		press( input, 'Enter' );

		expect( commit ).toHaveBeenCalledExactlyOnceWith( '4.7k' );
		expect( input.isConnected ).toBe( false );
	} );

	it( 'closes without a commit on Escape', () =>
	{
		const commit = vi.fn();
		const input = edit( document.body, { x : 0, y : 0 }, '5 V', commit );

		press( input, 'Escape' );

		expect( commit ).not.toHaveBeenCalled();
		expect( input.isConnected ).toBe( false );
	} );

	it( 'closes without a commit when it loses focus', () =>
	{
		const commit = vi.fn();
		const input = edit( document.body, { x : 0, y : 0 }, '5 V', commit );

		input.blur();

		expect( commit ).not.toHaveBeenCalled();
		expect( input.isConnected ).toBe( false );
	} );

	it( 'ignores other keys', () =>
	{
		const commit = vi.fn();
		const input = edit( document.body, { x : 0, y : 0 }, '5 V', commit );

		press( input, 'a' );

		expect( commit ).not.toHaveBeenCalled();
		expect( input.isConnected ).toBe( true );

		input.remove();
	} );
} );
