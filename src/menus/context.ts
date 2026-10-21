import type { Point } from '../types/point.ts';


export function open<Action extends string>( parent : HTMLElement, position : Point, labels : Record<Action, string>, actions : Action[], act : ( action : Action ) => void ) : HTMLElement
{
	const menu = document.createElement( 'div' );

	const close = () =>
	{
		menu.remove();

		document.removeEventListener( 'pointerdown', outside );
		document.removeEventListener( 'keydown', escape );
	};

	const outside = ( event : PointerEvent ) => { if( ! menu.contains( event.target as Node ) ) close(); };
	const escape = ( event : KeyboardEvent ) => { if( event.key === 'Escape' ) close(); };

	menu.className = 'context';
	menu.style.left = `${ position.x }px`;
	menu.style.top = `${ position.y }px`;

	actions.forEach( action =>
	{
		const item = document.createElement( 'button' );

		item.textContent = labels[ action ];
		item.addEventListener( 'click', () => { close(); act( action ); } );

		menu.append( item );
	} );

	document.addEventListener( 'pointerdown', outside );
	document.addEventListener( 'keydown', escape );

	parent.append( menu );

	return menu;
}
