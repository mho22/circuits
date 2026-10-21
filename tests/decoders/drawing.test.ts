import { describe, it, expect } from 'vitest';
import { decode } from '../../src/decoders/drawing.ts';
import { encode } from '../../src/encoders/drawing.ts';
import { Figure } from '../../src/enums/figure.ts';


describe( 'drawing decoder', () =>
{
	it( 'reads back each shape the encoder writes', () =>
	{
		const shapes = [
			{ figure : Figure.Line as const, from : { x : -1, y : 0 }, to : { x : 2, y : -3 } },
			{ figure : Figure.Circle as const, center : { x : 0, y : 1 }, radius : 2 },
		];

		expect( decode( encode( shapes ) ) ).toEqual( shapes );
	} );

	it( 'reads no shape from empty data', () =>
	{
		expect( decode( new Int32Array() ) ).toEqual( [] );
	} );
} );
