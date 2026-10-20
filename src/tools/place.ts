import { Circuit } from '../../bindings/web/pkg/web.js';
import { decode } from '../decoders/element.ts';
import { Kind, POINTS } from '../enums/kind.ts';
import { terminals } from '../terminals.ts';
import type { Element } from '../types/element.ts';
import type { Point } from '../types/point.ts';


const VALUES : Partial<Record<Kind, number>> = { [ Kind.Resistor ] : 1000, [ Kind.Battery ] : 5, [ Kind.Capacitor ] : 1e-5, [ Kind.Inductor ] : 1, [ Kind.Supply ] : 5 };


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
		if( ! this.start || POINTS.includes( this.kind ) ) return;

		const [ x, y ] = Circuit.snap( this.start.x, this.start.y, point.x, point.y );

		this.end = { x, y };
	}

	release() : void
	{
		const { start, end } = this;

		this.start = null;
		this.end = null;

		if( ! start || ! end ) return;

		const point = POINTS.includes( this.kind );

		if( ! point && start.x === end.x && start.y === end.y ) return;

		if( point && decode( this.circuit.elements() ).some( item => item.kind === this.kind && item.from.x === start.x && item.from.y === start.y ) ) return;

		this.circuit.add( this.kind, VALUES[ this.kind ] ?? 0, start.x, start.y, end.x, end.y );
	}

	preview() : Element | null
	{
		if( ! this.start || ! this.end ) return null;

		const count = terminals( this.kind, this.start, this.end, false ).length;

		return { id : -1, kind : this.kind, from : this.start, to : this.end, value : VALUES[ this.kind ] ?? 0, mirrored : false, potentials : Array( count ).fill( 0 ), currents : Array( count ).fill( NaN ) };
	}
}
