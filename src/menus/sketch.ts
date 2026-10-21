import { Figure } from '../enums/figure.ts';
import type { Change } from '../types/change.ts';
import type { Point } from '../types/point.ts';
import { open as context } from './context.ts';


const LABELS : Record<Change, string> = { move : 'Move', rotate : 'Rotate', delete : 'Delete' };


function changes( figure : Figure | null ) : Change[]
{
	if( figure === null ) return [ 'move' ];

	if( figure === Figure.Circle ) return [ 'move', 'delete' ];

	return [ 'move', 'rotate', 'delete' ];
}


export function open( parent : HTMLElement, position : Point, figure : Figure | null, act : ( change : Change ) => void ) : HTMLElement
{
	return context( parent, position, LABELS, changes( figure ), act );
}
