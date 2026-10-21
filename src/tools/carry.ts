import { FINE, snap } from '../grid.ts';
import { shift } from '../transform.ts';
import type { Pin } from '../types/pin.ts';
import type { Point } from '../types/point.ts';
import type { Shape } from '../types/shape.ts';
import type { Target } from '../types/target.ts';


export class CarryTool
{
	target : Target | null = null;

	private anchor : Point = { x : 0, y : 0 };

	private offset : Point = { x : 0, y : 0 };


	start( target : Target, anchor : Point ) : void
	{
		this.target = target;
		this.anchor = anchor;
		this.offset = { x : 0, y : 0 };
	}

	move( point : Point ) : void
	{
		this.offset = { x : point.x - this.anchor.x, y : point.y - this.anchor.y };
	}

	cancel() : void
	{
		this.target = null;
	}

	shapes( shapes : Shape[] ) : Shape[]
	{
		const { target } = this;

		if( ! target || target.pin ) return shapes;

		return shapes.map( ( shape, index ) => index === target.index ? shift( shape, snap( this.offset ) ) : shape );
	}

	pins( pins : Pin[] ) : Pin[]
	{
		const { target } = this;

		if( ! target || ! target.pin ) return pins;

		const held = pins[ target.index ].point;
		const point = { x : held.x + Math.round( this.offset.x / FINE ), y : held.y + Math.round( this.offset.y / FINE ) };

		if( pins.some( ( pin, index ) => index !== target.index && pin.point.x === point.x && pin.point.y === point.y ) ) return pins;

		return pins.map( ( pin, index ) => index === target.index ? { ... pin, point } : pin );
	}
}
