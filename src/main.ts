import init, { Circuit } from '../bindings/web/pkg/web.js';
import { fit, locate } from './board.ts';
import { Clock } from './clock.ts';
import { parse, SEPARATOR, write } from './decimal.ts';
import { decode as decodeBranches } from './decoders/branch.ts';
import { decode as decodeDrawing } from './decoders/drawing.ts';
import { decode as decodeElements } from './decoders/element.ts';
import { decode as decodeFile } from './decoders/file.ts';
import { decode as decodeLayout } from './decoders/layout.ts';
import { decode as decodeOutline } from './decoders/outline.ts';
import { Drift } from './drift.ts';
import { download } from './download.ts';
import { edit as entitle } from './editors/text.ts';
import { edit } from './editors/value.ts';
import { encode as encodeDrawing } from './encoders/drawing.ts';
import { encode } from './encoders/file.ts';
import { Color, NEXT } from './enums/color.ts';
import { Figure } from './enums/figure.ts';
import { INPUTS, Kind, NAMES, PINS, POINTS } from './enums/kind.ts';
import { extent } from './extent.ts';
import { FINE, snap } from './grid.ts';
import { grab } from './grab.ts';
import { read, shape } from './keyboard.ts';
import { junctions } from './junctions.ts';
import { leads } from './leads.ts';
import { Library } from './library.ts';
import { Maker } from './maker.ts';
import { open as fill } from './menus/blank.ts';
import { open as revise } from './menus/drawing.ts';
import { open } from './menus/part.ts';
import { open as offer } from './menus/selection.ts';
import { open as amend } from './menus/sketch.ts';
import { Palette } from './palette.ts';
import { describe } from './readout.ts';
import * as area from './renderers/area.ts';
import * as board from './renderers/board.ts';
import * as current from './renderers/current.ts';
import * as cursor from './renderers/cursor.ts';
import * as drawing from './renderers/drawing.ts';
import * as element from './renderers/element.ts';
import * as grid from './renderers/grid.ts';
import * as junction from './renderers/junction.ts';
import * as mask from './renderers/mask.ts';
import * as progress from './renderers/progress.ts';
import { measure } from './scale.ts';
import { cover, inside, span } from './selection.ts';
import { socket } from './socket.ts';
import { target } from './target.ts';
import { PART_GROUPS, PIN_GROUPS, Toolbar } from './toolbar.ts';
import { CarryTool } from './tools/carry.ts';
import { DrawTool } from './tools/draw.ts';
import { MoveTool } from './tools/move.ts';
import { PlaceTool } from './tools/place.ts';
import { SelectTool } from './tools/select.ts';
import { ShiftTool } from './tools/shift.ts';
import { toggle } from './tools/toggle.ts';
import type { Action } from './types/action.ts';
import type { Area } from './types/area.ts';
import type { Branch } from './types/branch.ts';
import type { Choice } from './types/choice.ts';
import type { Definition } from './types/definition.ts';
import type { Edit } from './types/edit.ts';
import type { Element } from './types/element.ts';
import type { Point } from './types/point.ts';
import type { Shape } from './types/shape.ts';
import { View } from './view.ts';


await init();


const STEPS_PER_FRAME = 100;
const LOAD_BUDGET = 12;
const LOAD_DELAY = 150;
const LOAD_DOTS = 16;
const KINDS = Object.keys( NAMES ).map( Number ) as Kind[];

const canvas = document.querySelector( 'canvas' )!;
const context = canvas.getContext( '2d' )!;
const status = document.querySelector( 'output' )!;
const mode = document.querySelector( '.mode' )!;
const picker = document.querySelector( 'input[type=file]' ) as HTMLInputElement;
const circuit = new Circuit();
const tool = new PlaceTool( circuit );
const mover = new MoveTool( circuit );
const selector = new SelectTool();
const drawer = new DrawTool();
const carrier = new CarryTool();
const shifter = new ShiftTool();
const slot = document.createElement( 'div' );
const toolbar = new Toolbar( document.querySelector( 'nav' )!, [ ... PART_GROUPS, ... PIN_GROUPS ], pick );
const pinbar = new Toolbar( slot, PIN_GROUPS, pick );
const library = new Library( document.querySelector( 'nav' )!, place );
const maker = new Maker( document.body, slot, make, sketch, discard, purge );
const palette = new Palette( document.querySelector( 'nav' )!, document.body, figure, unsketch );
const clock = new Clock( STEPS_PER_FRAME );
const drift = new Drift();
const view = new View();

let name = 'untitled.circuit';
let hover : Point | null = null;
let notice : string | null = null;
let previous : number | null = null;
let spaced = false;
let pressed : Point | null = null;
let marked : { area : Area, menu : HTMLElement } | null = null;
let copied = 0;
let sketching = false;
let origin : Point = { x : 0, y : 0 };
let aim : Point | null = null;
let picked : { area : Area, menu : HTMLElement } | null = null;
let loading : { name : string, since : number, progress : number } | null = null;


function select( kind : Kind | null ) : void
{
	tool.kind = kind;

	toolbar.select( kind );
	pinbar.select( kind );
	library.select( kind === Kind.Symbol ? tool.layer : null );
}

function pick( kind : Kind ) : void
{
	select( kind === tool.kind ? null : kind );
}

function allowed() : Kind[]
{
	if( maker.area ) return PINS;

	return circuit.layer === 0 ? KINDS.filter( kind => ! PINS.includes( kind ) ) : KINDS;
}

function place( layer : number ) : void
{
	const same = tool.kind === Kind.Symbol && tool.layer === layer;

	tool.layer = layer;

	select( same ? null : Kind.Symbol );
}

function define( layer : number ) : Definition
{
	return { name : circuit.name( layer ), drawing : decodeDrawing( circuit.drawing( layer ) ), labels : circuit.labels( layer ), layout : decodeLayout( circuit.layout( layer ) ), delay : circuit.delay( layer ) };
}

function outlined( item : Element ) : Element
{
	return item.kind === Kind.Symbol ? { ... item, outline : decodeOutline( circuit.outline( item.value ) ), definition : define( item.value ) } : item;
}

function labelled( elements : Element[], layer : number ) : Element[]
{
	const labels = circuit.labels( layer );
	const titles = circuit.symbolLabels( layer );
	let pin = 0;
	let symbol = 0;

	return elements.map( item => PINS.includes( item.kind ) ? { ... item, label : labels[ pin++ ] } : item.kind === Kind.Symbol ? { ... item, label : titles[ symbol++ ] } : item );
}

function captioned( elements : Element[] ) : Element[]
{
	const captions = circuit.captions();
	let index = 0;

	return elements.map( item => item.kind === Kind.Toggle ? { ... item, label : captions[ index++ ] } : item );
}

function parts() : Element[]
{
	const listening = Array.from( circuit.listening() );

	return captioned( labelled( decodeElements( circuit.elements() ), circuit.layer ) ).map( outlined ).map( item => listening.includes( item.id ) ? { ... item, listening : true } : item );
}

function layered( layer : number ) : Element[]
{
	return labelled( decodeElements( circuit.contents( layer ) ), layer );
}

async function save() : Promise<void>
{
	try { name = await download( encode( layered( 0 ), Array.from( circuit.symbols(), layer => ( { number : layer, definition : define( layer ), parts : layered( layer ) } ) ) ), name ); }

	catch( error ) { if( ( error as Error ).name !== 'AbortError' ) notice = ( error as Error ).message; }
}

async function load() : Promise<void>
{
	const [ file ] = picker.files ?? [];

	picker.value = '';

	if( ! file ) return;

	try
	{
		const contents = decodeFile( await file.text() );

		circuit.load( contents.parts, contents.labels, contents.layers, contents.names, contents.drawings, contents.layouts );

		loading = { name : file.name, since : performance.now(), progress : 0 };
	}

	catch( error ) { notice = ( error as Error ).message; }
}

function proceed() : void
{
	if( ! loading ) return;

	const start = performance.now();
	let pending = true;

	try { while( pending && performance.now() - start < LOAD_BUDGET ) pending = circuit.proceed(); }

	catch( error ) { notice = ( error as Error ).message; loading = null; return; }

	loading.progress = Math.max( loading.progress, circuit.progress );

	if( pending ) return;

	view.forget();

	name = loading.name;
	loading = null;
}

function indicate() : void
{
	if( ! loading || performance.now() - loading.since < LOAD_DELAY ) return;

	const middle = view.toGrid( innerWidth / 2, innerHeight / 2 );
	const from = { x : middle.x - Math.floor( ( LOAD_DOTS - 1 ) / 2 ), y : middle.y };
	const to = { x : from.x + LOAD_DOTS - 1, y : middle.y };

	context.setTransform( devicePixelRatio, 0, 0, devicePixelRatio, - view.x * devicePixelRatio, - view.y * devicePixelRatio );

	progress.draw( context, from, to, loading.progress );
}

function attempt( change : () => void ) : void
{
	try { change(); }

	catch( error ) { notice = ( error as Error ).message; }
}

function act( target : Element, anchor : Point, position : Point, action : Action ) : void
{
	if( action === 'move' ) { mover.start( [ target ], anchor, null ); return; }

	if( action === 'label' ) { entitle( document.body, position, target.label ?? '', '', text => attempt( () => circuit.setLabel( target.id, text.trim() ) ) ); return; }

	if( action === 'copy' ) { copied = circuit.copy( new Uint32Array( [ target.id ] ) ); return; }

	attempt( () =>
	{
		if( action === 'rotate' ) circuit.rotate( target.id );

		if( action === 'horizontal' ) circuit.mirror( target.id, false );

		if( action === 'vertical' ) circuit.mirror( target.id, true );

		if( action === 'color' ) circuit.setValue( target.id, NEXT[ target.value as Color ] );

		if( action === 'delete' ) circuit.delete( target.id );
	} );
}

function settle( area : Area, anchor : Point, choice : Choice ) : void
{
	const elements = parts();
	const ids = inside( elements, area );

	if( choice === 'move' ) { mover.start( elements.filter( item => ids.includes( item.id ) ), anchor, area ); return; }

	if( choice === 'copy' ) { copied = circuit.copy( new Uint32Array( ids ) ); return; }

	attempt( () =>
	{
		if( choice === 'symbol' ) maker.open( area );

		if( choice === 'delete' ) circuit.erase( new Uint32Array( ids ) );
	} );
}

function make( target : Area | number, name : string, shapes : Shape[], layout : Point[] ) : void
{
	if( typeof target === 'number' ) { rename( target, name, shapes, layout ); return; }

	const area = target;

	attempt( () => circuit.enclose( new Int32Array( [ area.low.x, area.low.y, area.high.x, area.high.y ] ), name, encodeDrawing( shapes ), new Int32Array( layout.flatMap( point => [ point.x, point.y ] ) ) ) );
}

function rename( layer : number, name : string, shapes : Shape[], layout : Point[] ) : void
{
	const before = define( layer );
	const same = before.name === name && JSON.stringify( before.drawing ) === JSON.stringify( shapes ) && JSON.stringify( before.layout ) === JSON.stringify( layout );

	if( ! same ) attempt( () => circuit.describe( name, encodeDrawing( shapes ), new Int32Array( layout.flatMap( point => [ point.x, point.y ] ) ) ) );

	circuit.leave();
}

function discard( steps : number, layer : number | null ) : void
{
	for( let index = 0; index < steps; index++ ) circuit.undo();

	if( layer !== null ) circuit.leave();
}

function purge( layer : number ) : void
{
	const name = circuit.name( layer ) || 'this symbol';
	const copies = Array.from( circuit.symbols() ).flatMap( inner => layered( inner ) ).concat( layered( 0 ) ).filter( item => item.kind === Kind.Symbol && item.value === layer ).length;

	if( ! confirm( `Delete ${ name } and its ${ copies } ${ copies === 1 ? 'copy' : 'copies' }?` ) ) return;

	attempt( () => circuit.purge() );
}

function enter( symbol : Element ) : void
{
	circuit.enter( symbol.id );

	const area = span( parts() );
	const middle = area ? { x : Math.floor( ( area.low.x + area.high.x ) / 2 ), y : Math.floor( ( area.low.y + area.high.y ) / 2 ) } : symbol.from;

	view.enter( circuit.layer, symbol.from, middle );
}

function follow() : void
{
	const layer = circuit.layer;

	view.follow( layer );

	if( maker.area || ( maker.layer ?? 0 ) === layer ) return;

	if( layer === 0 ) { maker.close(); return; }

	maker.edit( layer, define( layer ) );
}

function zone( elements : Element[] ) : Area | null
{
	return maker.area ?? ( maker.layer === null ? null : span( elements ) );
}

function sketch() : void
{
	const elements = parts();
	const area = zone( elements );

	if( ! area ) return;

	const ids = inside( elements, area );
	const ends = elements.filter( item => PINS.includes( item.kind ) && ids.includes( item.id ) );
	const fixed = maker.layer === null ? [] : decodeOutline( circuit.outline( maker.layer ) ).pins;

	origin = { x : Math.floor( ( area.low.x + area.high.x ) / 2 ), y : Math.floor( ( area.low.y + area.high.y ) / 2 ) };
	sketching = true;
	drawer.load( [ ... maker.drawing ], ends.map( ( item, index ) => ( { point : maker.layout[ index ] ?? fixed[ index ] ?? { x : item.from.x - origin.x, y : item.from.y - origin.y }, output : item.kind === Kind.Output, label : item.label ?? '' } ) ) );

	figure( drawer.figure );
	palette.open();
}

function unsketch() : void
{
	drawer.cancel();
	carrier.cancel();
	shifter.cancel();

	maker.drawing = drawer.shapes;
	maker.layout = drawer.pins.map( pin => pin.point );
	sketching = false;
	aim = null;

	palette.close();
}

function figure( choice : Figure ) : void
{
	drawer.cancel();
	drawer.figure = choice;

	palette.select( choice );
}

function fine( event : MouseEvent ) : Point
{
	return locate( fit( origin, view.x, view.y ), event.clientX, event.clientY );
}

function drop() : void
{
	drawer.change( drawer.shapes, carrier.pins( drawer.pins ) );

	carrier.cancel();
}

function land() : void
{
	drawer.change( shifter.shapes( drawer.shapes ), shifter.pins( drawer.pins ) );

	shifter.cancel();
}

function rework( area : Area, anchor : Point, edit : Edit ) : void
{
	const grabbed = grab( drawer.shapes, drawer.pins, area );

	if( edit === 'move' ) { shifter.start( grabbed, area, anchor ); return; }

	if( grabbed.shapes.length === 0 ) return;

	drawer.change( drawer.shapes.filter( ( _, index ) => ! grabbed.shapes.includes( index ) ), drawer.pins );
}

function inscribe( at : Point | null ) : void
{
	if( ! at ) return;

	const place = fit( origin, view.x, view.y );

	entitle( document.body, { x : place.x + at.x * place.unit, y : place.y + at.y * place.unit }, '', '', content => drawer.write( at, content ) ).parentElement!.classList.add( 'beside' );
}

function modify( event : MouseEvent ) : void
{
	const point = fine( event );
	const found = target( drawer.shapes, drawer.pins, point );

	if( ! found ) return;

	if( ! found.pin ) { drawer.change( drawer.shapes.filter( ( _, index ) => index !== found.index ), drawer.pins ); return; }

	amend( document.body, { x : event.clientX, y : event.clientY }, () => carrier.start( found.index, point ) );
}

function menu( event : MouseEvent ) : void
{
	const point = view.toGrid( event.clientX, event.clientY );
	const elements = parts();
	const targets = Array.from( circuit.elementsAt( point.x, point.y ) ).map( id => elements.find( item => item.id === id )! );
	const target = targets.find( item => item.kind !== Kind.Wire ) ?? targets[ 0 ];

	if( ! target && copied > 0 ) { fill( document.body, { x : event.clientX, y : event.clientY }, () => paste( point ) ); return; }

	if( ! target ) return;

	if( target.kind === Kind.Wire ) { circuit.delete( target.id ); return; }

	const position = { x : event.clientX, y : event.clientY };

	open( document.body, position, target, action => act( target, point, position, action ) );
}

function paste( point : Point ) : void
{
	attempt( () => circuit.paste( point.x, point.y ) );
}

function copy() : void
{
	const ids = marked?.menu.isConnected ? inside( parts(), marked.area ) : hover ? Array.from( circuit.elementsAt( hover.x, hover.y ) ).slice( 0, 1 ) : [];

	if( ids.length > 0 ) copied = circuit.copy( new Uint32Array( ids ) );
}

function report( text : string, error : boolean ) : void
{
	if( status.textContent !== text ) status.textContent = text;

	status.classList.toggle( 'error', error );
}

function present() : void
{
	const place = fit( origin, view.x, view.y );
	const draft = drawer.preview();
	const selection = selector.preview() ?? shifter.zone() ?? ( picked?.menu.isConnected ? picked.area : null );
	const shapes = shifter.shapes( drawer.shapes );
	const pins = shifter.pins( carrier.pins( drawer.pins ) );
	const icon = extent( draft ? [ ... shapes, draft ] : shapes, pins );

	context.setTransform( devicePixelRatio, 0, 0, devicePixelRatio, - view.x * devicePixelRatio, - view.y * devicePixelRatio );

	grid.draw( context, view.x, view.y, innerWidth, innerHeight, true );

	if( icon ) mask.draw( context, { low : { x : origin.x + icon.low.x, y : origin.y + icon.low.y }, high : { x : origin.x + icon.high.x, y : origin.y + icon.high.y } }, view.x, view.y, innerWidth, innerHeight );

	if( selection ) area.draw( context, { low : { x : origin.x + selection.low.x / FINE, y : origin.y + selection.low.y / FINE }, high : { x : origin.x + selection.high.x / FINE, y : origin.y + selection.high.y / FINE } } );

	context.setTransform( devicePixelRatio, 0, 0, devicePixelRatio, 0, 0 );

	board.draw( context, place, pins, aim );
	drawing.draw( context, shapes, place, 0, place.unit );

	context.globalAlpha = 0.5;

	if( draft ) drawing.draw( context, [ draft ], place, 0, place.unit );

	context.globalAlpha = 1;
}

function paint( elements : Element[], branches : Branch[], seconds : number ) : void
{
	if( sketching ) { present(); return; }

	const scale = measure( elements, branches );
	const draft = selector.preview() ? null : tool.preview();
	const previews = draft ? [ ... mover.preview(), draft ] : mover.preview();
	const selection = selector.preview() ?? mover.zone() ?? ( marked?.menu.isConnected ? marked.area : null );
	const paths = [ ... branches, ... elements.filter( item => item.kind !== Kind.Wire && ! POINTS.includes( item.kind ) && ! PINS.includes( item.kind ) ).flatMap( leads ) ];
	const distances = drift.advance( paths, scale.current, seconds );

	context.setTransform( devicePixelRatio, 0, 0, devicePixelRatio, - view.x * devicePixelRatio, - view.y * devicePixelRatio );

	grid.draw( context, view.x, view.y, innerWidth, innerHeight, false );

	elements.forEach( item => element.draw( context, item, scale ) );

	junctions( elements ).forEach( item => junction.draw( context, item, scale ) );

	paths.forEach( ( path, index ) => current.draw( context, path.from, path.to, path.current, scale.current, distances[ index ], current.COLOR ) );

	context.globalAlpha = 0.5;

	previews.forEach( item => element.draw( context, outlined( item ), { voltage : 0, current : 0 } ) );

	context.globalAlpha = 1;

	if( selection ) area.draw( context, selection );

	const zoned = zone( elements );

	if( zoned )
	{
		mask.draw( context, zoned, view.x, view.y, innerWidth, innerHeight );

		if( maker.area ) area.draw( context, zoned );
	}

	cursor.draw( context, tool.kind === null ? null : hover );
}

function frame( time : number ) : void
{
	const seconds = previous === null ? 0 : ( time - previous ) / 1000;

	previous = time;

	proceed();

	circuit.advance( clock.tick( seconds, circuit.rate ) );

	follow();

	const elements = parts();
	const branches = decodeBranches( circuit.branches() );

	paint( elements, branches, seconds );

	indicate();

	library.update( Array.from( circuit.symbols(), layer => [ layer, circuit.name( layer ) ] ) );

	const kinds = allowed();

	toolbar.limit( maker.area ? [] : kinds );

	if( tool.kind !== null && ! kinds.includes( tool.kind ) ) select( kinds[ 0 ] );

	canvas.classList.toggle( 'idle', tool.kind === null && ! sketching );

	const engine = circuit.mode;

	if( mode.textContent !== engine ) mode.textContent = engine;

	const error = circuit.error();
	const ids = hover ? Array.from( circuit.elementsAt( hover.x, hover.y ) ) : [];
	const plugged = elements.find( item => ids.includes( item.id ) && hover && socket( item, hover ) >= 0 );
	const hovered = plugged ?? elements.find( item => item.id === ids[ 0 ] );
	const branch = hover ? circuit.branchAt( hover.x, hover.y ) : undefined;

	report( error ?? notice ?? ( hovered ? describe( hovered, branch === undefined ? undefined : branches[ branch ], hover ) : '' ), error !== undefined || notice !== null );

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

	if( sketching && event.button === 0 && carrier.index !== null ) { drop(); return; }

	if( sketching && event.button === 0 && shifter.grab ) { land(); return; }

	if( sketching && event.button === 2 && aim ) { canvas.setPointerCapture( event.pointerId ); selector.press( aim ); return; }

	if( sketching && event.button === 0 ) { canvas.setPointerCapture( event.pointerId ); drawer.press( fine( event ) ); }

	if( sketching ) return;

	if( event.button === 1 || ( event.button === 0 && spaced ) )
	{
		event.preventDefault();

		canvas.setPointerCapture( event.pointerId );

		view.press( { x : event.clientX, y : event.clientY } );

		return;
	}

	if( maker.area && event.button !== 0 ) return;

	if( event.button === 2 ) { canvas.setPointerCapture( event.pointerId ); selector.press( point ); return; }

	if( event.button === 0 && mover.elements.length > 0 ) { attempt( () => mover.drop() ); return; }

	if( event.button === 0 ) pressed = point;

	if( event.button === 0 && event.detail < 2 )
	{
		canvas.setPointerCapture( event.pointerId );

		tool.press( point );
	}
} );

canvas.addEventListener( 'pointermove', ( event : PointerEvent ) =>
{
	if( sketching ) { aim = snap( fine( event ) ); carrier.move( aim ); shifter.move( aim ); selector.move( aim ); drawer.move( aim ); return; }

	hover = view.toGrid( event.clientX, event.clientY );

	if( view.move( { x : event.clientX, y : event.clientY } ) ) return;

	mover.move( hover );
	selector.move( hover );
	tool.move( hover );
} );

canvas.addEventListener( 'pointerup', ( event : PointerEvent ) =>
{
	const point = view.toGrid( event.clientX, event.clientY );
	const same = pressed && pressed.x === point.x && pressed.y === point.y;
	const selection = selector.release();

	pressed = null;

	view.release();

	const draft = tool.preview();
	const added = tool.release();
	const clicked = same && ! added;

	if( added && draft && maker.area ) { maker.record(); maker.area = cover( maker.area, [ draft.from, draft.to ] ); }

	if( sketching && selection && aim ) { const anchor = aim; picked = { area : selection, menu : revise( document.body, { x : event.clientX, y : event.clientY }, edit => rework( selection, anchor, edit ) ) }; return; }

	if( sketching ) { if( event.button === 0 ) inscribe( drawer.release( fine( event ) ) ); if( event.button === 2 ) modify( event ); return; }

	if( maker.area ) { if( clicked && toggle( circuit, point ) ) maker.record(); return; }

	if( selection ) marked = { area : selection, menu : offer( document.body, { x : event.clientX, y : event.clientY }, choice => settle( selection, point, choice ) ) };

	if( ! selection && event.button === 2 ) menu( event );

	if( clicked ) toggle( circuit, point );
} );

canvas.addEventListener( 'wheel', ( event : WheelEvent ) =>
{
	event.preventDefault();

	if( event.ctrlKey || sketching ) return;

	view.scroll( event.deltaX, event.deltaY, event.deltaMode );

	hover = view.toGrid( event.clientX, event.clientY );
}, { passive : false } );

canvas.addEventListener( 'pointerleave', () => hover = null );

canvas.addEventListener( 'dblclick', ( event : MouseEvent ) =>
{
	const point = view.toGrid( event.clientX, event.clientY );
	const ids = Array.from( circuit.elementsAt( point.x, point.y ) );
	const elements = parts();
	const items = ids.map( id => elements.find( item => item.id === id )! );
	const symbol = items.find( item => item.kind === Kind.Symbol );
	const target = items.find( item => INPUTS[ item.kind ] );

	if( maker.area ) return;

	if( symbol ) { enter( symbol ); return; }

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

	if( event.button !== 2 ) menu( event );
} );

addEventListener( 'keydown', ( event : KeyboardEvent ) =>
{
	if( event.target instanceof HTMLInputElement ) return;

	if( sketching )
	{
		if( event.key === 'Escape' && carrier.index !== null ) { carrier.cancel(); return; }

		if( event.key === 'Escape' && shifter.grab ) { shifter.cancel(); return; }

		if( event.key === 'Escape' && ! drawer.cancel() ) unsketch();

		const choice = shape( event );

		if( choice !== undefined ) figure( choice );

		if( read( event ) === 'undo' ) { event.preventDefault(); drawer.undo(); }

		if( read( event ) === 'redo' ) { event.preventDefault(); drawer.redo(); }

		return;
	}

	if( event.key === ' ' ) { event.preventDefault(); spaced = true; canvas.classList.add( 'grab' ); return; }

	if( event.key === 'Escape' && mover.elements.length > 0 ) { mover.cancel(); return; }

	if( event.key === 'Escape' && maker.area ) { maker.cancel(); return; }

	if( event.key === 'Escape' ) { select( null ); return; }

	const command = read( event );

	if( command === undefined ) return;

	event.preventDefault();

	if( command === 'undo' && maker.undo() ) { circuit.undo(); return; }

	if( command === 'redo' && maker.redo() ) { circuit.redo(); return; }

	if( maker.area && ( command === 'undo' || command === 'redo' ) ) return;

	if( command === 'undo' ) { circuit.undo(); return; }

	if( command === 'redo' ) { circuit.redo(); return; }

	if( command === 'copy' ) { copy(); return; }

	if( command === 'paste' ) { if( hover && ! maker.area ) paste( hover ); return; }

	if( command === 'save' ) { save(); return; }

	if( command === 'open' ) { picker.click(); return; }

	if( allowed().includes( command ) ) pick( command );
} );

addEventListener( 'keyup', ( event : KeyboardEvent ) =>
{
	if( event.key !== ' ' ) return;

	spaced = false;

	canvas.classList.remove( 'grab' );
} );

addEventListener( 'resize', resize );


select( null );
resize();
requestAnimationFrame( frame );
