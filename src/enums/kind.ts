export enum Kind
{
	Wire = 0,
	Resistor = 1,
	Battery = 2,
	Ground = 3,
}


export const UNITS : Partial<Record<Kind, string>> = { [ Kind.Resistor ] : 'Ω', [ Kind.Battery ] : 'V' };
