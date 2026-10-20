import { Kind } from './enums/kind.ts';
import type { Branch } from './types/branch.ts';
import type { Element } from './types/element.ts';
import type { Scale } from './types/scale.ts';


function largest( values : number[] ) : number
{
	return values.reduce( ( largest, value ) => Math.abs( value ) > largest ? Math.abs( value ) : largest, 0 );
}


export function measure( elements : Element[], branches : Branch[] ) : Scale
{
	const voltage = largest( elements.flatMap( element => element.potentials ) );

	const current = largest( [ ... elements.filter( element => element.kind !== Kind.Wire ).flatMap( element => element.currents ), ... branches.map( branch => branch.current ) ] );

	return { voltage, current };
}
