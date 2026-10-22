import { clean } from '../decimal.ts';
import type { Point } from '../types/point.ts';
import { edit as write } from './text.ts';


export function edit( parent : HTMLElement, position : Point, text : string, unit : string, commit : ( text : string ) => void ) : HTMLInputElement
{
	const input = write( parent, position, text, unit, commit );

	input.inputMode = 'decimal';

	input.addEventListener( 'input', () =>
	{
		const cleaned = clean( input.value );

		if( cleaned !== input.value ) input.value = cleaned;
	} );

	return input;
}
