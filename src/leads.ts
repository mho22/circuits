import { terminals } from './terminals.ts';
import type { Branch } from './types/branch.ts';
import type { Element } from './types/element.ts';


export function leads( element : Element ) : Pick<Branch, 'from' | 'to' | 'current'>[]
{
	const [ base, collector, emitter ] = terminals( element.kind, element.from, element.to );

	if( ! emitter ) return [ { from : element.from, to : element.to, current : element.currents[ 0 ] } ];

	return [
		{ from : base, to : element.to, current : element.currents[ 0 ] },
		{ from : collector, to : element.to, current : element.currents[ 1 ] },
		{ from : element.to, to : emitter, current : - element.currents[ 2 ] },
	];
}
