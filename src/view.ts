import { SPACING, toGrid, toScreen } from './grid.ts';
import type { Point } from './types/point.ts';


export const STRIDES = [ 1, 2, 4, 8 ];

const LINE = 1;
const PAUSE = 200;
const SWIPE = 50;


export class View
{
	x = 0;

	y = 0;

	level = 0;

	private grab : Point | null = null;

	private spun = - Infinity;

	private swept = 0;

	private turned = false;

	private slowed = false;

	private last = 0;

	private entries : { layer : number, x : number, y : number, level : number }[] = [];


	get stride() : number
	{
		return STRIDES[ this.level ];
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

	zoom( level : number, point : Point ) : boolean
	{
		if( level < 0 || level >= STRIDES.length || level === this.level ) return false;

		const ratio = this.stride / STRIDES[ level ];

		this.x = ( point.x + this.x ) * ratio - point.x;
		this.y = ( point.y + this.y ) * ratio - point.y;
		this.level = level;

		return true;
	}

	spin( delta : number, deltaMode : number, time : number, point : Point ) : boolean
	{
		const pixels = deltaMode === LINE ? delta * SPACING : delta;
		const size = Math.abs( pixels );
		const pushed = this.slowed && size > this.last;

		if( time - this.spun > PAUSE || pushed ) { this.swept = 0; this.turned = false; }

		this.slowed = this.turned && ( this.slowed || size < this.last );
		this.spun = time;
		this.last = size;

		if( this.turned ) return false;

		this.swept += pixels;

		if( Math.abs( this.swept ) < SWIPE ) return false;

		this.turned = true;

		return this.zoom( this.level + Math.sign( this.swept ), point );
	}

	center( point : Point ) : void
	{
		this.show( { x : 0, y : 0 }, point );
	}

	enter( layer : number, middle : Point, cursor : Point ) : void
	{
		this.entries.push( { layer, x : this.x, y : this.y, level : this.level } );
		this.show( middle, cursor );
	}

	show( middle : Point, point : Point ) : void
	{
		this.x = toScreen( middle.x ) / this.stride - point.x;
		this.y = toScreen( middle.y ) / this.stride - point.y;
	}

	follow( layer : number ) : void
	{
		while( this.entries.length > 0 && this.entries[ this.entries.length - 1 ].layer !== layer )
		{
			const entry = this.entries.pop()!;

			this.x = entry.x;
			this.y = entry.y;
			this.level = entry.level;
		}
	}

	toScreen( point : Point ) : Point
	{
		return { x : toScreen( point.x ) / this.stride - this.x, y : toScreen( point.y ) / this.stride - this.y };
	}

	toPixels( x : number, y : number ) : Point
	{
		return { x : ( x + this.x ) * this.stride, y : ( y + this.y ) * this.stride };
	}

	toGrid( x : number, y : number ) : Point
	{
		const pixels = this.toPixels( x, y );

		return toGrid( pixels.x, pixels.y );
	}
}
