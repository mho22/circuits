import type { Color } from '../enums/color.ts';
import type { Point } from './point.ts';


export interface Light
{
	point : Point;
	color : Color;
}
