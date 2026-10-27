import { Kind } from './enums/kind.ts';
import type { Branch } from './types/branch.ts';
import type { Element } from './types/element.ts';
import type { Scale } from './types/scale.ts';


export const FROZEN : Scale = { voltage : NaN, current : 12 };
const LEAK = 1e-6;


function largest( values : number[], start : number ) : number
{
	return values.reduce( ( largest, value ) => Math.abs( value ) > largest ? Math.abs( value ) : largest, start );
}


export function measure( elements : Element[], branches : Branch[] ) : Scale
{
	const peak = elements.reduce( ( peak, element ) => largest( element.potentials, peak ), 0 );
	const voltage = peak < LEAK ? 0 : peak;

	const flowing = elements.reduce( ( peak, element ) => element.kind === Kind.Wire ? peak : largest( element.currents, peak ), 0 );
	const current = largest( branches.map( branch => branch.current ), flowing );

	return { voltage, current };
}
