import type { Point } from './point.ts';


export interface Pin
{
	point : Point;
	output : boolean;
	label : string;
}
