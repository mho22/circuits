import { Kind, POINTS } from '../enums/kind.ts';
import type { Action } from '../types/action.ts';
import type { Point } from '../types/point.ts';
import { open as context } from './context.ts';


const LABELS : Record<Action, string> = { move : 'Move', rotate : 'Rotate', horizontal : 'Mirror horizontally', vertical : 'Mirror vertically', delete : 'Delete' };


function actions( kind : Kind ) : Action[]
{
	if( kind === Kind.Symbol ) return [ 'move', 'rotate', 'delete' ];

	if( POINTS.includes( kind ) ) return [ 'move', 'delete' ];

	return [ 'move', 'rotate', 'horizontal', 'vertical', 'delete' ];
}


export function open( parent : HTMLElement, position : Point, kind : Kind, act : ( action : Action ) => void ) : HTMLElement
{
	return context( parent, position, LABELS, actions( kind ), act );
}
