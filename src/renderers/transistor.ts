import { color } from '../colors/voltage.ts';
import { Kind } from '../enums/kind.ts';
import { toScreen } from '../grid.ts';
import { terminals } from '../terminals.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as arrow from './arrow.ts';
import * as line from './line.ts';


const BAR = 10;
const SPREAD = 5;
const BODY = '#d0d0d0';


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const [ base, collector, emitter ] = terminals( element.kind, element.from, element.to ).map( point => ( { x : toScreen( point.x ), y : toScreen( point.y ) } ) );
	const barX = toScreen( element.to.x );
	const barY = toScreen( element.to.y );
	const length = Math.hypot( barX - base.x, barY - base.y );
	const alongX = ( barX - base.x ) / length;
	const alongY = ( barY - base.y ) / length;
	const [ baseColor, collectorColor, emitterColor ] = element.potentials.map( potential => color( potential, scale.voltage ) );
	const startX = barX - alongY * SPREAD;
	const startY = barY + alongX * SPREAD;
	const outward = element.kind === Kind.Npn ? 1 : -1;
	const emitterLength = Math.hypot( emitter.x - startX, emitter.y - startY );
	const directionX = ( emitter.x - startX ) / emitterLength * outward;
	const directionY = ( emitter.y - startY ) / emitterLength * outward;
	const middleX = ( startX + emitter.x ) / 2;
	const middleY = ( startY + emitter.y ) / 2;

	context.lineCap = 'butt';

	line.draw( context, baseColor, 2, base.x, base.y, barX, barY );
	line.draw( context, collectorColor, 2, barX + alongY * SPREAD, barY - alongX * SPREAD, collector.x, collector.y );
	line.draw( context, emitterColor, 2, startX, startY, emitter.x, emitter.y );
	line.draw( context, BODY, 3, barX + alongY * BAR, barY - alongX * BAR, barX - alongY * BAR, barY + alongX * BAR );

	arrow.draw( context, emitterColor, middleX, middleY, directionX, directionY );
}
