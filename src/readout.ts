import { Kind, NAMES, UNITS } from './enums/kind.ts';
import { format } from './si.ts';
import type { Branch } from './types/branch.ts';
import type { Element } from './types/element.ts';


function detail( element : Element ) : string | undefined
{
	const unit = UNITS[ element.kind ];

	if( unit ) return format( element.value, unit );

	if( element.kind === Kind.Switch ) return element.value ? 'closed' : 'open';

	return undefined;
}


export function describe( element : Element, branch : Branch | undefined ) : string
{
	const title = [ NAMES[ element.kind ], detail( element ) ].filter( Boolean ).join( ' ' );

	if( element.kind === Kind.Wire ) return [ title, format( element.potentials[ 0 ], 'V' ), format( Math.abs( branch?.current ?? NaN ), 'A' ) ].join( ' · ' );

	if( element.kind === Kind.Ground ) return [ title, format( element.potentials[ 0 ], 'V' ), format( Math.abs( element.current ), 'A' ) ].join( ' · ' );

	const across = Math.abs( element.potentials[ 1 ] - element.potentials[ 0 ] );

	return [ title, `${ format( across, 'V' ) } across`, format( Math.abs( element.current ), 'A' ) ].join( ' · ' );
}
