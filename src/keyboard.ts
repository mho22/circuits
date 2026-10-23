import { Figure } from './enums/figure.ts';
import { Kind } from './enums/kind.ts';
import type { Command } from './types/command.ts';


export const SHORTCUTS : Record<string, Kind> = { w : Kind.Wire, r : Kind.Resistor, b : Kind.Battery, g : Kind.Ground, c : Kind.Capacitor, l : Kind.Inductor, s : Kind.Switch, d : Kind.Diode, n : Kind.Npn, p : Kind.Pnp, m : Kind.Nmos, f : Kind.Pmos, v : Kind.Supply, i : Kind.Input, o : Kind.Output, t : Kind.Toggle, e : Kind.Led, k : Kind.Clock };


export const FIGURES : Record<string, Figure> = { c : Figure.Circle, l : Figure.Line, u : Figure.Curve, t : Figure.Text };


export function read( event : Pick<KeyboardEvent, 'key' | 'ctrlKey' | 'metaKey' | 'shiftKey'> ) : Command | undefined
{
	const key = event.key.toLowerCase();
	const modified = event.ctrlKey || event.metaKey;

	if( modified && key === 'z' && event.shiftKey ) return 'redo';

	if( modified && key === 'z' ) return 'undo';

	if( modified && key === 'y' ) return 'redo';

	if( modified && key === 'c' ) return 'copy';

	if( modified && key === 'v' ) return 'paste';

	if( modified && key === 's' ) return 'save';

	if( modified && key === 'o' ) return 'open';

	if( modified && ( key === '+' || key === '=' ) ) return 'zoom';

	if( modified && key === '-' ) return 'dezoom';

	if( modified ) return undefined;

	return SHORTCUTS[ key ];
}

export function shape( event : Pick<KeyboardEvent, 'key' | 'ctrlKey' | 'metaKey'> ) : Figure | undefined
{
	if( event.ctrlKey || event.metaKey ) return undefined;

	return FIGURES[ event.key.toLowerCase() ];
}
