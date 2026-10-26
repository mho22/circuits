const SLOW = 0.95;


export function pace( speed : number ) : string
{
	if( speed >= SLOW ) return '';

	return ` × ${ Number( speed.toPrecision( 1 ) ) }`;
}
