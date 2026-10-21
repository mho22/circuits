export enum Color
{
	Red = 0,
	Green = 1,
	Blue = 2,
}


export const NAMES : Record<Color, string> = {
	[ Color.Red ] : 'Red',
	[ Color.Green ] : 'Green',
	[ Color.Blue ] : 'Blue',
};

export const NEXT : Record<Color, Color> = {
	[ Color.Red ] : Color.Green,
	[ Color.Green ] : Color.Blue,
	[ Color.Blue ] : Color.Red,
};
