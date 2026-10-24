import { brightness, DARK, LIT } from '../colors/light.ts';
import type { Color } from '../enums/color.ts';
import { toScreen } from '../grid.ts';
import type { Element } from '../types/element.ts';
import type { Scale } from '../types/scale.ts';
import * as diode from './diode.ts';
import * as line from './line.ts';


const HALO = 14;
const GAP = 10;
const RAY = 5;
const SPREAD = 3;


export function draw( context : CanvasRenderingContext2D, element : Element, scale : Scale ) : void
{
	const fromX = toScreen( element.from.x );
	const fromY = toScreen( element.from.y );
	const toX = toScreen( element.to.x );
	const toY = toScreen( element.to.y );
	const length = Math.hypot( toX - fromX, toY - fromY );
	const alongX = ( toX - fromX ) / length;
	const alongY = ( toY - fromY ) / length;
	const middleX = ( fromX + toX ) / 2;
	const middleY = ( fromY + toY ) / 2;
	const current = element.currents[ 0 ];
	const glow = brightness( current );
	const lit = LIT[ element.value as Color ];

	if( glow > 0 )
	{
		context.fillStyle = `rgba(${ lit }, ${ glow / 2 })`;
		context.beginPath();
		context.arc( middleX, middleY, HALO, 0, 2 * Math.PI );
		context.fill();
	}

	diode.draw( context, element, scale );

	context.lineCap = 'round';

	for( const shift of [ - SPREAD, SPREAD ] )
	{
		const startX = middleX + alongY * GAP + alongX * shift;
		const startY = middleY - alongX * GAP + alongY * shift;

		line.draw( context, glow > 0 ? `rgb(${ lit })` : DARK, 2, startX, startY, startX + ( alongY + alongX ) * RAY, startY + ( alongY - alongX ) * RAY );
	}
}
