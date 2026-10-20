import { describe, it, expect } from 'vitest';
import { Kind } from '../../src/enums/kind.ts';
import { draw } from '../../src/renderers/element.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'element renderer', () =>
{
	const scale = { voltage : 0, current : 0 };

	function drawn( kind : Kind ) : unknown[][]
	{
		const { context, calls } = record();

		draw( context, element( { kind, value : 1 } ), scale );

		return calls;
	}

	it( 'draws each kind with its own renderer', () =>
	{
		expect( drawn( Kind.Wire ).some( call => call[ 0 ] === 'createLinearGradient' || call[ 0 ] === 'fillText' ) ).toBe( false );
		expect( drawn( Kind.Resistor ).some( call => call[ 0 ] === 'createLinearGradient' ) ).toBe( true );
		expect( drawn( Kind.Battery ).filter( call => call[ 0 ] === 'stroke' ) ).toHaveLength( 4 );
		expect( drawn( Kind.Ground ).filter( call => call[ 0 ] === 'moveTo' ) ).toHaveLength( 4 );
	} );
} );
