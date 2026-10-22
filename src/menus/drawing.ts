import type { Edit } from '../types/edit.ts';
import type { Point } from '../types/point.ts';
import { open as context } from './context.ts';


const LABELS : Record<Edit, string> = { move : 'Move', delete : 'Delete' };


export function open( parent : HTMLElement, position : Point, act : ( edit : Edit ) => void ) : HTMLElement
{
	return context( parent, position, LABELS, [ 'move', 'delete' ], act );
}
