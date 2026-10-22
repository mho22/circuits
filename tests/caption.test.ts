import { describe, it, expect } from 'vitest';
import { caption } from '../src/caption.ts';
import { Kind } from '../src/enums/kind.ts';
import { element } from './stubs/element.ts';


describe( 'caption', () =>
{
	it( 'covers the label of a pin on the side away from its lead', () =>
	{
		const input = element( { kind : Kind.Input, from : { x : 10, y : 10 }, to : { x : 11, y : 10 } } );

		expect( caption( { ... input, label : 'foo' } ) ).toEqual( [ { x : 8, y : 9 }, { x : 10, y : 11 } ] );
		expect( caption( { ... input, label : 'foobarbaz' } ) ).toEqual( [ { x : 6, y : 9 }, { x : 10, y : 11 } ] );
		expect( caption( { ... input, to : { x : 9, y : 10 }, label : 'foo' } ) ).toEqual( [ { x : 10, y : 9 }, { x : 12, y : 11 } ] );
	} );

	it( 'centers the label of an upright pin above or below it', () =>
	{
		const output = element( { kind : Kind.Output, from : { x : 10, y : 10 }, to : { x : 10, y : 11 }, label : 'foo' } );

		expect( caption( output ) ).toEqual( [ { x : 9, y : 8 }, { x : 11, y : 10 } ] );
		expect( caption( { ... output, to : { x : 10, y : 9 } } ) ).toEqual( [ { x : 9, y : 10 }, { x : 11, y : 12 } ] );
	} );

	it( 'covers nothing for a pin without a label', () =>
	{
		expect( caption( element( { kind : Kind.Input, label : '' } ) ) ).toEqual( [] );
		expect( caption( element( { kind : Kind.Input } ) ) ).toEqual( [] );
	} );
} );
