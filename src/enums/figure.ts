export enum Figure
{
	Circle = 1,
	Line = 2,
	Curve = 3,
}


export const NAMES : Record<Figure, string> = {
	[ Figure.Circle ] : 'Circle',
	[ Figure.Line ] : 'Line',
	[ Figure.Curve ] : 'Curve',
};
