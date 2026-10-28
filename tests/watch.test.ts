import { describe, it, expect } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { Watch } from '../src/watch.ts';
import { element } from './stubs/element.ts';
import type { Scene } from '../src/types/scene.ts';


describe( 'watch', () =>
{
	const range = { low : { x : 0, y : 0 }, high : { x : 10, y : 10 } };
	const scene = ( elements = [ element(), element( { from : { x : 20, y : 0 }, to : { x : 24, y : 0 } } ), element( { kind : Kind.Symbol, from : { x : 40, y : 40 }, to : { x : 40, y : 40 } } ) ] ) : Scene => ( { revision : 0, elements, starts : [ 0, 0, 0 ], junctions : [ { point : { x : 2, y : 0 }, wire : 0 }, { point : { x : 20, y : 0 }, wire : 1 } ] } );

	it( 'finds the parts and the junctions in an area and every part with a caption', () =>
	{
		expect( new Watch().look( scene(), range ) ).toEqual( { indexes : [ 0, 2 ], junctions : [ { point : { x : 2, y : 0 }, wire : 0 } ] } );
	} );

	it( 'keeps what it found for a same scene and a same area', () =>
	{
		const watch = new Watch();
		const shown = scene();
		const found = watch.look( shown, range );

		expect( watch.look( shown, { low : { x : 0, y : 0 }, high : { x : 10, y : 10 } } ) ).toBe( found );
	} );

	it( 'looks again after the area or the scene changes', () =>
	{
		const watch = new Watch();
		const shown = scene();

		watch.look( shown, range );

		expect( watch.look( shown, { low : { x : 20, y : 0 }, high : { x : 30, y : 10 } } ).indexes ).toEqual( [ 1, 2 ] );
		expect( watch.look( scene( [ element() ] ), { low : { x : 20, y : 0 }, high : { x : 30, y : 10 } } ).indexes ).toEqual( [] );
	} );
} );
