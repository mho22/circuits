export enum Kind
{
	Wire = 0,
	Resistor = 1,
	Battery = 2,
	Ground = 3,
	Capacitor = 4,
	Inductor = 5,
	Switch = 6,
	Diode = 7,
	Npn = 8,
	Pnp = 9,
	Nmos = 10,
	Pmos = 11,
	Supply = 12,
}


export const NAMES : Record<Kind, string> = {
	[ Kind.Wire ] : 'Wire',
	[ Kind.Resistor ] : 'Resistor',
	[ Kind.Battery ] : 'Battery',
	[ Kind.Ground ] : 'Ground',
	[ Kind.Capacitor ] : 'Capacitor',
	[ Kind.Inductor ] : 'Inductor',
	[ Kind.Switch ] : 'Switch',
	[ Kind.Diode ] : 'Diode',
	[ Kind.Npn ] : 'NPN',
	[ Kind.Pnp ] : 'PNP',
	[ Kind.Nmos ] : 'NMOS',
	[ Kind.Pmos ] : 'PMOS',
	[ Kind.Supply ] : 'VCC',
};

export const POINTS = [ Kind.Ground, Kind.Supply ];

export const FIELDS : Partial<Record<Kind, string>> = { [ Kind.Resistor ] : 'ohms', [ Kind.Battery ] : 'volts', [ Kind.Capacitor ] : 'farads', [ Kind.Inductor ] : 'henries', [ Kind.Supply ] : 'volts' };

export const UNITS : Partial<Record<Kind, string>> = { [ Kind.Resistor ] : 'Ω', [ Kind.Battery ] : 'V', [ Kind.Capacitor ] : 'F', [ Kind.Inductor ] : 'H', [ Kind.Supply ] : 'V' };

export const INPUTS : Partial<Record<Kind, [ string, number ]>> = { [ Kind.Resistor ] : [ 'Ω', 1 ], [ Kind.Battery ] : [ 'V', 1 ], [ Kind.Capacitor ] : [ 'µF', 1e-6 ], [ Kind.Inductor ] : [ 'mH', 1e-3 ], [ Kind.Supply ] : [ 'V', 1 ] };
