import type { Figure } from '../enums/figure.ts';
import type { Point } from './point.ts';


export type Shape =
	| { figure : Figure.Rectangle, from : Point, to : Point }
	| { figure : Figure.Circle, center : Point, radius : number }
	| { figure : Figure.Line, from : Point, to : Point };
