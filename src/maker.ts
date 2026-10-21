import { parse, SEPARATOR, write } from './decimal.ts';
import { NANO } from './si.ts';
import type { Area } from './types/area.ts';
import type { Definition } from './types/definition.ts';
import type { Point } from './types/point.ts';
import type { Shape } from './types/shape.ts';


export class Maker
{
	area : Area | null = null;

	layer : number | null = null;

	drawing : Shape[] = [];

	layout : Point[] = [];

	steps = 0;

	private undone = 0;

	private bar = document.createElement( 'div' );

	private name = document.createElement( 'input' );

	private delay = document.createElement( 'input' );

	private delete = document.createElement( 'button' );


	constructor( parent : HTMLElement, picker : HTMLElement, finish : ( target : Area | number, name : string, drawing : Shape[], layout : Point[], delay : number ) => void, draw : () => void, private discard : ( steps : number, layer : number | null ) => void, remove : ( layer : number ) => void )
	{
		const unit = document.createElement( 'span' );
		const icon = document.createElement( 'button' );
		const done = document.createElement( 'button' );
		const cancel = document.createElement( 'button' );

		this.bar.className = 'maker';
		this.bar.hidden = true;

		this.name.placeholder = 'Name';

		this.delay.className = 'delay';
		this.delay.placeholder = 'Delay';
		this.delay.setAttribute( 'aria-label', 'Delay of each gate' );

		unit.textContent = 'ns';

		icon.textContent = 'Icon';
		icon.addEventListener( 'click', draw );

		this.delete.textContent = 'Delete';
		this.delete.addEventListener( 'click', () => { if( this.layer !== null ) remove( this.layer ); } );

		done.textContent = 'Done';
		done.addEventListener( 'click', () =>
		{
			const target = this.area ?? this.layer;

			if( target === null ) return;

			const drawing = this.drawing;
			const layout = this.layout;
			const delay = parse( this.delay.value.trim() ) * NANO;

			this.close();

			finish( target, this.name.value.trim(), drawing, layout, delay );
		} );

		cancel.textContent = 'Cancel';
		cancel.addEventListener( 'click', () => this.cancel() );

		this.bar.append( this.name, picker, this.delay, unit, icon, this.delete, done, cancel );

		parent.append( this.bar );
	}


	open( area : Area ) : void
	{
		this.show( null, '', [], [], NANO );

		this.area = area;
		this.delete.hidden = true;

		this.name.focus();
	}

	edit( layer : number, definition : Definition ) : void
	{
		this.show( layer, definition.name, definition.drawing, definition.layout, definition.delay );

		this.delete.hidden = false;
	}

	cancel() : void
	{
		if( ! this.area && this.layer === null ) return;

		const { steps, layer } = this;

		this.close();
		this.discard( steps, layer );
	}

	record() : void
	{
		this.steps++;
		this.undone = 0;
	}

	undo() : boolean
	{
		if( this.steps === 0 ) return false;

		this.steps--;
		this.undone++;

		return true;
	}

	redo() : boolean
	{
		if( this.undone === 0 ) return false;

		this.undone--;
		this.steps++;

		return true;
	}

	close() : void
	{
		this.area = null;
		this.layer = null;
		this.drawing = [];
		this.layout = [];
		this.steps = 0;
		this.undone = 0;
		this.bar.hidden = true;
	}

	private show( layer : number | null, name : string, drawing : Shape[], layout : Point[], delay : number ) : void
	{
		this.area = null;
		this.layer = layer;
		this.drawing = drawing;
		this.layout = layout;
		this.steps = 0;
		this.undone = 0;
		this.name.value = name;
		this.delay.value = write( delay / NANO, SEPARATOR );
		this.bar.hidden = false;
	}
}
