import type { Figure } from '../enums/figure.ts';
import type { Change } from '../types/change.ts';
import type { Point } from '../types/point.ts';
import { open as context } from './context.ts';


const LABELS : Record<Change, string> = { move : 'Move', delete : 'Delete' };


function changes( figure : Figure | null ) : Change[]
{
	if( figure === null ) return [ 'move' ];

	return [ 'move', 'delete' ];
}


export function open( parent : HTMLElement, position : Point, figure : Figure | null, act : ( change : Change ) => void ) : HTMLElement
{
	return context( parent, position, LABELS, changes( figure ), act );
}
