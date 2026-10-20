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


export const UNITS : Partial<Record<Kind, string>> = { [ Kind.Resistor ] : 'Ω', [ Kind.Battery ] : 'V', [ Kind.Capacitor ] : 'F', [ Kind.Inductor ] : 'H' };
