import { describe, it, expect } from 'vitest';
import { Kind } from '../src/enums/kind.ts';
import { Junctions } from '../src/junctions.ts';
import { element } from './stubs/element.ts';


describe( 'junctions', () =>
{
	const junctions = ( elements : Parameters<Junctions[ 'find' ]>[ 0 ] ) => new Junctions().find( elements );

	it( 'marks a point where three wire ends meet with the voltage of the wires', () =>
	{
		const wires = [
			element( { from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, potentials : [ 5, 5 ] } ),
			element( { from : { x : 2, y : 0 }, to : { x : 4, y : 0 }, potentials : [ 5, 5 ] } ),
			element( { from : { x : 2, y : 0 }, to : { x : 2, y : 2 }, potentials : [ 5, 5 ] } ),
		];

		expect( junctions( wires ) ).toEqual( [ { point : { x : 2, y : 0 }, potential : 5 } ] );
	} );

	it( 'marks a point where two wire ends meet', () =>
	{
		const wires = [ element( { from : { x : 2, y : -2 }, to : { x : 2, y : 0 } } ), element( { from : { x : 2, y : 0 }, to : { x : 2, y : 2 } } ) ];

		expect( junctions( wires ) ).toEqual( [ { point : { x : 2, y : 0 }, potential : 0 } ] );
	} );

	it( 'leaves a wire end on a part or on the middle of a wire unmarked', () =>
	{
		expect( junctions( [ element( { from : { x : 0, y : 0 }, to : { x : 4, y : 0 } } ), element( { from : { x : 2, y : 0 }, to : { x : 2, y : 2 } } ) ] ) ).toEqual( [] );
		expect( junctions( [ element( { from : { x : 0, y : 0 }, to : { x : 2, y : 0 } } ), element( { kind : Kind.Resistor, from : { x : 2, y : 0 }, to : { x : 4, y : 0 } } ) ] ) ).toEqual( [] );
	} );

	it( 'marks a wire end that meets two terminals and counts a pin and a symbol once each', () =>
	{
		const wire = element( { from : { x : 0, y : 0 }, to : { x : 2, y : 0 } } );
		const resistor = element( { kind : Kind.Resistor, from : { x : 2, y : 0 }, to : { x : 4, y : 0 } } );

		expect( junctions( [ wire, resistor, element( { kind : Kind.Diode, from : { x : 2, y : 0 }, to : { x : 2, y : 2 } } ) ] ) ).toHaveLength( 1 );
		expect( junctions( [ wire, element( { kind : Kind.Input, from : { x : 3, y : 0 }, to : { x : 2, y : 0 } } ) ] ) ).toEqual( [] );
		expect( junctions( [ wire, resistor, element( { kind : Kind.Symbol, from : { x : 4, y : 0 }, to : { x : 4, y : 0 }, outline : { low : { x : -2, y : -2 }, high : { x : 2, y : 2 }, pins : [ { x : -2, y : 0 } ], lights : [] } } ) ] ) ).toHaveLength( 1 );
	} );

	it( 'keeps the junctions of a same layout and reads the new voltage of its wires', () =>
	{
		const finder = new Junctions();
		const wires = ( volts : number ) => [ element( { from : { x : 0, y : 0 }, to : { x : 2, y : 0 }, potentials : [ volts, volts ] } ), element( { from : { x : 2, y : 0 }, to : { x : 2, y : 2 }, potentials : [ volts, volts ] } ) ];

		finder.find( wires( 1 ) );

		expect( finder.find( wires( 3 ) ) ).toEqual( [ { point : { x : 2, y : 0 }, potential : 3 } ] );
	} );

	it( 'finds the junctions again when a part moves', () =>
	{
		const finder = new Junctions();
		const wire = element( { from : { x : 0, y : 0 }, to : { x : 2, y : 0 } } );

		finder.find( [ wire, element( { from : { x : 2, y : 0 }, to : { x : 2, y : 2 } } ) ] );

		expect( finder.find( [ wire, element( { from : { x : 4, y : 0 }, to : { x : 4, y : 2 } } ) ] ) ).toEqual( [] );
	} );
} );
