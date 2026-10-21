export const NANO = 1e-9;

const PREFIXES : [ number, string ][] = [ [ 9, 'G' ], [ 6, 'M' ], [ 3, 'k' ], [ 0, '' ], [ -3, 'm' ], [ -6, 'µ' ], [ -9, 'n' ], [ -12, 'p' ] ];


export function format( value : number, unit : string ) : string
{
	if( ! Number.isFinite( value ) ) return `— ${ unit }`;

	if( value === 0 ) return `0 ${ unit }`;

	const [ exponent, prefix ] = PREFIXES.find( ( [ exponent ] ) => Math.abs( value ) >= 10 ** exponent * 0.9995 ) ?? PREFIXES[ PREFIXES.length - 1 ];

	return `${ Number( ( value / 10 ** exponent ).toPrecision( 3 ) ) } ${ prefix }${ unit }`;
}
