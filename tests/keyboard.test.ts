import { describe, it, expect } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { Kind } from '../src/enums/kind.ts';
import { read, shape } from '../src/keyboard.ts';


describe( 'keyboard', () =>
{
	function press( key : string, modifiers : { ctrlKey? : boolean, metaKey? : boolean, shiftKey? : boolean } = {} ) : ReturnType<typeof read>
	{
		return read( { key, ctrlKey : false, metaKey : false, shiftKey : false, ... modifiers } );
	}

	it( 'picks a part with its letter in either case', () =>
	{
		expect( [ 'w', 'r', 'b', 'g', 'c', 'l', 's', 'd', 'n', 'p', 'm', 'f', 'v', 'i', 'o', 't', 'e', 'k' ].map( key => press( key ) ) ).toEqual( [ Kind.Wire, Kind.Resistor, Kind.Battery, Kind.Ground, Kind.Capacitor, Kind.Inductor, Kind.Switch, Kind.Diode, Kind.Npn, Kind.Pnp, Kind.Nmos, Kind.Pmos, Kind.Supply, Kind.Input, Kind.Output, Kind.Toggle, Kind.Led, Kind.Clock ] );
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

	it( 'saves and opens with control or command and s or o', () =>
	{
		expect( press( 's', { ctrlKey : true } ) ).toBe( 'save' );
		expect( press( 'S', { metaKey : true } ) ).toBe( 'save' );
		expect( press( 'o', { ctrlKey : true } ) ).toBe( 'open' );
		expect( press( 'o', { metaKey : true } ) ).toBe( 'open' );
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

	it( 'picks a figure with its letter in either case, without control or command', () =>
	{
		const key = ( key : string, modified = false ) => shape( { key, ctrlKey : modified, metaKey : false } );

		expect( [ 'C', 'l', 'r' ].map( letter => key( letter ) ) ).toEqual( [ Figure.Circle, Figure.Line, undefined ] );
		expect( key( 'l', true ) ).toBeUndefined();
	} );
} );
