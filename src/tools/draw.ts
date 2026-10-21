import { arc } from '../arc.ts';
import { Figure } from '../enums/figure.ts';
import { HALF, snap } from '../grid.ts';
import type { Point } from '../types/point.ts';
import type { Shape } from '../types/shape.ts';


const DOT = 1;


export class DrawTool
{
	figure = Figure.Circle;

	shapes : Shape[] = [];

	private start : Point | null = null;

	private cursor : Point | null = null;

	private bend : { from : Point, to : Point } | null = null;


	press( point : Point ) : void
	{
		this.start = snap( point );
		this.cursor = snap( point );
	}

	move( point : Point ) : void
	{
		this.cursor = snap( point );
	}

	release( point : Point ) : void
	{
		this.cursor = snap( point );

		const shape = this.preview();

		this.start = null;

		if( ! shape ) return;

		if( this.figure === Figure.Curve && shape.figure === Figure.Line ) { this.bend = { from : shape.from, to : shape.to }; return; }

		this.bend = null;
		this.shapes.push( shape );
	}

	cancel() : boolean
	{
		const busy = this.start !== null || this.bend !== null;

		this.start = null;
		this.bend = null;

		return busy;
	}

	undo() : void
	{
		this.shapes.pop();
	}

	preview() : Shape | null
	{
		const { start, cursor, bend } = this;

		if( ! cursor ) return null;

		if( bend ) return arc( bend.from, cursor, bend.to ) ? { figure : Figure.Curve, from : bend.from, through : cursor, to : bend.to } : { figure : Figure.Line, ... bend };

		if( ! start ) return null;

		if( this.figure === Figure.Circle )
		{
			const radius = Math.round( Math.hypot( cursor.x - start.x, cursor.y - start.y ) / HALF ) * HALF;

			return { figure : Figure.Circle, center : start, radius : radius || DOT };
		}

		return start.x !== cursor.x || start.y !== cursor.y ? { figure : Figure.Line, from : start, to : cursor } : null;
	}
}
