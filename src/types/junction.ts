import type { Point } from './point.ts';


export interface Junction
{
	point : Point;
	potentials : number[];
	lanes? : number;
}
