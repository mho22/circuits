import { Kind } from './enums/kind.ts';
import type { Point } from './types/point.ts';


const TRANSISTORS = [ Kind.Npn, Kind.Pnp ];


export function terminals( kind : Kind, from : Point, to : Point ) : Point[]
{
	if( ! TRANSISTORS.includes( kind ) ) return [ from, to ];

	const dx = Math.sign( to.x - from.x );
	const dy = Math.sign( to.y - from.y );

	return [ from, { x : to.x + dx + dy, y : to.y + dy - dx }, { x : to.x + dx - dy, y : to.y + dy + dx } ];
}
