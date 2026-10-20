import type { Point } from '../types/point.ts';


export function edit( parent : HTMLElement, position : Point, text : string, commit : ( text : string ) => void ) : HTMLInputElement
{
	const input = document.createElement( 'input' );

	let closed = false;

	const close = () =>
	{
		if( closed ) return;

		closed = true;

		input.remove();
	};

	input.className = 'editor';
	input.value = text;
	input.style.left = `${ position.x }px`;
	input.style.top = `${ position.y }px`;

	input.addEventListener( 'keydown', ( event : KeyboardEvent ) =>
	{
		if( event.key === 'Enter' ) commit( input.value );

		if( event.key === 'Enter' || event.key === 'Escape' ) close();
	} );

	input.addEventListener( 'blur', close );

	parent.append( input );

	input.focus();
	input.select();

	return input;
}
