export enum Kind
{
	Wire = 0,
	Resistor = 1,
	Battery = 2,
	Ground = 3,
	Capacitor = 4,
	Inductor = 5,
	Switch = 6,
}


export const NAMES : Record<Kind, string> = {
	[ Kind.Wire ] : 'Wire',
	[ Kind.Resistor ] : 'Resistor',
	[ Kind.Battery ] : 'Battery',
	[ Kind.Ground ] : 'Ground',
	[ Kind.Capacitor ] : 'Capacitor',
	[ Kind.Inductor ] : 'Inductor',
	[ Kind.Switch ] : 'Switch',
};

export const UNITS : Partial<Record<Kind, string>> = { [ Kind.Resistor ] : 'Ω', [ Kind.Battery ] : 'V', [ Kind.Capacitor ] : 'F', [ Kind.Inductor ] : 'H' };
