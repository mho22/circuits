const SEPARATOR = '/';


export function split( name : string ) : { path : string[], title : string }
{
	const parts = name.split( SEPARATOR );
	const title = parts.pop()!.trim();

	return { path : parts.map( part => part.trim() ).filter( Boolean ), title };
}
