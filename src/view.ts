import { SPACING, toGrid } from './grid.ts';
import type { Point } from './types/point.ts';


const LINE = 1;


export class View
{
	x = 0;

	y = 0;

	private grab : Point | null = null;


	scroll( deltaX : number, deltaY : number, deltaMode : number ) : void
	{
		const factor = deltaMode === LINE ? SPACING : 1;

		this.x += deltaX * factor;
		this.y += deltaY * factor;
	}

	press( point : Point ) : void
	{
		this.grab = point;
	}

	move( point : Point ) : boolean
	{
		if( ! this.grab ) return false;

		this.x -= point.x - this.grab.x;
		this.y -= point.y - this.grab.y;
		this.grab = point;

		return true;
	}

	release() : void
	{
		this.grab = null;
	}

	center() : void
	{
		this.x = 0;
		this.y = 0;
	}

	toGrid( x : number, y : number ) : Point
	{
		return toGrid( x + this.x, y + this.y );
	}
}
