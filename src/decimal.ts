const SEPARATORS = /[.,]/;

const NUMBER = /^-?(?:\d+(?:[.,]\d*)?|[.,]\d+)$/;


export function clean( text : string ) : string
{
	const sign = text.startsWith( '-' ) ? '-' : '';
	const kept = text.replace( /[^\d.,]/g, '' );
	const index = kept.search( SEPARATORS );

	if( index < 0 ) return sign + kept;

	return sign + kept.slice( 0, index + 1 ) + kept.slice( index + 1 ).replace( /[.,]/g, '' );
}

export function parse( text : string ) : number
{
	if( ! NUMBER.test( text ) ) return NaN;

	return Number( text.replace( ',', '.' ) );
}

export function write( value : number, separator : string ) : string
{
	return value.toLocaleString( 'en', { useGrouping : false, maximumSignificantDigits : 12 } ).replace( '.', separator );
}
