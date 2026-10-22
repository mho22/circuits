import { describe, it, expect } from 'vitest';
import { Figure } from '../src/enums/figure.ts';
import { Kind } from '../src/enums/kind.ts';
import { cover, inside, span } from '../src/selection.ts';
import { element } from './stubs/element.ts';


describe( 'selection', () =>
{
	const area = { low : { x : 0, y : 0 }, high : { x : 4, y : 4 } };

	it( 'keeps the parts with every terminal inside the area or on its edge', () =>
	{
		const elements = [
			element( { id : 1, from : { x : 0, y : 0 }, to : { x : 4, y : 0 } } ),
			element( { id : 2, from : { x : 2, y : 2 }, to : { x : 6, y : 2 } } ),
			element( { id : 3, kind : Kind.Npn, from : { x : 1, y : 2 }, to : { x : 3, y : 2 } } ),
			element( { id : 4, kind : Kind.Npn, from : { x : 1, y : 0 }, to : { x : 3, y : 0 } } ),
		];

		expect( inside( elements, area ) ).toEqual( [ 1, 3 ] );
	} );

	it( 'keeps an input or an output only when its square is inside too', () =>
	{
		const elements = [
			element( { id : 1, kind : Kind.Input, from : { x : 1, y : 1 }, to : { x : 2, y : 1 } } ),
			element( { id : 2, kind : Kind.Output, from : { x : 5, y : 1 }, to : { x : 4, y : 1 } } ),
		];

		expect( inside( elements, area ) ).toEqual( [ 1 ] );
	} );

	it( 'keeps a symbol only when its pins are inside too', () =>
	{
		const outline = { low : { x : -2, y : -2 }, high : { x : 2, y : 2 }, pins : [ { x : -2, y : 0 }, { x : 2, y : 0 } ] };
		const elements = [
			element( { id : 1, kind : Kind.Symbol, from : { x : 2, y : 2 }, to : { x : 2, y : 2 }, outline } ),
			element( { id : 2, kind : Kind.Symbol, from : { x : 3, y : 2 }, to : { x : 3, y : 2 }, outline } ),
		];

		expect( inside( elements, area ) ).toEqual( [ 1 ] );
	} );

	it( 'grows the area to cover points with one step around them and keeps it for points one step inside', () =>
	{
		expect( cover( area, [ { x : -2, y : 1 }, { x : 1, y : 6 } ] ) ).toEqual( { low : { x : -3, y : 0 }, high : { x : 4, y : 7 } } );
		expect( cover( area, [ { x : 4, y : 2 }, { x : 2, y : 0 } ] ) ).toEqual( { low : { x : 0, y : -1 }, high : { x : 5, y : 4 } } );
		expect( cover( area, [ { x : 1, y : 1 }, { x : 3, y : 3 } ] ) ).toEqual( area );
	} );

	it( 'spans every part with one step around them, or nothing without a part', () =>
	{
		const elements = [ element( { from : { x : 1, y : 2 }, to : { x : 5, y : 2 } } ), element( { kind : Kind.Resistor, from : { x : 3, y : 2 }, to : { x : 3, y : 4 } } ) ];

		expect( span( elements ) ).toEqual( { low : { x : 0, y : 1 }, high : { x : 6, y : 5 } } );
		expect( span( [] ) ).toBeNull();
	} );

	it( 'spans the bars of a ground below its point and of a vcc above its point', () =>
	{
		const ground = element( { kind : Kind.Ground, from : { x : 0, y : 0 }, to : { x : 0, y : 0 } } );
		const supply = element( { kind : Kind.Supply, from : { x : 0, y : 0 }, to : { x : 0, y : 0 } } );

		expect( span( [ ground ] ) ).toEqual( { low : { x : -1, y : -1 }, high : { x : 1, y : 2 } } );
		expect( span( [ supply ] ) ).toEqual( { low : { x : -1, y : -2 }, high : { x : 1, y : 1 } } );
	} );

	it( 'spans the drawing of a symbol, turned with the symbol, with one step around it', () =>
	{
		const outline = { low : { x : -2, y : -2 }, high : { x : 2, y : 2 }, pins : [ { x : -2, y : 0 } ] };
		const definition = { name : 'foo', drawing : [ { figure : Figure.Line as const, from : { x : -8, y : -10 }, to : { x : 4, y : 6 } } ], labels : [], layout : [], delay : NaN };

		expect( span( [ element( { kind : Kind.Symbol, from : { x : 0, y : 0 }, to : { x : 0, y : 0 }, outline, definition } ) ] ) ).toEqual( { low : { x : -3, y : -4 }, high : { x : 2, y : 3 } } );
		expect( span( [ element( { kind : Kind.Symbol, from : { x : 0, y : 0 }, to : { x : 0, y : 0 }, rotation : 1, outline, definition } ) ] ) ).toEqual( { low : { x : -3, y : -3 }, high : { x : 4, y : 2 } } );
	} );

	it( 'spans the box of a symbol without a drawing with one step around it', () =>
	{
		const outline = { low : { x : -3, y : -1 }, high : { x : 2, y : 1 }, pins : [ { x : -3, y : 0 } ] };

		expect( span( [ element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline } ) ] ) ).toEqual( { low : { x : 1, y : 3 }, high : { x : 8, y : 7 } } );
		expect( span( [ element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, rotation : 1, outline } ) ] ) ).toEqual( { low : { x : 3, y : 1 }, high : { x : 7, y : 8 } } );
	} );
} );
