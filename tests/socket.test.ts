import { describe, it, expect } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { socket } from '../src/socket.ts';
import { element } from './stubs/element.ts';


describe( 'socket', () =>
{
	const outline = { low : { x : -2, y : -2 }, high : { x : 2, y : 2 }, pins : [ { x : -2, y : 0 }, { x : 2, y : 0 } ], lights : [] };

	it( 'finds the pin of a symbol at a point after the turn', () =>
	{
		const symbol = element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline, rotation : 1 } );

		expect( [ socket( symbol, { x : 5, y : 3 } ), socket( symbol, { x : 5, y : 7 } ) ] ).toEqual( [ 0, 1 ] );
	} );

	it( 'finds no pin away from the pins or on a part without an outline', () =>
	{
		expect( socket( element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline } ), { x : 5, y : 5 } ) ).toBe( -1 );
		expect( socket( element(), { x : 0, y : 0 } ) ).toBe( -1 );
	} );
} );
