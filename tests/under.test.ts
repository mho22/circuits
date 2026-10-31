import { describe, it, expect } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { under } from '../src/under.ts';
import { element } from './stubs/element.ts';


describe( 'under', () =>
{
	const symbol = element( { id : 0, kind : Kind.Symbol, from : { x : -2, y : 0 }, to : { x : -2, y : 0 } } );
	const wire = element( { id : 1, from : { x : 0, y : 0 }, to : { x : 1, y : 0 } } );

	it( 'picks a wire under a point between its ends before a symbol', () =>
	{
		expect( under( [ symbol, wire ], { x : 2, y : 0 } ) ).toBe( wire );
		expect( under( [ symbol, wire ], { x : 2, y : 1 } ) ).toBe( wire );
	} );

	it( 'picks a symbol before a wire at an end of the wire or beside the wire', () =>
	{
		expect( under( [ symbol, wire ], { x : 1, y : 0 } ) ).toBe( symbol );
		expect( under( [ symbol, wire ], { x : 2, y : 2 } ) ).toBe( symbol );
	} );

	it( 'picks the first wire when no symbol is under the point', () =>
	{
		expect( under( [ wire ], { x : 0, y : 0 } ) ).toBe( wire );
	} );

	it( 'picks nothing from no element', () =>
	{
		expect( under( [], { x : 0, y : 0 } ) ).toBeUndefined();
	} );
} );
