import type { Branch } from '../types/branch.ts';


const STRIDE = 6;


export function decode( data : Float64Array ) : Branch[]
{
	const branches : Branch[] = [];

	for( let index = 0; index < data.length; index += STRIDE )
	{
		const [ fromX, fromY, toX, toY, voltage, current ] = data.subarray( index, index + STRIDE );

		branches.push( { from : { x : fromX, y : fromY }, to : { x : toX, y : toY }, voltage, current } );
	}

	return branches;
}
