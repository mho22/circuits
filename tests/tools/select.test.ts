import { describe, it, expect } from 'vitest';
import { SelectTool } from '../../src/tools/select.ts';


describe( 'select tool', () =>
{
	it( 'previews the area between the press and the cursor in any direction', () =>
	{
		const tool = new SelectTool();

		tool.press( { x : 4, y : 1 } );
		tool.move( { x : 1, y : 3 } );

		expect( tool.preview() ).toEqual( { low : { x : 1, y : 1 }, high : { x : 4, y : 3 } } );
	} );

	it( 'gives the area on release and stops', () =>
	{
		const tool = new SelectTool();

		tool.press( { x : 0, y : 0 } );
		tool.move( { x : 2, y : 2 } );

		expect( tool.release() ).toEqual( { low : { x : 0, y : 0 }, high : { x : 2, y : 2 } } );
		expect( tool.preview() ).toBeNull();
	} );

	it( 'gives no area for a click or a flat drag', () =>
	{
		const tool = new SelectTool();

		tool.press( { x : 0, y : 0 } );

		expect( tool.release() ).toBeNull();

		tool.press( { x : 0, y : 0 } );
		tool.move( { x : 5, y : 0 } );

		expect( tool.release() ).toBeNull();
	} );

	it( 'ignores moves and releases without a press', () =>
	{
		const tool = new SelectTool();

		tool.move( { x : 3, y : 3 } );

		expect( tool.preview() ).toBeNull();
		expect( tool.release() ).toBeNull();
	} );
} );
