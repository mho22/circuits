import type { Change } from '../types/change.ts';
import type { Point } from '../types/point.ts';
import { open as context } from './context.ts';


const LABELS : Record<Change, string> = { move : 'Move' };


export function open( parent : HTMLElement, position : Point, act : ( change : Change ) => void ) : HTMLElement
{
	return context( parent, position, LABELS, [ 'move' ], act );
}
