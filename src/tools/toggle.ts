import type { Circuit } from '../../bindings/web/pkg/web.js';
import { decode } from '../decoders/element.ts';
import { Kind } from '../enums/kind.ts';
import type { Point } from '../types/point.ts';


export function toggle( circuit : Circuit, point : Point ) : void
{
	const elements = decode( circuit.elements() );
	const ids = Array.from( circuit.elementsAt( point.x, point.y ) );
	const target = elements.find( item => item.kind === Kind.Switch && ids.includes( item.id ) );

	if( ! target ) return;

	circuit.setClosed( target.id, target.value !== 1 );
}
