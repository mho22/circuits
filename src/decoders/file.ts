import { FIELDS, Kind, NAMES, TRANSISTORS } from '../enums/kind.ts';


const VERSION = 1;
const KINDS = Object.keys( NAMES ).map( Number ) as Kind[];


function unreadable() : never
{
	throw new Error( 'This file is not a circuit.' );
}

function parse( text : string ) : Record<string, unknown>
{
	try { return JSON.parse( text ) ?? unreadable(); }

	catch { return unreadable(); }
}

function number( value : unknown ) : number
{
	return typeof value === 'number' ? value : unreadable();
}

function point( value : unknown ) : number[]
{
	if( ! Array.isArray( value ) || value.length !== 2 || ! value.every( Number.isInteger ) ) unreadable();

	return value;
}

function row( part : Record<string, unknown> | null ) : number[]
{
	if( typeof part !== 'object' || part === null ) unreadable();

	const kind = KINDS.find( kind => NAMES[ kind ].toLowerCase() === part.kind ) ?? unreadable();
	const field = FIELDS[ kind ];
	const closed = kind === Kind.Switch && ( typeof part.closed === 'boolean' ? part.closed : unreadable() );
	const mirrored = TRANSISTORS.includes( kind ) && ( typeof ( part.mirrored ?? false ) === 'boolean' ? part.mirrored === true : unreadable() );
	const value = field ? number( part[ field ] ) : Number( closed || mirrored );

	return [ kind, value, ... point( part.from ), ... point( part.to ) ];
}


export function decode( text : string ) : Float64Array
{
	const file = parse( text );
	const version = number( file.version );

	if( version > VERSION ) throw new Error( 'This circuit comes from a newer version of Circuits.' );

	if( version !== VERSION || ! Array.isArray( file.parts ) ) unreadable();

	return new Float64Array( file.parts.flatMap( row ) );
}
