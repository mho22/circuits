import { Kind } from '../enums/kind.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as battery from './battery.ts';
import * as capacitor from './capacitor.ts';
import * as diode from './diode.ts';
import * as ground from './ground.ts';
import * as inductor from './inductor.ts';
import * as mosfet from './mosfet.ts';
import * as resistor from './resistor.ts';
import * as toggle from './switch.ts';
import * as transistor from './transistor.ts';
import * as wire from './wire.ts';


const RENDERERS : Record<Kind, ( context : CanvasRenderingContext2D, element : Element, scale : Scale ) => void> = {
	[ Kind.Wire ] : wire.draw,
	[ Kind.Resistor ] : resistor.draw,
	[ Kind.Battery ] : battery.draw,
	[ Kind.Ground ] : ground.draw,
	[ Kind.Capacitor ] : capacitor.draw,
	[ Kind.Inductor ] : inductor.draw,
	[ Kind.Switch ] : toggle.draw,
	[ Kind.Diode ] : diode.draw,
	[ Kind.Npn ] : transistor.draw,
	[ Kind.Pnp ] : transistor.draw,
	[ Kind.Nmos ] : mosfet.draw,
	[ Kind.Pmos ] : mosfet.draw,
};


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	RENDERERS[ element.kind ]( context, element, scale );
}
