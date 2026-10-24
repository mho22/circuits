import { crowded } from '../crowd.ts';
import { FINE } from '../grid.ts';
import { translate } from '../translate.ts';
import type { Area } from '../types/area.ts';
import type { Grab } from '../types/grab.ts';
import type { Pin } from '../types/pin.ts';
import type { Point } from '../types/point.ts';
import type { Shape } from '../types/shape.ts';


export class ShiftTool
{
	grab : Grab | null = null;

	private area : Area | null = null;

	private anchor : Point = { x : 0, y : 0 };

	private offset : Point = { x : 0, y : 0 };


	start( grab : Grab, area : Area, anchor : Point ) : void
	{
		this.grab = grab;
		this.area = area;
		this.anchor = anchor;
		this.offset = { x : 0, y : 0 };
	}

	move( point : Point ) : void
	{
		const step = this.grab && this.grab.pins.length > 0 ? FINE : 1;

		this.offset = { x : Math.round( ( point.x - this.anchor.x ) / step ) * step, y : Math.round( ( point.y - this.anchor.y ) / step ) * step };
	}

	cancel() : void
	{
		this.grab = null;
		this.area = null;
	}

	shapes( shapes : Shape[] ) : Shape[]
	{
		const { grab, offset } = this;

		if( ! grab ) return shapes;

		return shapes.map( ( shape, index ) => grab.shapes.includes( index ) ? translate( shape, offset ) : shape );
	}

	pins( pins : Pin[] ) : Pin[]
	{
		const { grab } = this;

		if( ! grab ) return pins;

		const step = { x : this.offset.x / FINE, y : this.offset.y / FINE };
		const moved = pins.map( ( pin, index ) => grab.pins.includes( index ) ? { ... pin, point : { x : pin.point.x + step.x, y : pin.point.y + step.y } } : pin );
		const taken = grab.pins.some( index => crowded( moved, moved[ index ].point ) );

		return taken ? pins : moved;
	}

	zone() : Area | null
	{
		const { area, offset } = this;

		if( ! area ) return null;

		return { low : { x : area.low.x + offset.x, y : area.low.y + offset.y }, high : { x : area.high.x + offset.x, y : area.high.y + offset.y } };
	}
}
