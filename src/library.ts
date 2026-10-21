export class Library
{
	private group = document.createElement( 'div' );

	private main = document.createElement( 'button' );

	private menu = document.createElement( 'div' );

	private symbols : [ number, string ][] = [];

	private shown : number | null = null;


	constructor( parent : HTMLElement, private pick : ( layer : number ) => void )
	{
		const open = document.createElement( 'button' );

		this.group.className = 'group';
		this.group.hidden = true;
		this.main.className = 'pick';
		this.main.addEventListener( 'click', () => { if( this.shown !== null ) pick( this.shown ); } );

		open.className = 'open';
		open.textContent = '▾';
		open.setAttribute( 'aria-label', 'Symbols' );
		open.addEventListener( 'click', () => this.menu.hidden = ! this.menu.hidden );

		this.menu.className = 'menu';
		this.menu.hidden = true;

		this.group.append( this.main, open, this.menu );

		parent.append( this.group );

		document.addEventListener( 'pointerdown', event => { if( ! this.group.contains( event.target as Node ) ) this.menu.hidden = true; } );

		document.addEventListener( 'keydown', event => { if( event.key === 'Escape' ) this.menu.hidden = true; } );
	}


	update( symbols : [ number, string ][] ) : void
	{
		if( JSON.stringify( symbols ) === JSON.stringify( this.symbols ) ) return;

		const layers = symbols.map( ( [ layer ] ) => layer );

		this.symbols = symbols;
		this.group.hidden = symbols.length === 0;

		this.show( this.shown !== null && layers.includes( this.shown ) ? this.shown : layers[ 0 ] ?? null );

		this.menu.replaceChildren( ... layers.map( layer =>
		{
			const item = document.createElement( 'button' );

			item.textContent = this.title( layer );
			item.addEventListener( 'click', () => { this.menu.hidden = true; this.pick( layer ); } );

			return item;
		} ) );
	}

	select( layer : number | null ) : void
	{
		if( layer !== null ) this.show( layer );

		this.main.classList.toggle( 'active', layer !== null );
	}

	private show( layer : number | null ) : void
	{
		this.shown = layer;
		this.main.textContent = layer === null ? '' : this.title( layer );
	}

	private title( layer : number ) : string
	{
		return this.symbols.find( ( [ other ] ) => other === layer )?.[ 1 ] || `Symbol ${ layer }`;
	}
}
