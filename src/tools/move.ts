import type { Circuit } from '../../bindings/web/pkg/web.js';
import type { Element } from '../types/element.ts';
import type { Point } from '../types/point.ts';


export class MoveTool
{
	element : Element | null = null;

	private anchor : Point = { x : 0, y : 0 };

	private offset : Point = { x : 0, y : 0 };


	constructor( private circuit : Circuit ) {}


	start( element : Element, anchor : Point ) : void
	{
		this.element = element;
		this.anchor = anchor;
		this.offset = { x : 0, y : 0 };
	}

	move( point : Point ) : void
	{
		this.offset = { x : point.x - this.anchor.x, y : point.y - this.anchor.y };
	}

	drop() : void
	{
		const { element, offset } = this;

		this.element = null;

		if( ! element || ( offset.x === 0 && offset.y === 0 ) ) return;

		this.circuit.translate( element.id, offset.x, offset.y );
	}

	cancel() : void
	{
		this.element = null;
	}

	preview() : Element | null
	{
		if( ! this.element ) return null;

		const shift = ( point : Point ) => ( { x : point.x + this.offset.x, y : point.y + this.offset.y } );

		return { ... this.element, from : shift( this.element.from ), to : shift( this.element.to ), potentials : this.element.potentials.map( () => 0 ) };
	}
}
