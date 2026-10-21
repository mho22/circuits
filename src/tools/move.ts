import type { Circuit } from '../../bindings/web/pkg/web.js';
import type { Area } from '../types/area.ts';
import type { Element } from '../types/element.ts';
import type { Point } from '../types/point.ts';


export class MoveTool
{
	elements : Element[] = [];

	private area : Area | null = null;

	private anchor : Point = { x : 0, y : 0 };

	private offset : Point = { x : 0, y : 0 };


	constructor( private circuit : Circuit ) {}


	start( elements : Element[], anchor : Point, area : Area | null ) : void
	{
		this.elements = elements;
		this.area = area;
		this.anchor = anchor;
		this.offset = { x : 0, y : 0 };
	}

	move( point : Point ) : void
	{
		this.offset = { x : point.x - this.anchor.x, y : point.y - this.anchor.y };
	}

	drop() : void
	{
		const { elements, area, offset } = this;

		this.cancel();

		if( elements.length === 0 || ( offset.x === 0 && offset.y === 0 ) ) return;

		if( area ) { this.circuit.shift( area.low.x, area.low.y, area.high.x, area.high.y, offset.x, offset.y ); return; }

		this.circuit.translate( elements[ 0 ].id, offset.x, offset.y );
	}

	cancel() : void
	{
		this.elements = [];
		this.area = null;
	}

	preview() : Element[]
	{
		return this.elements.map( element => ( { ... element, from : this.shift( element.from ), to : this.shift( element.to ), potentials : element.potentials.map( () => 0 ) } ) );
	}

	zone() : Area | null
	{
		if( ! this.area ) return null;

		return { low : this.shift( this.area.low ), high : this.shift( this.area.high ) };
	}

	private shift( point : Point ) : Point
	{
		return { x : point.x + this.offset.x, y : point.y + this.offset.y };
	}
}
