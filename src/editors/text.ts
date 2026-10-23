import type { Point } from '../types/point.ts';


export function edit( parent : HTMLElement, position : Point, text : string, unit : string, commit : ( text : string ) => void ) : HTMLInputElement
{
	const field = document.createElement( 'label' );
	const input = document.createElement( 'input' );
	const suffix = document.createElement( 'span' );

	let closed = false;

	const close = () =>
	{
		if( closed ) return;

		closed = true;

		field.remove();

		document.removeEventListener( 'zoom', close );
	};

	field.className = 'editor';
	field.style.left = `${ position.x }px`;
	field.style.top = `${ position.y }px`;

	input.value = text;

	suffix.textContent = unit;

	input.addEventListener( 'keydown', ( event : KeyboardEvent ) =>
	{
		if( event.key === 'Enter' ) commit( input.value );

		if( event.key === 'Enter' || event.key === 'Escape' ) close();
	} );

	input.addEventListener( 'blur', close );

	document.addEventListener( 'zoom', close );

	field.append( input, suffix );
	parent.append( field );

	input.focus();
	input.select();

	return input;
}
