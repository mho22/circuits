import { bit } from './bit.ts';
import { Kind, NAMES, PINS, UNITS } from './enums/kind.ts';
import { format } from './si.ts';
import type { Branch } from './types/branch.ts';
import type { Element } from './types/element.ts';


const MOSFETS = [ Kind.Nmos, Kind.Pmos ];


function detail( element : Element ) : string | undefined
{
	const unit = UNITS[ element.kind ];

	if( unit ) return format( element.value, unit );

	if( element.kind === Kind.Switch ) return element.value ? 'closed' : 'open';

	if( element.kind === Kind.Symbol ) return String( element.value );

	if( PINS.includes( element.kind ) ) return element.label;

	return undefined;
}


export function describe( element : Element, branch : Branch | undefined ) : string
{
	const title = element.definition?.name || [ NAMES[ element.kind ], detail( element ) ].filter( Boolean ).join( ' ' );

	if( element.kind === Kind.Wire ) return [ title, format( element.potentials[ 0 ], 'V' ), format( Math.abs( branch?.current ?? NaN ), 'A' ) ].join( ' · ' );

	if( element.kind === Kind.Symbol )
	{
		const delay = element.definition?.delay ?? NaN;

		return [ title, Number.isFinite( delay ) ? format( delay, 's' ) : '', ... element.potentials.map( potential => format( potential, 'V' ) ) ].filter( Boolean ).join( ' · ' );
	}

	if( PINS.includes( element.kind ) ) return [ title, String( bit( element.potentials[ 0 ] ) ), format( element.potentials[ 0 ], 'V' ) ].join( ' · ' );

	if( element.kind === Kind.Toggle ) return [ title, String( element.value ), format( element.potentials[ 1 ], 'V' ), format( Math.abs( element.currents[ 0 ] ), 'A' ) ].join( ' · ' );

	if( element.kind === Kind.Clock ) return [ title, String( bit( element.potentials[ 1 ] ) ), format( element.potentials[ 1 ], 'V' ), format( Math.abs( element.currents[ 0 ] ), 'A' ) ].join( ' · ' );

	if( element.kind === Kind.Supply ) return [ title, format( Math.abs( element.currents[ 0 ] ), 'A' ) ].join( ' · ' );

	if( element.kind === Kind.Ground ) return [ title, format( element.potentials[ 0 ], 'V' ), format( Math.abs( element.currents[ 0 ] ), 'A' ) ].join( ' · ' );

	if( MOSFETS.includes( element.kind ) )
	{
		const [ gate, drain, source ] = element.potentials;

		return [ title, `Vgs ${ format( Math.abs( gate - source ), 'V' ) }`, `Vds ${ format( Math.abs( drain - source ), 'V' ) }`, `Id ${ format( Math.abs( element.currents[ 1 ] ), 'A' ) }` ].join( ' · ' );
	}

	if( element.potentials.length === 3 )
	{
		const [ base, collector, emitter ] = element.potentials;

		return [ title, `Vbe ${ format( Math.abs( base - emitter ), 'V' ) }`, `Vce ${ format( Math.abs( collector - emitter ), 'V' ) }`, `Ib ${ format( Math.abs( element.currents[ 0 ] ), 'A' ) }`, `Ic ${ format( Math.abs( element.currents[ 1 ] ), 'A' ) }` ].join( ' · ' );
	}

	const across = Math.abs( element.potentials[ 1 ] - element.potentials[ 0 ] );

	return [ title, `${ format( across, 'V' ) } across`, format( Math.abs( element.currents[ 0 ] ), 'A' ) ].join( ' · ' );
}
