const PREFIXES : [ number, string ][] = [ [ 9, 'G' ], [ 6, 'M' ], [ 3, 'k' ], [ 0, '' ], [ -3, 'm' ], [ -6, 'µ' ], [ -9, 'n' ], [ -12, 'p' ] ];

const EXPONENTS : Record<string, number> = { G : 9, M : 6, k : 3, K : 3, '' : 0, m : -3, u : -6, µ : -6, n : -9, p : -12 };

const NOTATION = /^(-?(?:\d+\.?\d*|\.\d+))\s*([GMkKmuµnp]?)[a-zA-ZΩ]*$/;


export function format( value : number, unit : string ) : string
{
	if( ! Number.isFinite( value ) ) return `— ${ unit }`;

	if( value === 0 ) return `0 ${ unit }`;

	const [ exponent, prefix ] = PREFIXES.find( ( [ exponent ] ) => Math.abs( value ) >= 10 ** exponent * 0.9995 ) ?? PREFIXES[ PREFIXES.length - 1 ];

	return `${ Number( ( value / 10 ** exponent ).toPrecision( 3 ) ) } ${ prefix }${ unit }`;
}

export function parse( text : string ) : number
{
	const match = text.trim().match( NOTATION );

	if( ! match ) return NaN;

	return Number( `${ match[ 1 ] }e${ EXPONENTS[ match[ 2 ] ] }` );
}
