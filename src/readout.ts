import { Kind, UNITS } from './enums/kind.ts';
import { format } from './si.ts';
import type { Branch } from './types/branch.ts';
import type { Element } from './types/element.ts';


const NAMES : Record<Kind, string> = { [ Kind.Wire ] : 'Wire', [ Kind.Resistor ] : 'Resistor', [ Kind.Battery ] : 'Battery', [ Kind.Ground ] : 'Ground' };


export function describe( element : Element, branch : Branch | undefined ) : string
{
	const unit = UNITS[ element.kind ];
	const title = unit ? `${ NAMES[ element.kind ] } ${ format( element.value, unit ) }` : NAMES[ element.kind ];

	if( element.kind === Kind.Wire ) return [ title, format( element.potentials[ 0 ], 'V' ), format( Math.abs( branch?.current ?? NaN ), 'A' ) ].join( ' · ' );

	if( element.kind === Kind.Ground ) return [ title, format( element.potentials[ 0 ], 'V' ), format( Math.abs( element.current ), 'A' ) ].join( ' · ' );

	const across = Math.abs( element.potentials[ 1 ] - element.potentials[ 0 ] );

	return [ title, `${ format( across, 'V' ) } across`, format( Math.abs( element.current ), 'A' ) ].join( ' · ' );
}
