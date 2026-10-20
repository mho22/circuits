import { channel } from './channel.ts';
import { Kind } from './enums/kind.ts';
import { terminals } from './terminals.ts';
import type { Branch } from './types/branch.ts';
import type { Element } from './types/element.ts';


const MOSFETS = [ Kind.Nmos, Kind.Pmos ];


export function leads( element : Element ) : Pick<Branch, 'from' | 'to' | 'current'>[]
{
	const [ base, collector, emitter ] = terminals( element.kind, element.from, element.to, element.mirrored );

	if( ! emitter ) return [ { from : element.from, to : element.to, current : element.currents[ 0 ] } ];

	if( MOSFETS.includes( element.kind ) )
	{
		const [ top, bottom ] = channel( element.from, element.to, element.mirrored );

		return [
			{ from : collector, to : top, current : element.currents[ 1 ] },
			{ from : top, to : bottom, current : element.currents[ 1 ] },
			{ from : bottom, to : emitter, current : - element.currents[ 2 ] },
		];
	}

	return [
		{ from : base, to : element.to, current : element.currents[ 0 ] },
		{ from : collector, to : element.to, current : element.currents[ 1 ] },
		{ from : element.to, to : emitter, current : - element.currents[ 2 ] },
	];
}
