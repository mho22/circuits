import type { Insertion } from '../types/insertion.ts';
import type { Point } from '../types/point.ts';
import { open as context } from './context.ts';


const LABELS : Record<Insertion, string> = { paste : 'Paste' };


export function open( parent : HTMLElement, position : Point, act : ( insertion : Insertion ) => void ) : HTMLElement
{
	return context( parent, position, LABELS, [ 'paste' ], act );
}
