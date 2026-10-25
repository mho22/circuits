import { Color } from '../enums/color.ts';


const THRESHOLD = 1e-4;
const FULL = 0.005;
const FLOOR = 0x1a;


export const LIT : Record<Color, string> = { [ Color.Red ] : '255, 48, 48', [ Color.Green ] : '48, 220, 96', [ Color.Blue ] : '48, 128, 255' };

export const DARK = '#1a1a1a';


export function brightness( current : number ) : number
{
	return current >= THRESHOLD ? Math.min( 1, current / FULL ) : 0;
}

export function mix( lights : [ Color, number ][] ) : string
{
	const channels = lights.map( ( [ color, glow ] ) => LIT[ color ].split( ', ' ).map( value => Number( value ) * glow ) );

	return [ 0, 1, 2 ].map( channel => Math.round( Math.min( 255, Math.max( FLOOR, channels.reduce( ( sum, rgb ) => sum + rgb[ channel ], 0 ) ) ) ) ).join( ', ' );
}
