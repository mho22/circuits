import { describe, it, expect } from 'vitest';
import { seen } from '../src/sight.ts';


describe( 'sight', () =>
{
	const sight = { low : { x : 0, y : 0 }, high : { x : 10, y : 10 } };

	it( 'sees a segment inside the area or within the margin around it', () =>
	{
		expect( seen( sight, { x : 2, y : 2 }, { x : 4, y : 2 } ) ).toBe( true );
		expect( seen( sight, { x : 13, y : 2 }, { x : 14, y : 2 } ) ).toBe( true );
		expect( seen( sight, { x : -6, y : 5 }, { x : 16, y : 5 } ) ).toBe( true );
	} );

	it( 'misses a segment beyond the margin', () =>
	{
		expect( seen( sight, { x : 15, y : 2 }, { x : 16, y : 2 } ) ).toBe( false );
		expect( seen( sight, { x : 2, y : -8 }, { x : 2, y : -5 } ) ).toBe( false );
	} );
} );
