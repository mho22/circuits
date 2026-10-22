import { Figure, NAMES } from './enums/figure.ts';
import { FIGURES as KEYS } from './keyboard.ts';


const HINTS : Record<Figure, string> = {
	[ Figure.Circle ] : 'Press on the center and drag to the radius, or click for a dot.',
	[ Figure.Line ] : 'Drag from one end to the other.',
	[ Figure.Curve ] : 'Drag between the ends, then click a dot on the curve.',
};
const MENU = 'Right-click a shape to delete it, or a pin to move it. Right-drag to select.';
const FIGURES = [ Figure.Line, Figure.Circle, Figure.Curve ];


export class Palette
{
	private group = document.createElement( 'div' );

	private bar = document.createElement( 'div' );

	private hint = document.createElement( 'span' );

	private buttons : HTMLButtonElement[];


	constructor( nav : HTMLElement, parent : HTMLElement, pick : ( figure : Figure ) => void, done : () => void )
	{
		const finish = document.createElement( 'button' );

		this.buttons = FIGURES.map( figure =>
		{
			const button = document.createElement( 'button' );
			const shortcut = document.createElement( 'kbd' );

			shortcut.textContent = Object.keys( KEYS ).find( key => KEYS[ key ] === figure )!.toUpperCase();

			button.append( NAMES[ figure ], shortcut );
			button.addEventListener( 'click', () => pick( figure ) );

			return button;
		} );

		this.group.className = 'palette';
		this.group.hidden = true;
		this.group.append( ... this.buttons );

		finish.textContent = 'Done';
		finish.addEventListener( 'click', done );

		this.bar.className = 'maker sketch';
		this.bar.hidden = true;
		this.bar.append( this.hint, finish );

		nav.append( this.group );
		parent.append( this.bar );
	}


	open() : void
	{
		this.show( true );
	}

	close() : void
	{
		this.show( false );
	}

	select( figure : Figure ) : void
	{
		this.buttons.forEach( ( button, index ) => button.classList.toggle( 'active', FIGURES[ index ] === figure ) );

		this.hint.textContent = `${ HINTS[ figure ] } ${ MENU }`;
	}

	private show( shown : boolean ) : void
	{
		this.group.hidden = ! shown;
		this.bar.hidden = ! shown;

		document.body.classList.toggle( 'sketching', shown );
	}
}
