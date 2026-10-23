export class Trail
{
	private element = document.createElement( 'span' );


	constructor( parent : HTMLElement )
	{
		this.element.className = 'trail';
		this.element.hidden = true;

		parent.append( this.element );
	}


	update( names : string[] ) : void
	{
		if( this.element.textContent === names.join( ' > ' ) ) return;

		const outer = document.createElement( 'span' );

		outer.textContent = names.slice( 0, -1 ).map( name => `${ name } > ` ).join( '' );

		this.element.replaceChildren( outer, names.at( -1 ) ?? '' );
		this.element.hidden = names.length === 0;
		this.element.scrollLeft = this.element.scrollWidth;
	}
}
