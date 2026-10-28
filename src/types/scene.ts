import type { Element } from './element.ts';
import type { Point } from './point.ts';


export interface Scene
{
	revision : number;
	elements : Element[];
	starts : number[];
	junctions : { point : Point, wire : number }[];
}
