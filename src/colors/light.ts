import { Color } from '../enums/color.ts';


const THRESHOLD = 1e-4;
const FULL = 0.005;
const FLOOR = 0x1a;


export const LIT : Record<Color, string> = { [ Color.Red ] : '255, 48, 48', [ Color.Green ] : '48, 220, 96', [ Color.Blue ] : '48, 128, 255' };

export const DARK = '#1a1a1a';

export const CHANNELS = Object.fromEntries( Object.entries( LIT ).map( ( [ color, rgb ] ) => [ color, rgb.split( ', ' ).map( Number ) ] ) ) as Record<Color, number[]>;

const FILLS = new Map<number, string>();


function level( sum : number ) : number
{
	return Math.round( Math.min( 255, Math.max( FLOOR, sum ) ) );
}

function rgb( packed : number ) : string
{
	return `${ packed >> 16 }, ${ packed >> 8 & 0xff }, ${ packed & 0xff }`;
}


export function brightness( current : number ) : number
{
	return current >= THRESHOLD ? Math.min( 1, current / FULL ) : 0;
}

export function fill( red : number, green : number, blue : number ) : string
{
	const packed = level( red ) << 16 | level( green ) << 8 | level( blue );
	const known = FILLS.get( packed );

	if( known ) return known;

	const style = `rgb(${ rgb( packed ) })`;

	FILLS.set( packed, style );

	return style;
}
