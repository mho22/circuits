import { SPACING, toGrid, toScreen } from './grid.ts';
import type { Point } from './types/point.ts';


const LINE = 1;


export class View
{
	x = 0;

	y = 0;

	private grab : Point | null = null;

	private entries : { layer : number, x : number, y : number }[] = [];


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

	center( point : Point ) : void
	{
		this.show( { x : 0, y : 0 }, point );
	}

	enter( layer : number, middle : Point, cursor : Point ) : void
	{
		this.entries.push( { layer, x : this.x, y : this.y } );
		this.show( middle, cursor );
	}

	show( middle : Point, point : Point ) : void
	{
		this.x = toScreen( middle.x ) - point.x;
		this.y = toScreen( middle.y ) - point.y;
	}

	follow( layer : number ) : void
	{
		while( this.entries.length > 0 && this.entries[ this.entries.length - 1 ].layer !== layer )
		{
			const entry = this.entries.pop()!;

			this.x = entry.x;
			this.y = entry.y;
		}
	}

	toGrid( x : number, y : number ) : Point
	{
		return toGrid( x + this.x, y + this.y );
	}
}
