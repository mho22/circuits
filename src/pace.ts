import { format } from './si.ts';


const SLOW = 0.95;


export function pace( hertz : number | undefined, speed : number ) : string
{
	if( hertz === undefined || speed >= SLOW ) return '';

	return format( hertz * speed, 'Hz' ).replace( /[\d.]+/, digits => String( Math.round( Number( digits ) ) ) );
}
