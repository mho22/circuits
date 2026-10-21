import type { Point } from './point.ts';


export interface Arc
{
	center : Point;
	radius : number;
	start : number;
	end : number;
	counterclockwise : boolean;
}
