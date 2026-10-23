import { describe, it, expect } from 'vitest';
import { split, title } from '../src/category.ts';


describe( 'category', () =>
{
	it( 'splits a name into its path and its title', () =>
	{
		expect( split( 'foo' ) ).toEqual( { path : [], title : 'foo' } );
		expect( split( 'foo/bar/baz' ) ).toEqual( { path : [ 'foo', 'bar' ], title : 'baz' } );
		expect( split( ' foo / bar / baz ' ) ).toEqual( { path : [ 'foo', 'bar' ], title : 'baz' } );
		expect( split( 'foo//bar' ) ).toEqual( { path : [ 'foo' ], title : 'bar' } );
		expect( split( 'foo/' ) ).toEqual( { path : [ 'foo' ], title : '' } );
		expect( split( '' ) ).toEqual( { path : [], title : '' } );
	} );

	it( 'gives the title of a name, or the number of its layer when the title is empty', () =>
	{
		expect( title( 'foo/bar', 3 ) ).toBe( 'bar' );
		expect( title( 'foo/', 3 ) ).toBe( 'Symbol 3' );
		expect( title( '', 4 ) ).toBe( 'Symbol 4' );
	} );
} );
