import type { Pin } from './pin.ts';
import type { Shape } from './shape.ts';


export interface Sketch
{
	shapes : Shape[];
	pins : Pin[];
}
