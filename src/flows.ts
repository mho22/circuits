import { Kind, PINS, POINTS } from './enums/kind.ts';
import { leads } from './leads.ts';
import type { Branch } from './types/branch.ts';
import type { Element } from './types/element.ts';


export function flows( elements : Element[], branches : Branch[], frozen : boolean ) : Pick<Branch, 'from' | 'to' | 'current'>[]
{
	if( frozen ) return elements.filter( item => item.kind === Kind.Wire ).map( wire => ( { from : wire.from, to : wire.to, current : 1 } ) );

	return [ ... branches, ... elements.filter( item => item.kind !== Kind.Wire && ! POINTS.includes( item.kind ) && ! PINS.includes( item.kind ) ).flatMap( leads ) ];
}
