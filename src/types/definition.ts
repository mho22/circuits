import type { Point } from './point.ts';
import type { Shape } from './shape.ts';


export interface Definition
{
	name : string;
	drawing : Shape[];
	labels : string[];
	layout : Point[];
	delay : number;
}
