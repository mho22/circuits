import { Kind, NAMES } from './enums/kind.ts';
import { SHORTCUTS } from './keyboard.ts';
import type { Group } from './types/group.ts';


export const PART_GROUPS : Group[] = [
	{ name : 'Wiring', kinds : [ Kind.Wire, Kind.Switch ] },
	{ name : 'Passive', kinds : [ Kind.Resistor, Kind.Capacitor, Kind.Inductor ] },
	{ name : 'Semiconductors', kinds : [ Kind.Diode, Kind.Led, Kind.Npn, Kind.Pnp, Kind.Nmos, Kind.Pmos ] },
	{ name : 'Sources', kinds : [ Kind.Battery, Kind.Supply, Kind.Toggle, Kind.Ground ] },
];

export const PIN_GROUPS : Group[] = [
	{ name : 'Pins', kinds : [ Kind.Input, Kind.Output ] },
];


function button( className : string, kind : Kind ) : HTMLButtonElement
{
	const element = document.createElement( 'button' );

	element.className = className;

	label( element, kind );

	return element;
}

function label( element : HTMLButtonElement, kind : Kind ) : void
{
	const shortcut = document.createElement( 'kbd' );

	shortcut.textContent = Object.keys( SHORTCUTS ).find( key => SHORTCUTS[ key ] === kind )!.toUpperCase();

	element.dataset.kind = String( kind );
	element.replaceChildren( NAMES[ kind ], shortcut );
}


export class Toolbar
{
	private containers : HTMLElement[] = [];

	private picks : HTMLButtonElement[] = [];

	private menus : HTMLElement[] = [];


	constructor( parent : HTMLElement, private groups : Group[], pick : ( kind : Kind ) => void )
	{
		groups.forEach( group =>
		{
			const container = document.createElement( 'div' );
			const main = button( 'pick', group.kinds[ 0 ] );

			container.className = 'group';
			container.append( main );

			main.addEventListener( 'click', () => pick( Number( main.dataset.kind ) ) );

			parent.append( container );

			this.containers.push( container );
			this.picks.push( main );

			if( group.kinds.length < 2 ) return;

			const open = document.createElement( 'button' );
			const menu = document.createElement( 'div' );

			open.className = 'open';
			open.textContent = '▾';
			open.setAttribute( 'aria-label', `${ group.name } parts` );
			open.addEventListener( 'click', () =>
			{
				const hidden = menu.hidden;

				this.close();

				menu.hidden = ! hidden;
			} );

			menu.className = 'menu';
			menu.hidden = true;

			group.kinds.forEach( kind =>
			{
				const item = button( 'item', kind );

				item.addEventListener( 'click', () => { this.close(); pick( kind ); } );

				menu.append( item );
			} );

			container.append( open, menu );

			this.menus.push( menu );
		} );

		document.addEventListener( 'pointerdown', event => { if( ! parent.contains( event.target as Node ) ) this.close(); } );

		document.addEventListener( 'keydown', event => { if( event.key === 'Escape' ) this.close(); } );
	}


	select( kind : Kind | null ) : void
	{
		this.groups.forEach( ( group, index ) =>
		{
			const member = kind !== null && group.kinds.includes( kind );

			if( member ) label( this.picks[ index ], kind );

			this.picks[ index ].classList.toggle( 'active', member );
		} );
	}

	limit( kinds : Kind[] ) : void
	{
		this.groups.forEach( ( group, index ) => this.containers[ index ].hidden = ! group.kinds.some( kind => kinds.includes( kind ) ) );
	}

	close() : void
	{
		this.menus.forEach( menu => menu.hidden = true );
	}
}
