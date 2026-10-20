import { Kind } from '../enums/kind.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as battery from './battery.ts';
import * as ground from './ground.ts';
import * as resistor from './resistor.ts';
import * as wire from './wire.ts';


const RENDERERS : Record<Kind, ( context : CanvasRenderingContext2D, element : Element, scale : Scale ) => void> = {
	[ Kind.Wire ] : wire.draw,
	[ Kind.Resistor ] : resistor.draw,
	[ Kind.Battery ] : battery.draw,
	[ Kind.Ground ] : ground.draw,
};


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	RENDERERS[ element.kind ]( context, element, scale );
}
