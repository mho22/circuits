import type { Point } from './point.ts';


export interface Branch
{
	from : Point;
	to : Point;
	voltage : number;
	current : number;
}
