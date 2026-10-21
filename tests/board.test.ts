import { describe, it, expect } from 'vitest';
import { fit, locate } from '../src/board.ts';


describe( 'board', () =>
{
	it( 'puts the board center on a grid point of the circuit as the view shows it, at the size of the circuit', () =>
	{
		expect( fit( { x : 30, y : 20 }, 0, 0 ) ).toEqual( { x : 600, y : 400, unit : 5 } );
		expect( fit( { x : 30, y : 20 }, 13, -7 ) ).toEqual( { x : 587, y : 407, unit : 5 } );
	} );

	it( 'snaps a screen point to the nearest fine point', () =>
	{
		const board = { x : 600, y : 400, unit : 5 };

		expect( locate( board, 602, 401 ) ).toEqual( { x : 0, y : 0 } );
		expect( locate( board, 611, 389 ) ).toEqual( { x : 2, y : -2 } );
		expect( locate( board, 0, 1000 ) ).toEqual( { x : -120, y : 120 } );
	} );
} );
