import type { Kind } from '../enums/kind.ts';
import type { Point } from './point.ts';


export interface Element
{
	id : number;
	kind : Kind;
	from : Point;
	to : Point;
	value : number;
	potentials : [ number, number ];
	current : number;
}
