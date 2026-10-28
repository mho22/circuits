import { describe, it, expect } from 'vitest';
import { seen, sight } from '../src/sight.ts';


describe( 'sight', () =>
{
	const area = { low : { x : 0, y : 0 }, high : { x : 10, y : 10 } };

	it( 'widens the view by a margin of whole grid points', () =>
	{
		expect( sight( { x : 0.5, y : -1.5 }, { x : 10.2, y : 8 } ) ).toEqual( { low : { x : -4, y : -6 }, high : { x : 15, y : 12 } } );
	} );

	it( 'sees a segment inside the area, on its edge, or across it', () =>
	{
		expect( seen( area, { x : 2, y : 2 }, { x : 4, y : 2 } ) ).toBe( true );
		expect( seen( area, { x : 10, y : 2 }, { x : 14, y : 2 } ) ).toBe( true );
		expect( seen( area, { x : -6, y : 5 }, { x : 16, y : 5 } ) ).toBe( true );
	} );

	it( 'misses a segment outside the area', () =>
	{
		expect( seen( area, { x : 11, y : 2 }, { x : 16, y : 2 } ) ).toBe( false );
		expect( seen( area, { x : 2, y : -8 }, { x : 2, y : -1 } ) ).toBe( false );
	} );
} );
