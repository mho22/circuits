import { Circuit } from '../../bindings/web/pkg/web.js';
import { Kind } from '../enums/kind.ts';
import type { Element } from '../types/element.ts';
import type { Point } from '../types/point.ts';


const VALUES : Partial<Record<Kind, number>> = { [ Kind.Resistor ] : 1000, [ Kind.Battery ] : 5 };


export class PlaceTool
{
	kind = Kind.Wire;

	start : Point | null = null;

	end : Point | null = null;


	constructor( private circuit : Circuit ) {}


	press( point : Point ) : void
	{
		this.start = point;
		this.end = point;
	}

	move( point : Point ) : void
	{
		if( ! this.start || this.kind === Kind.Ground ) return;

		const [ x, y ] = Circuit.snap( this.start.x, this.start.y, point.x, point.y );

		this.end = { x, y };
	}

	release() : void
	{
		const { start, end } = this;

		this.start = null;
		this.end = null;

		if( ! start || ! end ) return;

		if( this.kind !== Kind.Ground && start.x === end.x && start.y === end.y ) return;

		this.circuit.add( this.kind, VALUES[ this.kind ] ?? 0, start.x, start.y, end.x, end.y );
	}

	preview() : Element | null
	{
		if( ! this.start || ! this.end ) return null;

		return { id : -1, kind : this.kind, from : this.start, to : this.end, value : VALUES[ this.kind ] ?? 0, potentials : [ 0, 0 ], current : NaN };
	}
}
