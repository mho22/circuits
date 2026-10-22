// @vitest-environment happy-dom

import { describe, it, expect } from 'vitest';
import { edit } from '../../src/editors/value.ts';


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
} );
