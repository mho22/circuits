import { describe, it, expect } from 'vitest';
import { split } from '../src/category.ts';


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
} );
