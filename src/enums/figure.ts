export enum Figure
{
	Rectangle = 0,
	Circle = 1,
	Line = 2,
}


export const NAMES : Record<Figure, string> = {
	[ Figure.Rectangle ] : 'Rectangle',
	[ Figure.Circle ] : 'Circle',
	[ Figure.Line ] : 'Line',
};
