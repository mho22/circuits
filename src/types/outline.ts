import type { Light } from './light.ts';
import type { Point } from './point.ts';


export interface Outline
{
	low : Point;
	high : Point;
	pins : Point[];
	lights : Light[];
}
