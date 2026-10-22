import type { Circuit } from '../../bindings/web/pkg/web.js';
import { decode } from '../decoders/element.ts';
import { Kind, LEVELS } from '../enums/kind.ts';
import type { Point } from '../types/point.ts';


export function toggle( circuit : Circuit, point : Point ) : boolean
{
	const elements = decode( circuit.elements() );
	const ids = Array.from( circuit.elementsAt( point.x, point.y ) );
	const target = elements.find( item => item.kind === Kind.Switch && ids.includes( item.id ) );
	const source = elements.find( item => LEVELS.includes( item.kind ) && ids.includes( item.id ) );

	if( target ) { circuit.setClosed( target.id, target.value !== 1 ); return true; }

	if( ! source || circuit.listening().includes( source.id ) ) return false;

	circuit.setHigh( source.id, source.value !== 1 );

	return true;
}
