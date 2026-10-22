import { Color, NAMES, NEXT } from '../enums/color.ts';
import { Kind, PINS, POINTS } from '../enums/kind.ts';
import type { Action } from '../types/action.ts';
import type { Element } from '../types/element.ts';
import type { Point } from '../types/point.ts';
import { open as context } from './context.ts';


const LABELS : Record<Exclude<Action, 'color' | 'label'>, string> = { move : 'Move', rotate : 'Rotate', horizontal : 'Mirror horizontally', vertical : 'Mirror vertically', copy : 'Copy', delete : 'Delete' };


function actions( kind : Kind ) : Action[]
{
	if( kind === Kind.Symbol ) return [ 'move', 'rotate', 'copy', 'delete' ];

	if( POINTS.includes( kind ) ) return [ 'move', 'copy', 'delete' ];

	if( PINS.includes( kind ) ) return [ 'move', 'rotate', 'horizontal', 'vertical', 'label', 'copy', 'delete' ];

	if( kind === Kind.Led ) return [ 'move', 'rotate', 'horizontal', 'vertical', 'color', 'copy', 'delete' ];

	return [ 'move', 'rotate', 'horizontal', 'vertical', 'copy', 'delete' ];
}


export function open( parent : HTMLElement, position : Point, element : Element, act : ( action : Action ) => void ) : HTMLElement
{
	const labels = { ... LABELS, color : `Change in ${ NAMES[ NEXT[ element.value as Color ] ].toLowerCase() }`, label : element.label ? 'Edit label' : 'Add label' };

	return context( parent, position, labels, actions( element.kind ), act );
}
