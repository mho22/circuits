import type { Area } from '../types/area.ts';
import type { Point } from '../types/point.ts';


export class SelectTool
{
	private start : Point | null = null;

	private end : Point | null = null;


	press( point : Point ) : void
	{
		this.start = point;
		this.end = point;
	}

	move( point : Point ) : void
	{
		if( this.start ) this.end = point;
	}

	release() : Area | null
	{
		const area = this.preview();

		this.start = null;
		this.end = null;

		if( ! area || area.low.x === area.high.x || area.low.y === area.high.y ) return null;

		return area;
	}

	preview() : Area | null
	{
		const { start, end } = this;

		if( ! start || ! end ) return null;

		return { low : { x : Math.min( start.x, end.x ), y : Math.min( start.y, end.y ) }, high : { x : Math.max( start.x, end.x ), y : Math.max( start.y, end.y ) } };
	}
}
