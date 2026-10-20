import { Kind } from './enums/kind.ts';
import type { Command } from './types/command.ts';


const SHORTCUTS : Record<string, Kind> = { w : Kind.Wire, r : Kind.Resistor, b : Kind.Battery, g : Kind.Ground, c : Kind.Capacitor, l : Kind.Inductor, s : Kind.Switch };


export function read( event : Pick<KeyboardEvent, 'key' | 'ctrlKey' | 'metaKey' | 'shiftKey'> ) : Command | undefined
{
	const key = event.key.toLowerCase();
	const modified = event.ctrlKey || event.metaKey;

	if( modified && key === 'z' && event.shiftKey ) return 'redo';

	if( modified && key === 'z' ) return 'undo';

	if( modified && key === 'y' ) return 'redo';

	if( modified ) return undefined;

	return SHORTCUTS[ key ];
}
