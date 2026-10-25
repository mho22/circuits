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
	Input = 13,
	Symbol = 14,
	Output = 15,
	Toggle = 16,
	Led = 17,
	Clock = 18,
	Slider = 19,
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
	[ Kind.Input ] : 'Input',
	[ Kind.Symbol ] : 'Symbol',
	[ Kind.Output ] : 'Output',
	[ Kind.Toggle ] : 'Toggle',
	[ Kind.Led ] : 'LED',
	[ Kind.Clock ] : 'Clock',
	[ Kind.Slider ] : 'Slider',
};

export const POINTS = [ Kind.Ground, Kind.Supply, Kind.Symbol ];

export const PINS = [ Kind.Input, Kind.Output ];

export const LEVELS = [ Kind.Input, Kind.Toggle ];

export const TRANSISTORS = [ Kind.Npn, Kind.Pnp, Kind.Nmos, Kind.Pmos ];

export const FIELDS : Partial<Record<Kind, string>> = { [ Kind.Resistor ] : 'ohms', [ Kind.Battery ] : 'volts', [ Kind.Capacitor ] : 'farads', [ Kind.Inductor ] : 'henries', [ Kind.Supply ] : 'volts', [ Kind.Symbol ] : 'layer', [ Kind.Clock ] : 'hertz', [ Kind.Slider ] : 'volts' };

export const UNITS : Partial<Record<Kind, string>> = { [ Kind.Resistor ] : 'Ω', [ Kind.Battery ] : 'V', [ Kind.Capacitor ] : 'F', [ Kind.Inductor ] : 'H', [ Kind.Supply ] : 'V', [ Kind.Clock ] : 'Hz', [ Kind.Slider ] : 'V' };

export const INPUTS : Partial<Record<Kind, [ string, number ]>> = { [ Kind.Resistor ] : [ 'Ω', 1 ], [ Kind.Battery ] : [ 'V', 1 ], [ Kind.Capacitor ] : [ 'µF', 1e-6 ], [ Kind.Inductor ] : [ 'mH', 1e-3 ], [ Kind.Supply ] : [ 'V', 1 ], [ Kind.Clock ] : [ 'MHz', 1e6 ], [ Kind.Slider ] : [ 'V', 1 ] };
