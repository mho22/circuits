import type { Definition } from './definition.ts';
import type { Element } from './element.ts';


export interface Layer
{
	number : number;
	definition : Definition;
	parts : Element[];
}
