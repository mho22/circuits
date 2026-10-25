import type { Circuit } from '../../bindings/web/pkg/web.js';
import { Kind } from '../enums/kind.ts';
import { covers, level } from '../slider.ts';
import type { Element } from '../types/element.ts';
import type { Point } from '../types/point.ts';


export class SlideTool
{
	private slider : Element | null = null;

	private amending = false;


	constructor( private circuit : Circuit ) {}


	press( elements : Element[], point : Point ) : boolean
	{
		this.slider = elements.find( item => item.kind === Kind.Slider && ! item.listening && covers( item.from, point ) ) ?? null;
		this.amending = false;

		return this.move( point );
	}

	move( point : Point ) : boolean
	{
		if( ! this.slider ) return false;

		const volts = level( this.slider.from, point.x );

		if( volts === this.slider.value ) return true;

		if( this.amending ) this.circuit.amendValue( this.slider.id, volts );

		if( ! this.amending ) this.circuit.setValue( this.slider.id, volts );

		this.slider = { ... this.slider, value : volts };
		this.amending = true;

		return true;
	}

	release() : boolean
	{
		const sliding = this.slider !== null;

		this.slider = null;

		return sliding;
	}
}
