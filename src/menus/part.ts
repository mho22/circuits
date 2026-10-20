import { type Kind, POINTS } from '../enums/kind.ts';
import type { Action } from '../types/action.ts';
import type { Point } from '../types/point.ts';


const LABELS : Record<Action, string> = { move : 'Move', rotate : 'Rotate', horizontal : 'Mirror horizontally', vertical : 'Mirror vertically', delete : 'Delete' };


export function open( parent : HTMLElement, position : Point, kind : Kind, act : ( action : Action ) => void ) : HTMLElement
{
	const menu = document.createElement( 'div' );
	const actions : Action[] = POINTS.includes( kind ) ? [ 'move', 'delete' ] : [ 'move', 'rotate', 'horizontal', 'vertical', 'delete' ];

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

		item.textContent = LABELS[ action ];
		item.addEventListener( 'click', () => { close(); act( action ); } );

		menu.append( item );
	} );

	document.addEventListener( 'pointerdown', outside );
	document.addEventListener( 'keydown', escape );

	parent.append( menu );

	return menu;
}
