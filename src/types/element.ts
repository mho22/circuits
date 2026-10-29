import type { Kind } from '../enums/kind.ts';
import type { Definition } from './definition.ts';
import type { Outline } from './outline.ts';
import type { Point } from './point.ts';


export interface Element
{
	id : number;
	kind : Kind;
	from : Point;
	to : Point;
	value : number;
	mirrored : boolean;
	rotation : number;
	potentials : number[];
	currents : number[];
	outline? : Outline;
	definition? : Definition;
	label? : string;
	listening? : boolean;
	on? : boolean;
	lights? : number[];
}
