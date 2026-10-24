import { Color } from '../enums/color.ts';


const THRESHOLD = 1e-4;
const FULL = 0.005;


export const LIT : Record<Color, string> = { [ Color.Red ] : '255, 48, 48', [ Color.Green ] : '48, 220, 96', [ Color.Blue ] : '48, 128, 255' };

export const DARK = '#1a1a1a';


export function brightness( current : number ) : number
{
	return current >= THRESHOLD ? Math.min( 1, current / FULL ) : 0;
}
