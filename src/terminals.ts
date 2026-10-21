import { Kind, PINS, TRANSISTORS } from './enums/kind.ts';
import type { Point } from './types/point.ts';


export function terminals( kind : Kind, from : Point, to : Point, mirrored : boolean ) : Point[]
{
	if( PINS.includes( kind ) || kind === Kind.Toggle ) return [ to, to ];

	if( ! TRANSISTORS.includes( kind ) ) return [ from, to ];

	const dx = Math.sign( to.x - from.x );
	const dy = Math.sign( to.y - from.y );

	const left = { x : to.x + dx + dy, y : to.y + dy - dx };
	const right = { x : to.x + dx - dy, y : to.y + dy + dx };

	return mirrored ? [ from, right, left ] : [ from, left, right ];
}
