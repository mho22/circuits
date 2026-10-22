import type { Choice } from '../types/choice.ts';
import type { Point } from '../types/point.ts';
import { open as context } from './context.ts';


const LABELS : Record<Choice, string> = { move : 'Move', symbol : 'Make symbol', copy : 'Copy', delete : 'Delete' };


export function open( parent : HTMLElement, position : Point, act : ( choice : Choice ) => void ) : HTMLElement
{
	return context( parent, position, LABELS, [ 'move', 'symbol', 'copy', 'delete' ], act );
}
