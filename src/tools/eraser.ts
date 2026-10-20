import type { Circuit } from '../../bindings/web/pkg/web.js';
import type { Point } from '../types/point.ts';


export function erase( circuit : Circuit, point : Point ) : void
{
	const [ id ] = circuit.elementsAt( point.x, point.y );

	if( id === undefined ) return;

	circuit.delete( id );
}
