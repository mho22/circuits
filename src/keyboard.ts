import { Kind } from './enums/kind.ts';
import type { Command } from './types/command.ts';


export const SHORTCUTS : Record<string, Kind> = { w : Kind.Wire, r : Kind.Resistor, b : Kind.Battery, g : Kind.Ground, c : Kind.Capacitor, l : Kind.Inductor, s : Kind.Switch, d : Kind.Diode, n : Kind.Npn, p : Kind.Pnp, m : Kind.Nmos, f : Kind.Pmos, v : Kind.Supply };


export function read( event : Pick<KeyboardEvent, 'key' | 'ctrlKey' | 'metaKey' | 'shiftKey'> ) : Command | undefined
{
	const key = event.key.toLowerCase();
	const modified = event.ctrlKey || event.metaKey;

	if( modified && key === 'z' && event.shiftKey ) return 'redo';

	if( modified && key === 'z' ) return 'undo';

	if( modified && key === 'y' ) return 'redo';

	if( modified && key === 's' ) return 'save';

	if( modified && key === 'o' ) return 'open';

	if( modified ) return undefined;

	return SHORTCUTS[ key ];
}
