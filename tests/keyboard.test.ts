import { describe, it, expect } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { read } from '../src/keyboard.ts';


describe( 'keyboard', () =>
{
	function press( key : string, modifiers : { ctrlKey? : boolean, metaKey? : boolean, shiftKey? : boolean } = {} ) : ReturnType<typeof read>
	{
		return read( { key, ctrlKey : false, metaKey : false, shiftKey : false, ... modifiers } );
	}

	it( 'picks a part with its letter in either case', () =>
	{
		expect( [ 'w', 'r', 'b', 'g', 'c', 'l', 's', 'd', 'n', 'p', 'm', 'f' ].map( key => press( key ) ) ).toEqual( [ Kind.Wire, Kind.Resistor, Kind.Battery, Kind.Ground, Kind.Capacitor, Kind.Inductor, Kind.Switch, Kind.Diode, Kind.Npn, Kind.Pnp, Kind.Nmos, Kind.Pmos ] );
		expect( press( 'R', { shiftKey : true } ) ).toBe( Kind.Resistor );
	} );

	it( 'undoes with control or command and z', () =>
	{
		expect( press( 'z', { ctrlKey : true } ) ).toBe( 'undo' );
		expect( press( 'z', { metaKey : true } ) ).toBe( 'undo' );
	} );

	it( 'redoes with shift and z, or with y', () =>
	{
		expect( press( 'Z', { metaKey : true, shiftKey : true } ) ).toBe( 'redo' );
		expect( press( 'y', { ctrlKey : true } ) ).toBe( 'redo' );
	} );

	it( 'leaves other shortcuts of the browser alone', () =>
	{
		expect( press( 'c', { metaKey : true } ) ).toBeUndefined();
		expect( press( 'r', { ctrlKey : true } ) ).toBeUndefined();
	} );

	it( 'ignores a key without a command', () =>
	{
		expect( press( 'x' ) ).toBeUndefined();
		expect( press( 'Escape' ) ).toBeUndefined();
	} );
} );
