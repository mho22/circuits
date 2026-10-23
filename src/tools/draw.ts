import { arc } from '../arc.ts';
import { Figure } from '../enums/figure.ts';
import { HALF, snap } from '../grid.ts';
import type { Pin } from '../types/pin.ts';
import type { Point } from '../types/point.ts';
import type { Shape } from '../types/shape.ts';
import type { Sketch } from '../types/sketch.ts';


const DOT = 1;


export class DrawTool
{
	figure = Figure.Line;

	shapes : Shape[] = [];

	pins : Pin[] = [];

	private past : Sketch[] = [];

	private undone : Sketch[] = [];

	private start : Point | null = null;

	private cursor : Point | null = null;

	private bend : { from : Point, to : Point } | null = null;


	load( shapes : Shape[], pins : Pin[] ) : void
	{
		this.shapes = shapes;
		this.pins = pins;
		this.past = shapes.map( ( _, index ) => ( { shapes : shapes.slice( 0, index ), pins } ) );
		this.undone = [];
	}

	change( shapes : Shape[], pins : Pin[] ) : void
	{
		this.past.push( { shapes : this.shapes, pins : this.pins } );
		this.undone = [];
		this.shapes = shapes;
		this.pins = pins;
	}

	press( point : Point ) : void
	{
		this.start = snap( point );
		this.cursor = snap( point );
	}

	move( point : Point ) : void
	{
		this.cursor = snap( point );
	}

	release( point : Point ) : Point | null
	{
		const { start } = this;

		this.cursor = snap( point );

		const shape = this.preview();

		this.start = null;

		if( this.figure === Figure.Text ) return start;

		if( ! shape ) return null;

		if( this.figure === Figure.Curve && shape.figure === Figure.Line ) { this.bend = { from : shape.from, to : shape.to }; return null; }

		this.bend = null;
		this.change( [ ... this.shapes, shape ], this.pins );

		return null;
	}

	write( at : Point, content : string ) : void
	{
		const text = content.trim();

		if( ! text ) return;

		this.change( [ ... this.shapes, { figure : Figure.Text, at, content : text } ], this.pins );
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
		const sketch = this.past.pop();

		if( ! sketch ) return;

		this.undone.push( { shapes : this.shapes, pins : this.pins } );
		this.shapes = sketch.shapes;
		this.pins = sketch.pins;
	}

	redo() : void
	{
		const sketch = this.undone.pop();

		if( ! sketch ) return;

		this.past.push( { shapes : this.shapes, pins : this.pins } );
		this.shapes = sketch.shapes;
		this.pins = sketch.pins;
	}

	preview() : Shape | null
	{
		const { start, cursor, bend } = this;

		if( ! cursor ) return null;

		if( bend ) return arc( bend.from, cursor, bend.to ) ? { figure : Figure.Curve, from : bend.from, through : cursor, to : bend.to } : { figure : Figure.Line, ... bend };

		if( ! start || this.figure === Figure.Text ) return null;

		if( this.figure === Figure.Circle )
		{
			const radius = Math.round( Math.hypot( cursor.x - start.x, cursor.y - start.y ) / HALF ) * HALF;

			return { figure : Figure.Circle, center : start, radius : radius || DOT };
		}

		return start.x !== cursor.x || start.y !== cursor.y ? { figure : Figure.Line, from : start, to : cursor } : null;
	}
}
