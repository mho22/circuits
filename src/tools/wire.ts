import { Circuit } from '../../bindings/web/pkg/web.js';
import type { Point } from '../types/point.ts';


export class WireTool
{
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
		if( ! this.start ) return;

		const [ x, y ] = Circuit.snap( this.start.x, this.start.y, point.x, point.y );

		this.end = { x, y };
	}

	release() : void
	{
		const { start, end } = this;

		this.start = null;
		this.end = null;

		if( ! start || ! end ) return;

		if( start.x === end.x && start.y === end.y ) return;

		this.circuit.addWire( start.x, start.y, end.x, end.y );
	}

	erase( point : Point ) : void
	{
		const id = this.circuit.wireAt( point.x, point.y );

		if( id === undefined ) return;

		this.circuit.deleteWire( id );
	}

	preview() : number[]
	{
		if( ! this.start || ! this.end ) return [];

		return [ this.start.x, this.start.y, this.end.x, this.end.y ];
	}
}
