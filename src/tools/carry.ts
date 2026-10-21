import { FINE } from '../grid.ts';
import type { Pin } from '../types/pin.ts';
import type { Point } from '../types/point.ts';


export class CarryTool
{
	index : number | null = null;

	private anchor : Point = { x : 0, y : 0 };

	private offset : Point = { x : 0, y : 0 };


	start( index : number, anchor : Point ) : void
	{
		this.index = index;
		this.anchor = anchor;
		this.offset = { x : 0, y : 0 };
	}

	move( point : Point ) : void
	{
		this.offset = { x : point.x - this.anchor.x, y : point.y - this.anchor.y };
	}

	cancel() : void
	{
		this.index = null;
	}

	pins( pins : Pin[] ) : Pin[]
	{
		const { index } = this;

		if( index === null ) return pins;

		const held = pins[ index ].point;
		const point = { x : held.x + Math.round( this.offset.x / FINE ), y : held.y + Math.round( this.offset.y / FINE ) };

		if( pins.some( ( pin, other ) => other !== index && pin.point.x === point.x && pin.point.y === point.y ) ) return pins;

		return pins.map( ( pin, other ) => other === index ? { ... pin, point } : pin );
	}
}
