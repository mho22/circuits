import init, { Circuit } from '../bindings/web/pkg/web.js';
import { Clock } from './clock.ts';
import { parse, write } from './decimal.ts';
import { decode as decodeBranches } from './decoders/branch.ts';
import { decode as decodeElements } from './decoders/element.ts';
import { decode as decodeFile } from './decoders/file.ts';
import { Drift } from './drift.ts';
import { download } from './download.ts';
import { edit } from './editors/value.ts';
import { encode } from './encoders/file.ts';
import { INPUTS, Kind, POINTS } from './enums/kind.ts';
import { read } from './keyboard.ts';
import { leads } from './leads.ts';
import { open } from './menus/part.ts';
import { describe } from './readout.ts';
import * as current from './renderers/current.ts';
import * as cursor from './renderers/cursor.ts';
import * as element from './renderers/element.ts';
import * as grid from './renderers/grid.ts';
import { measure } from './scale.ts';
import { Toolbar } from './toolbar.ts';
import { MoveTool } from './tools/move.ts';
import { PlaceTool } from './tools/place.ts';
import { toggle } from './tools/toggle.ts';
import type { Action } from './types/action.ts';
import type { Element } from './types/element.ts';
import type { Point } from './types/point.ts';
import { View } from './view.ts';


await init();


const STEPS_PER_SECOND = 1000;
const STEPS_PER_FRAME = 100;
const SEPARATOR = ( 1.5 ).toLocaleString().charAt( 1 );

const canvas = document.querySelector( 'canvas' )!;
const context = canvas.getContext( '2d' )!;
const status = document.querySelector( 'output' )!;
const picker = document.querySelector( 'input[type=file]' ) as HTMLInputElement;
const circuit = new Circuit();
const tool = new PlaceTool( circuit );
const mover = new MoveTool( circuit );
const toolbar = new Toolbar( document.querySelector( 'nav' )!, select );
const clock = new Clock( STEPS_PER_SECOND, STEPS_PER_FRAME );
const drift = new Drift();
const view = new View();

let name = 'untitled.circuit';
let hover : Point | null = null;
let notice : string | null = null;
let previous : number | null = null;
let spaced = false;
let pressed : Point | null = null;


function select( kind : Kind ) : void
{
	tool.kind = kind;

	toolbar.select( kind );
}

async function save() : Promise<void>
{
	try { name = await download( encode( decodeElements( circuit.elements() ) ), name ); }

	catch( error ) { if( ( error as Error ).name !== 'AbortError' ) notice = ( error as Error ).message; }
}

async function load() : Promise<void>
{
	const [ file ] = picker.files ?? [];

	picker.value = '';

	if( ! file ) return;

	try { circuit.open( decodeFile( await file.text() ) ); name = file.name; }

	catch( error ) { notice = ( error as Error ).message; }
}

function attempt( change : () => void ) : void
{
	try { change(); }

	catch( error ) { notice = ( error as Error ).message; }
}

function act( target : Element, anchor : Point, action : Action ) : void
{
	if( action === 'move' ) { mover.start( target, anchor ); return; }

	attempt( () =>
	{
		if( action === 'rotate' ) circuit.rotate( target.id );

		if( action === 'horizontal' ) circuit.mirror( target.id, false );

		if( action === 'vertical' ) circuit.mirror( target.id, true );

		if( action === 'delete' ) circuit.delete( target.id );
	} );
}

function menu( event : MouseEvent ) : void
{
	const point = view.toGrid( event.clientX, event.clientY );
	const elements = decodeElements( circuit.elements() );
	const targets = Array.from( circuit.elementsAt( point.x, point.y ) ).map( id => elements.find( item => item.id === id )! );
	const target = targets.find( item => item.kind !== Kind.Wire ) ?? targets[ 0 ];

	if( ! target ) return;

	if( target.kind === Kind.Wire ) { circuit.delete( target.id ); return; }

	open( document.body, { x : event.clientX, y : event.clientY }, target.kind, action => act( target, point, action ) );
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
	const preview = mover.preview() ?? tool.preview();
	const paths = [ ... branches, ... elements.filter( item => item.kind !== Kind.Wire && ! POINTS.includes( item.kind ) ).flatMap( leads ) ];
	const distances = drift.advance( paths, scale.current, seconds );

	context.setTransform( devicePixelRatio, 0, 0, devicePixelRatio, - view.x * devicePixelRatio, - view.y * devicePixelRatio );

	grid.draw( context, view.x, view.y, innerWidth, innerHeight );

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
}


canvas.addEventListener( 'pointerdown', ( event : PointerEvent ) =>
{
	const point = view.toGrid( event.clientX, event.clientY );

	notice = null;

	if( event.button === 1 || ( event.button === 0 && spaced ) )
	{
		event.preventDefault();

		canvas.setPointerCapture( event.pointerId );

		view.press( { x : event.clientX, y : event.clientY } );

		return;
	}

	if( event.button === 0 && mover.element ) { attempt( () => mover.drop() ); return; }

	if( event.button === 0 ) pressed = point;

	if( event.button === 0 && event.detail < 2 )
	{
		canvas.setPointerCapture( event.pointerId );

		tool.press( point );
	}
} );

canvas.addEventListener( 'pointermove', ( event : PointerEvent ) =>
{
	hover = view.toGrid( event.clientX, event.clientY );

	if( view.move( { x : event.clientX, y : event.clientY } ) ) return;

	mover.move( hover );
	tool.move( hover );
} );

canvas.addEventListener( 'pointerup', ( event : PointerEvent ) =>
{
	const point = view.toGrid( event.clientX, event.clientY );
	const clicked = pressed && pressed.x === point.x && pressed.y === point.y && ! POINTS.includes( tool.kind );

	pressed = null;

	view.release();
	tool.release();

	if( clicked ) toggle( circuit, point );
} );

canvas.addEventListener( 'wheel', ( event : WheelEvent ) =>
{
	event.preventDefault();

	if( event.ctrlKey ) return;

	view.scroll( event.deltaX, event.deltaY, event.deltaMode );

	hover = view.toGrid( event.clientX, event.clientY );
}, { passive : false } );

canvas.addEventListener( 'pointerleave', () => hover = null );

canvas.addEventListener( 'dblclick', ( event : MouseEvent ) =>
{
	const point = view.toGrid( event.clientX, event.clientY );
	const ids = Array.from( circuit.elementsAt( point.x, point.y ) );
	const elements = decodeElements( circuit.elements() );
	const target = ids.map( id => elements.find( item => item.id === id )! ).find( item => INPUTS[ item.kind ] );

	if( ! target ) return;

	const [ unit, scale ] = INPUTS[ target.kind ]!;

	edit( document.body, { x : event.clientX, y : event.clientY }, write( target.value / scale, SEPARATOR ), unit, text =>
	{
		const value = parse( text ) * scale;

		if( Number.isNaN( value ) ) { notice = `"${ text }" is not a value. Write a number such as 4700 or 4,7.`; return; }

		try { circuit.setValue( target.id, value ); }

		catch( error ) { notice = ( error as Error ).message; }
	} );
} );

document.querySelector( 'button.center' )!.addEventListener( 'click', () => view.center() );

document.querySelector( 'button.save' )!.addEventListener( 'click', save );

document.querySelector( 'button.load' )!.addEventListener( 'click', () => picker.click() );

picker.addEventListener( 'change', load );

canvas.addEventListener( 'contextmenu', ( event : MouseEvent ) =>
{
	event.preventDefault();

	menu( event );
} );

addEventListener( 'keydown', ( event : KeyboardEvent ) =>
{
	if( event.target instanceof HTMLInputElement ) return;

	if( event.key === ' ' ) { event.preventDefault(); spaced = true; canvas.classList.add( 'grab' ); return; }

	if( event.key === 'Escape' ) { mover.cancel(); return; }

	const command = read( event );

	if( command === undefined ) return;

	event.preventDefault();

	if( command === 'undo' ) { circuit.undo(); return; }

	if( command === 'redo' ) { circuit.redo(); return; }

	if( command === 'save' ) { save(); return; }

	if( command === 'open' ) { picker.click(); return; }

	select( command );
} );

addEventListener( 'keyup', ( event : KeyboardEvent ) =>
{
	if( event.key !== ' ' ) return;

	spaced = false;

	canvas.classList.remove( 'grab' );
} );

addEventListener( 'resize', resize );


select( Kind.Wire );
resize();
requestAnimationFrame( frame );
