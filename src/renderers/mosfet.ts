import { channel } from '../channel.ts';
import { color } from '../colors/voltage.ts';
import { Kind } from '../enums/kind.ts';
import { toScreen } from '../grid.ts';
import { terminals } from '../terminals.ts';
import type { Element } from '../types/element.ts';
import type { Point } from '../types/point.ts';
import type { Scale } from '../types/scale.ts';
import * as arrow from './arrow.ts';
import * as line from './line.ts';


const GAP = 6;
const BODY = '#d0d0d0';


function screen( point : Point ) : Point
{
	return { x : toScreen( point.x ), y : toScreen( point.y ) };
}


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const [ gate, drain, source ] = terminals( element.kind, element.from, element.to ).map( screen );
	const [ top, bottom ] = channel( element.from, element.to ).map( screen );
	const bar = screen( element.to );
	const length = Math.hypot( bar.x - gate.x, bar.y - gate.y );
	const alongX = ( bar.x - gate.x ) / length;
	const alongY = ( bar.y - gate.y ) / length;
	const [ gateColor, drainColor, sourceColor ] = element.potentials.map( potential => color( potential, scale.voltage ) );
	const outward = element.kind === Kind.Nmos ? 1 : -1;

	context.lineCap = 'butt';

	line.draw( context, gateColor, 2, gate.x, gate.y, bar.x - alongX * GAP, bar.y - alongY * GAP );
	line.draw( context, gateColor, 2, top.x - alongX * GAP, top.y - alongY * GAP, bottom.x - alongX * GAP, bottom.y - alongY * GAP );
	line.draw( context, drainColor, 2, top.x, top.y, drain.x, drain.y );
	line.draw( context, sourceColor, 2, bottom.x, bottom.y, source.x, source.y );
	line.draw( context, BODY, 3, top.x, top.y, bottom.x, bottom.y );

	arrow.draw( context, sourceColor, ( bottom.x + source.x ) / 2, ( bottom.y + source.y ) / 2, alongX * outward, alongY * outward );
}
