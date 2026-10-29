import { encode } from '../encoders/drawing.ts';
import { Color, NAMES as COLORS } from '../enums/color.ts';
import { Figure, NAMES as FIGURES } from '../enums/figure.ts';
import { FIELDS, Kind, LEVELS, NAMES, PINS, TRANSISTORS } from '../enums/kind.ts';
import type { Contents } from '../types/contents.ts';
import type { Point } from '../types/point.ts';
import type { Shape } from '../types/shape.ts';


const VERSION = 1;
const KINDS = Object.keys( NAMES ).map( Number ) as Kind[];
const SHAPES = Object.keys( FIGURES ).map( Number ) as Figure[];
const SHADES = Object.keys( COLORS ).map( Number ) as Color[];


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

function text( value : unknown ) : string
{
	return typeof ( value ?? '' ) === 'string' ? ( value ?? '' ) as string : unreadable();
}

function at( value : unknown ) : Point
{
	const [ x, y ] = point( value );

	return { x, y };
}

function sides( from : Point, to : Point ) : Shape[]
{
	const corners = [ from, { x : to.x, y : from.y }, to, { x : from.x, y : to.y } ];

	return corners.map( ( corner, index ) => ( { figure : Figure.Line, from : corner, to : corners[ ( index + 1 ) % 4 ] } ) );
}

function shape( value : Record<string, unknown> | null ) : Shape[]
{
	if( typeof value !== 'object' || value === null ) unreadable();

	if( value.shape === 'rectangle' ) return sides( at( value.from ), at( value.to ) );

	const figure = SHAPES.find( figure => FIGURES[ figure ].toLowerCase() === value.shape ) ?? unreadable();

	if( figure === Figure.Text ) return [ { figure, at : at( value.at ), content : text( value.content ) } ];

	if( figure === Figure.Circle ) return [ { figure, center : at( value.center ), radius : Number.isInteger( value.radius ) ? value.radius as number : unreadable() } ];

	if( figure === Figure.Curve ) return [ { figure, from : at( value.from ), through : at( value.through ), to : at( value.to ) } ];

	return [ { figure, from : at( value.from ), to : at( value.to ) } ];
}

function drawing( value : unknown ) : number[]
{
	const shapes = Array.isArray( value ?? [] ) ? ( value ?? [] ) as Record<string, unknown>[] : unreadable();
	const rows = encode( shapes.flatMap( shape ) );

	return [ rows.length, ... rows ];
}

function layout( value : unknown ) : number[]
{
	const points = Array.isArray( value ?? [] ) ? ( value ?? [] ) as unknown[] : unreadable();

	return [ points.length, ... points.flatMap( point ) ];
}

function rotation( value : unknown ) : number
{
	return typeof value === 'number' && [ 0, 1, 2, 3 ].includes( value ) ? value : unreadable();
}

function level( value : unknown ) : number
{
	return value === 0 || value === 1 ? value : unreadable();
}

function color( value : unknown ) : number
{
	return SHADES.find( shade => COLORS[ shade ].toLowerCase() === value ) ?? unreadable();
}

function row( layer : number, part : Record<string, unknown> | null ) : [ number[], string ]
{
	if( typeof part !== 'object' || part === null ) unreadable();

	const kind = KINDS.find( kind => NAMES[ kind ].toLowerCase() === part.kind ) ?? unreadable();
	const field = FIELDS[ kind ];
	const closed = kind === Kind.Switch && ( typeof part.closed === 'boolean' ? part.closed : unreadable() );
	const mirrored = TRANSISTORS.includes( kind ) && ( typeof ( part.mirrored ?? false ) === 'boolean' ? part.mirrored === true : unreadable() );
	const value = field ? number( part[ field ] ) : LEVELS.includes( kind ) ? level( part.level ?? 0 ) : kind === Kind.Led ? color( part.color ?? 'red' ) : Number( closed || mirrored );
	const turn = kind === Kind.Symbol ? rotation( part.rotation ?? 0 ) : kind === Kind.Clock ? level( part.level ?? 1 ) : 0;

	if( kind === Kind.Symbol && ! ( Number.isInteger( value ) && value > 0 ) ) unreadable();

	return [ [ layer, kind, value, turn, ... point( part.from ), ... point( part.to ) ], PINS.includes( kind ) || kind === Kind.Symbol ? text( part.label ) : '' ];
}

function definition( symbol : Record<string, unknown> | null ) : [ [ number[], string ][], number, string, number[], number[] ]
{
	if( typeof symbol !== 'object' || symbol === null ) unreadable();

	const layer = number( symbol.layer );

	if( ! Number.isInteger( layer ) || layer < 1 || ! Array.isArray( symbol.parts ) ) unreadable();

	return [ symbol.parts.map( part => row( layer, part ) ), layer, text( symbol.name ), drawing( symbol.drawing ), layout( symbol.layout ) ];
}


export function decode( source : string ) : Contents
{
	const file = parse( source );
	const version = number( file.version );
	const symbols = file.symbols ?? [];

	if( version > VERSION ) throw new Error( 'This circuit comes from a newer version of Circuits.' );

	if( version !== VERSION || ! Array.isArray( file.parts ) || ! Array.isArray( symbols ) ) unreadable();

	const definitions = symbols.map( definition );
	const rows = [ ... file.parts.map( part => row( 0, part ) ), ... definitions.flatMap( ( [ parts ] ) => parts ) ];

	return { parts : new Float64Array( rows.flatMap( ( [ numbers ] ) => numbers ) ), labels : rows.map( ( [ , label ] ) => label ), layers : new Uint32Array( definitions.map( ( [ , layer ] ) => layer ) ), names : definitions.map( ( [ , , name ] ) => name ), drawings : new Int32Array( definitions.flatMap( ( [ , , , rows ] ) => rows ) ), layouts : new Int32Array( definitions.flatMap( ( [ , , , , points ] ) => points ) ) };
}
