// @vitest-environment happy-dom

import { afterEach, describe, it, expect, vi } from 'vitest';
import { download } from '../src/download.ts';


describe( 'download', () =>
{
	afterEach( () =>
	{
		vi.restoreAllMocks();
		vi.unstubAllGlobals();
	} );

	it( 'saves the text in a file with its name and frees the link', async () =>
	{
		const blobs : Blob[] = [];
		const links : HTMLAnchorElement[] = [];

		vi.spyOn( URL, 'createObjectURL' ).mockImplementation( blob => { blobs.push( blob as Blob ); return 'blob:foo'; } );
		vi.spyOn( HTMLAnchorElement.prototype, 'click' ).mockImplementation( function( this : HTMLAnchorElement ) { links.push( this ); } );

		const revoke = vi.spyOn( URL, 'revokeObjectURL' ).mockImplementation( () => {} );

		expect( await download( '{ "foo" : 1 }', 'bar.circuit' ) ).toBe( 'bar.circuit' );
		expect( links.map( link => [ link.href, link.download ] ) ).toEqual( [ [ 'blob:foo', 'bar.circuit' ] ] );
		expect( await blobs[ 0 ].text() ).toBe( '{ "foo" : 1 }' );
		expect( revoke ).toHaveBeenCalledWith( 'blob:foo' );
	} );

	it( 'asks for the name of the file when the browser can and writes the text in it', async () =>
	{
		const chunks : string[] = [];
		const close = vi.fn();
		const picker = vi.fn( async () => ( { name : 'baz.circuit', createWritable : async () => ( { write : async ( chunk : string ) => { chunks.push( chunk ); }, close } ) } ) );
		const click = vi.spyOn( HTMLAnchorElement.prototype, 'click' );

		vi.stubGlobal( 'showSaveFilePicker', picker );

		expect( await download( '{ "foo" : 1 }', 'bar.circuit' ) ).toBe( 'baz.circuit' );
		expect( picker ).toHaveBeenCalledWith( { suggestedName : 'bar.circuit', types : [ { description : 'Circuit', accept : { 'application/json' : [ '.circuit' ] } } ] } );
		expect( chunks ).toEqual( [ '{ "foo" : 1 }' ] );
		expect( close ).toHaveBeenCalled();
		expect( click ).not.toHaveBeenCalled();
	} );

	it( 'writes nothing when the save is canceled', async () =>
	{
		const cancel = new DOMException( 'The user aborted a request.', 'AbortError' );

		vi.stubGlobal( 'showSaveFilePicker', vi.fn( async () => { throw cancel; } ) );

		await expect( download( '{ "foo" : 1 }', 'bar.circuit' ) ).rejects.toBe( cancel );
	} );
} );
