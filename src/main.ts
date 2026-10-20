import init, { Circuit } from '../bindings/web/pkg/web.js';
import { Clock } from './clock.ts';
import { decode as decodeBranches } from './decoders/branch.ts';
import { decode as decodeElements } from './decoders/element.ts';
import { Drift } from './drift.ts';
import { edit } from './editors/value.ts';
import { Kind, UNITS } from './enums/kind.ts';
import { toGrid } from './grid.ts';
import { describe } from './readout.ts';
import * as current from './renderers/current.ts';
import * as cursor from './renderers/cursor.ts';
import * as element from './renderers/element.ts';
import * as grid from './renderers/grid.ts';
import { measure } from './scale.ts';
import { format, parse } from './si.ts';
import { erase } from './tools/eraser.ts';
import { PlaceTool } from './tools/place.ts';
import type { Point } from './types/point.ts';


await init();


const SHORTCUTS : Record<string, Kind> = { w : Kind.Wire, r : Kind.Resistor, b : Kind.Battery, g : Kind.Ground, c : Kind.Capacitor, l : Kind.Inductor, s : Kind.Switch };
const STEPS_PER_SECOND = 1000;
const STEPS_PER_FRAME = 100;

const canvas = document.querySelector( 'canvas' )!;
const context = canvas.getContext( '2d' )!;
const status = document.querySelector( 'output' )!;
const buttons = document.querySelectorAll<HTMLButtonElement>( 'nav button' );
const circuit = new Circuit();
const tool = new PlaceTool( circuit );
const clock = new Clock( STEPS_PER_SECOND, STEPS_PER_FRAME );
const drift = new Drift();

let hover : Point | null = null;
let notice : string | null = null;
let previous : number | null = null;


function select( kind : Kind ) : void
{
	tool.kind = kind;

	buttons.forEach( button => button.classList.toggle( 'active', Number( button.dataset.kind ) === kind ) );
}

function report( text : string, error : boolean ) : void
{
	if( status.textContent !== text ) status.textContent = text;

	status.classList.toggle( 'error', error );
}

function frame( time : number ) : void
{
	const seconds = previous === null ? 0 : ( time - previous ) / 1000;

	previous = time;

	circuit.advance( clock.tick( seconds ) );

	const elements = decodeElements( circuit.elements() );
	const branches = decodeBranches( circuit.branches() );
	const scale = measure( elements, branches );
	const preview = tool.preview();
	const paths = [ ... branches, ... elements.filter( item => item.kind !== Kind.Wire && item.kind !== Kind.Ground ) ];
	const distances = drift.advance( paths, scale.current, seconds );

	grid.draw( context, innerWidth, innerHeight );

	elements.forEach( item => element.draw( context, item, scale ) );

	paths.forEach( ( path, index ) => current.draw( context, path.from, path.to, path.current, scale.current, distances[ index ] ) );

	if( preview )
	{
		context.globalAlpha = 0.5;

		element.draw( context, preview, { voltage : 0, current : 0 } );

		context.globalAlpha = 1;
	}

	cursor.draw( context, hover );

	const error = circuit.error();
	const [ id ] = hover ? circuit.elementsAt( hover.x, hover.y ) : [];
	const hovered = elements.find( item => item.id === id );
	const branch = hover ? circuit.branchAt( hover.x, hover.y ) : undefined;

	report( error ?? notice ?? ( hovered ? describe( hovered, branch === undefined ? undefined : branches[ branch ] ) : '' ), error !== undefined || notice !== null );

	requestAnimationFrame( frame );
}

function resize() : void
{
	canvas.width = innerWidth * devicePixelRatio;
	canvas.height = innerHeight * devicePixelRatio;

	context.setTransform( devicePixelRatio, 0, 0, devicePixelRatio, 0, 0 );
}


canvas.addEventListener( 'pointerdown', ( event : PointerEvent ) =>
{
	const point = toGrid( event.clientX, event.clientY );

	notice = null;

	if( event.button === 2 ) erase( circuit, point );

	if( event.button === 0 && event.detail < 2 )
	{
		canvas.setPointerCapture( event.pointerId );

		tool.press( point );
	}
} );

canvas.addEventListener( 'pointermove', ( event : PointerEvent ) =>
{
	hover = toGrid( event.clientX, event.clientY );

	tool.move( hover );
} );

canvas.addEventListener( 'pointerup', () => tool.release() );

canvas.addEventListener( 'pointerleave', () => hover = null );

canvas.addEventListener( 'dblclick', ( event : MouseEvent ) =>
{
	const point = toGrid( event.clientX, event.clientY );
	const ids = Array.from( circuit.elementsAt( point.x, point.y ) );
	const elements = decodeElements( circuit.elements() );
	const target = ids.map( id => elements.find( item => item.id === id )! ).find( item => UNITS[ item.kind ] || item.kind === Kind.Switch );

	if( ! target ) return;

	if( target.kind === Kind.Switch ) { circuit.setClosed( target.id, ! target.value ); return; }

	const unit = UNITS[ target.kind ]!;

	edit( document.body, { x : event.clientX, y : event.clientY }, format( target.value, unit ), text =>
	{
		const value = parse( text );

		if( Number.isNaN( value ) ) { notice = `"${ text }" is not a value. Write a number such as 4.7k or 9.`; return; }

		try { circuit.setValue( target.id, value ); }

		catch( error ) { notice = ( error as Error ).message; }
	} );
} );

canvas.addEventListener( 'contextmenu', ( event : MouseEvent ) => event.preventDefault() );

buttons.forEach( button => button.addEventListener( 'click', () => select( Number( button.dataset.kind ) ) ) );

addEventListener( 'keydown', ( event : KeyboardEvent ) =>
{
	if( event.target instanceof HTMLInputElement ) return;

	const kind = SHORTCUTS[ event.key.toLowerCase() ];

	if( kind === undefined ) return;

	select( kind );
} );

addEventListener( 'resize', resize );


select( Kind.Wire );
resize();
requestAnimationFrame( frame );
