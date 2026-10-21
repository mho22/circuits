import { turn } from './turn.ts';
import type { Outline } from './types/outline.ts';
import type { Point } from './types/point.ts';


export function pins( center : Point, outline : Outline, rotation : number ) : Point[]
{
	return outline.pins.map( pin => turn( pin, rotation ) ).map( pin => ( { x : center.x + pin.x, y : center.y + pin.y } ) );
}
