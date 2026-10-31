import { describe, it, expect } from 'vitest';
import { color } from '../../src/colors/voltage.ts';
import { Color } from '../../src/enums/color.ts';
import { Figure } from '../../src/enums/figure.ts';
import { Kind } from '../../src/enums/kind.ts';
import { FINE, SPACING } from '../../src/grid.ts';
import { draw } from '../../src/renderers/symbol.ts';
import { BUS } from '../../src/renderers/wire.ts';
import { record } from '../stubs/context.ts';
import { element } from '../stubs/element.ts';


describe( 'symbol renderer', () =>
{
	const outline = { low : { x : -2, y : -3 }, high : { x : 2, y : 3 }, pins : [ { x : -2, y : 1 }, { x : 2, y : 0 } ], lights : [] };

	it( 'draws a rounded box around its outline without a drawing and one dot per pin in the color of its voltage', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline, potentials : [ 5, 0 ] } ), { voltage : 5, current : 0 }, 1 );

		expect( calls ).toContainEqual( [ 'roundRect', 3 * SPACING, 2 * SPACING, 4 * SPACING, 6 * SPACING, 4 ] );
		expect( calls.filter( call => call[ 0 ] === 'arc' ).map( call => call.slice( 1, 3 ) ) ).toEqual( [ [ 3 * SPACING, 6 * SPACING ], [ 7 * SPACING, 5 * SPACING ] ] );
		expect( calls.filter( call => call[ 1 ] === 'fillStyle' ).map( call => call[ 2 ] ) ).toEqual( [ color( 5, 5 ), color( 0, 5 ) ] );
	} );

	it( 'draws a stack of pins as one line along the edge of the box, as wide as a bus, in the color of its pins', () =>
	{
		const { context, calls } = record();
		const stacked = { ... outline, pins : [ { x : 2, y : 0 }, { x : 2, y : 0 }, { x : 2, y : 0 }, { x : 0, y : 3 }, { x : 0, y : 3 } ] };

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline : stacked, potentials : [ 0, 0, 5, 0, 0 ] } ), { voltage : 5, current : 0 }, 1 );

		expect( calls.filter( call => call[ 0 ] === 'arc' ) ).toEqual( [] );
		expect( calls.filter( call => call[ 0 ] === 'moveTo' || call[ 0 ] === 'lineTo' ).slice( -4 ) ).toEqual( [ [ 'moveTo', 7 * SPACING, 5 * SPACING - BUS / 2 ], [ 'lineTo', 7 * SPACING, 5 * SPACING + BUS / 2 ], [ 'moveTo', 5 * SPACING - BUS / 2, 8 * SPACING ], [ 'lineTo', 5 * SPACING + BUS / 2, 8 * SPACING ] ] );
		expect( calls.filter( call => call[ 1 ] === 'lineWidth' ).slice( -2 ) ).toEqual( [ [ 'set', 'lineWidth', BUS ], [ 'set', 'lineWidth', BUS ] ] );
		expect( calls.filter( call => call[ 1 ] === 'strokeStyle' ).slice( -2 ).map( call => call[ 2 ] ) ).toEqual( [ 'rgb(0, 255, 0)', 'rgb(128, 128, 128)' ] );
	} );

	it( 'turns the line of a stack of pins with the box', () =>
	{
		const { context, calls } = record();
		const stacked = { ... outline, pins : [ { x : 2, y : 0 }, { x : 2, y : 0 } ] };

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline : stacked, rotation : 1 } ), { voltage : 5, current : 0 }, 1 );

		expect( calls.filter( call => call[ 0 ] === 'moveTo' || call[ 0 ] === 'lineTo' ).slice( -2 ) ).toEqual( [ [ 'moveTo', 5 * SPACING + BUS / 2, 7 * SPACING ], [ 'lineTo', 5 * SPACING - BUS / 2, 7 * SPACING ] ] );
	} );

	it( 'swaps the width and the height of a box turned a quarter', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline, rotation : 1 } ), { voltage : 5, current : 0 }, 1 );

		expect( calls ).toContainEqual( [ 'roundRect', 2 * SPACING, 3 * SPACING, 6 * SPACING, 4 * SPACING, 4 ] );
		expect( calls.filter( call => call[ 0 ] === 'arc' ).map( call => call.slice( 1, 3 ) ) ).toEqual( [ [ 4 * SPACING, 3 * SPACING ], [ 5 * SPACING, 7 * SPACING ] ] );
	} );

	it( 'draws each light as a filled square without a border, in its color scaled by the current of its led and dark otherwise', () =>
	{
		const { context, calls } = record();
		const lights = [ { point : { x : 0, y : -1 }, color : Color.Green }, { point : { x : 1, y : 0 }, color : Color.Red } ];

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline : { ... outline, lights }, potentials : [ 5, 0 ], lights : [ 0.0025, 0 ] } ), { voltage : 5, current : 0 }, 1 );

		expect( calls.filter( call => call[ 0 ] === 'rect' ) ).toEqual( [ [ 'rect', 90.75, 70.75, 18.5, 18.5 ], [ 'rect', 110.75, 90.75, 18.5, 18.5 ] ] );
		expect( calls.filter( call => call[ 1 ] === 'fillStyle' ).map( call => call[ 2 ] ).slice( 0, 2 ) ).toEqual( [ 'rgb(26, 110, 48)', '#1a1a1a' ] );
	} );

	it( 'widens each light by a glow of a quarter of a screen pixel, so the lights overlap when zoomed out', () =>
	{
		const fill = ( stride : number ) =>
		{
			const { context, calls } = record();

			draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline : { ... outline, lights : [ { point : { x : 0, y : 0 }, color : Color.Red } ] } } ), { voltage : 5, current : 0 }, stride );

			return calls.find( call => call[ 0 ] === 'rect' );
		};

		expect( fill( 2 ) ).toEqual( [ 'rect', 90.5, 90.5, 19, 19 ] );
		expect( fill( 4 ) ).toEqual( [ 'rect', 90, 90, SPACING, SPACING ] );
		expect( fill( 8 ) ).toEqual( [ 'rect', 89, 89, 22, 22 ] );
	} );

	it( 'fills one square for the lights that share a point with the mix of the colors of their conducting leds, each scaled by its current', () =>
	{
		const { context, calls } = record();
		const lights = [ Color.Red, Color.Green, Color.Blue ].map( color => ( { point : { x : 0, y : 0 }, color } ) );
		const fills = ( currents : number[] ) =>
		{
			const { context, calls } = record();

			draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline : { ... outline, lights }, lights : currents } ), { voltage : 5, current : 0 }, 1 );

			expect( calls.filter( call => call[ 0 ] === 'rect' ) ).toHaveLength( 1 );

			return calls.find( call => call[ 1 ] === 'fillStyle' )?.[ 2 ];
		};

		expect( fills( [ 0.005, 0.005, 0.005 ] ) ).toEqual( 'rgb(255, 255, 255)' );
		expect( fills( [ 0.005, 0.005, 0 ] ) ).toEqual( 'rgb(255, 255, 144)' );
		expect( fills( [ 0.0025, 0, 0.005 ] ) ).toEqual( 'rgb(176, 152, 255)' );
		expect( fills( [ 0, 0, 0 ] ) ).toEqual( '#1a1a1a' );
	} );

	it( 'fills the squares of the lights that follow each other in one color as one path, in their order', () =>
	{
		const { context, calls } = record();
		const lights = [ 0, 1, 2, 3 ].map( x => ( { point : { x, y : 0 }, color : Color.Red } ) );

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline : { ... outline, pins : [], lights }, lights : [ 0.005, 0.005, 0, 0.005 ] } ), { voltage : 5, current : 0 }, 1 );

		const steps = calls.filter( call => [ 'rect', 'fill' ].includes( call[ 0 ] as string ) || call[ 1 ] === 'fillStyle' ).map( call => call[ 0 ] === 'rect' ? call[ 1 ] : call[ 0 ] === 'set' ? call[ 2 ] : call[ 0 ] );

		expect( steps ).toEqual( [ 'rgb(255, 48, 48)', 90.75, 110.75, 'fill', '#1a1a1a', 130.75, 'fill', 'rgb(255, 48, 48)', 150.75, 'fill' ] );
	} );

	it( 'fills a run of squares in one color as one rectangle once the squares touch', () =>
	{
		const squares = ( stride : number, rotation : number, currents : number[] ) =>
		{
			const { context, calls } = record();
			const lights = [ 0, 1, 2, 0, 1 ].map( ( x, index ) => ( { point : { x, y : index < 3 ? 0 : 1 }, color : Color.Red } ) );

			draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline : { ... outline, pins : [], lights }, rotation, lights : currents } ), { voltage : 5, current : 0 }, stride );

			return calls.filter( call => call[ 0 ] === 'rect' ).map( call => call.slice( 1 ) );
		};

		expect( squares( 4, 0, [ 0.005, 0.005, 0.005, 0.005, 0.005 ] ) ).toEqual( [ [ 90, 90, 60, SPACING ], [ 90, 110, 40, SPACING ] ] );
		expect( squares( 8, 1, [ 0.005, 0.005, 0.005, 0.005, 0.005 ] ) ).toEqual( [ [ 89, 89, 22, 62 ], [ 69, 89, 22, 42 ] ] );
		expect( squares( 4, 0, [ 0.005, 0, 0.005, 0, 0 ] ) ).toEqual( [ [ 90, 90, SPACING, SPACING ], [ 110, 90, SPACING, SPACING ], [ 130, 90, SPACING, SPACING ], [ 90, 110, 40, SPACING ] ] );
		expect( squares( 2, 0, [ 0.005, 0.005, 0.005, 0.005, 0.005 ] ) ).toHaveLength( 5 );
	} );

	it( 'turns its lights with the box', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline : { ... outline, lights : [ { point : { x : 1, y : 0 }, color : Color.Red } ] }, rotation : 1 } ), { voltage : 5, current : 0 }, 1 );

		expect( calls.filter( call => call[ 0 ] === 'rect' ) ).toEqual( [ [ 'rect', 90.75, 110.75, 18.5, 18.5 ] ] );
	} );

	it( 'turns the lights of one outline drawn at two rotations each with its own box', () =>
	{
		const shared = { ... outline, lights : [ { point : { x : 1, y : 0 }, color : Color.Red } ] };
		const squares = ( rotation : number ) =>
		{
			const { context, calls } = record();

			draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline : shared, rotation } ), { voltage : 5, current : 0 }, 1 );

			return calls.filter( call => call[ 0 ] === 'rect' );
		};

		expect( [ squares( 0 ), squares( 1 ), squares( 0 ) ] ).toEqual( [ [ [ 'rect', 110.75, 90.75, 18.5, 18.5 ] ], [ [ 'rect', 90.75, 110.75, 18.5, 18.5 ] ], [ [ 'rect', 110.75, 90.75, 18.5, 18.5 ] ] ] );
	} );

	it( 'draws nothing before its outline is known', () =>
	{
		const { context, calls } = record();

		draw( context, element( { kind : Kind.Symbol } ), { voltage : 5, current : 0 }, 1 );

		expect( calls ).toEqual( [] );
	} );

	it( 'draws its drawing without a box, without its name and without the labels of its pins', () =>
	{
		const { context, calls } = record();
		const definition = { name : 'foo', drawing : [ { figure : Figure.Circle as const, center : { x : 0, y : 0 }, radius : 4 } ], labels : [ 'bar', '' ], layout : [], delay : 1e-9 };

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline, definition } ), { voltage : 5, current : 0 }, 1 );

		expect( calls ).toContainEqual( [ 'arc', 5 * SPACING, 5 * SPACING, 4 * SPACING / FINE, 0, 2 * Math.PI ] );
		expect( calls.some( call => call[ 0 ] === 'roundRect' ) ).toBe( false );
		expect( calls.some( call => call[ 0 ] === 'fillText' ) ).toBe( false );
	} );

	it( 'turns its drawing with the box', () =>
	{
		const { context, calls } = record();
		const definition = { name : '', drawing : [ { figure : Figure.Line as const, from : { x : 4, y : 0 }, to : { x : 8, y : 0 } } ], labels : [], layout : [], delay : 1e-9 };

		draw( context, element( { kind : Kind.Symbol, from : { x : 5, y : 5 }, to : { x : 5, y : 5 }, outline, rotation : 1, definition } ), { voltage : 5, current : 0 }, 1 );

		expect( calls ).toContainEqual( [ 'moveTo', 5 * SPACING, 6 * SPACING ] );
		expect( calls ).toContainEqual( [ 'lineTo', 5 * SPACING, 7 * SPACING ] );
	} );
} );
