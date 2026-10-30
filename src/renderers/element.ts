import { Kind } from '../enums/kind.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as battery from './battery.ts';
import * as capacitor from './capacitor.ts';
import * as clock from './clock.ts';
import * as diode from './diode.ts';
import * as ground from './ground.ts';
import * as inductor from './inductor.ts';
import * as led from './led.ts';
import * as mosfet from './mosfet.ts';
import * as pin from './pin.ts';
import * as resistor from './resistor.ts';
import * as supply from './supply.ts';
import * as symbol from './symbol.ts';
import * as slider from './slider.ts';
import * as switcher from './switch.ts';
import * as toggle from './toggle.ts';
import * as transistor from './transistor.ts';
import * as wire from './wire.ts';


const RENDERERS : Record<Kind, ( context : CanvasRenderingContext2D, element : Element, scale : Scale, stride : number ) => void> = {
	[ Kind.Wire ] : wire.draw,
	[ Kind.Resistor ] : resistor.draw,
	[ Kind.Battery ] : battery.draw,
	[ Kind.Ground ] : ground.draw,
	[ Kind.Capacitor ] : capacitor.draw,
	[ Kind.Inductor ] : inductor.draw,
	[ Kind.Switch ] : switcher.draw,
	[ Kind.Diode ] : diode.draw,
	[ Kind.Npn ] : transistor.draw,
	[ Kind.Pnp ] : transistor.draw,
	[ Kind.Nmos ] : mosfet.draw,
	[ Kind.Pmos ] : mosfet.draw,
	[ Kind.Supply ] : supply.draw,
	[ Kind.Input ] : pin.draw,
	[ Kind.Symbol ] : symbol.draw,
	[ Kind.Output ] : pin.draw,
	[ Kind.Toggle ] : toggle.draw,
	[ Kind.Led ] : led.draw,
	[ Kind.Clock ] : clock.draw,
	[ Kind.Slider ] : slider.draw,
};


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale, stride : number ) : void
{
	RENDERERS[ element.kind ]( context, element, scale, stride );
}
