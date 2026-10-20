import { Kind } from '../../src/enums/kind.ts';
import type { Element } from '../../src/types/element.ts';


export function element( overrides : Partial<Element> = {} ) : Element
{
	return { id : 0, kind : Kind.Wire, from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, value : 0, potentials : [ 0, 0 ], currents : [ NaN, NaN ], ... overrides };
}
