import { Circuit } from '../../bindings/web/pkg/web.js';
import { decode } from '../decoders/element.ts';
import { decode as outline } from '../decoders/outline.ts';
import { Kind, POINTS } from '../enums/kind.ts';
import { pins } from '../pins.ts';
import { terminals } from '../terminals.ts';
import type { Element } from '../types/element.ts';
import type { Point } from '../types/point.ts';


const VALUES : Partial<Record<Kind, number>> = { [ Kind.Resistor ] : 1000, [ Kind.Battery ] : 5, [ Kind.Capacitor ] : 1e-5, [ Kind.Inductor ] : 1, [ Kind.Supply ] : 5 };


export class PlaceTool
{
	kind : Kind | null = Kind.Wire;

	layer = 0;

	start : Point | null = null;

	end : Point | null = null;


	constructor( private circuit : Circuit ) {}


	press( point : Point ) : void
	{
		if( this.kind === null ) return;

		this.start = point;
		this.end = point;
	}

	move( point : Point ) : void
	{
		if( ! this.start || this.kind === null || POINTS.includes( this.kind ) ) return;

		const [ x, y ] = Circuit.snap( this.start.x, this.start.y, point.x, point.y );

		this.end = { x, y };
	}

	release() : boolean
	{
		const { start, end, kind } = this;

		this.start = null;
		this.end = null;

		if( ! start || ! end || kind === null ) return false;

		const point = POINTS.includes( kind );

		if( ! point && start.x === end.x && start.y === end.y ) return false;

		if( point && decode( this.circuit.elements() ).some( item => item.kind === kind && item.from.x === start.x && item.from.y === start.y ) ) return false;

		if( point && this.covered( start ) ) return false;

		this.circuit.add( kind, this.value( kind ), start.x, start.y, end.x, end.y );

		return true;
	}

	preview() : Element | null
	{
		const { start, end, kind } = this;

		if( ! start || ! end || kind === null ) return null;

		const count = terminals( kind, start, end, false ).length;

		return { id : -1, kind, from : start, to : end, value : this.value( kind ), mirrored : false, rotation : 0, potentials : Array( count ).fill( 0 ), currents : Array( count ).fill( NaN ) };
	}

	private covered( point : Point ) : boolean
	{
		const ids = Array.from( this.circuit.elementsAt( point.x, point.y ) );
		const symbols = decode( this.circuit.elements() ).filter( item => item.kind === Kind.Symbol && ids.includes( item.id ) );

		return symbols.some( item => ! pins( item.from, outline( this.circuit.outline( item.value ) ), item.rotation ).some( pin => pin.x === point.x && pin.y === point.y ) );
	}

	private value( kind : Kind ) : number
	{
		return kind === Kind.Symbol ? this.layer : VALUES[ kind ] ?? 0;
	}
}
