import { Color, NAMES as COLORS } from '../enums/color.ts';
import { Figure, NAMES as FIGURES } from '../enums/figure.ts';
import { FIELDS, Kind, NAMES, PINS } from '../enums/kind.ts';
import type { Element } from '../types/element.ts';
import type { Layer } from '../types/layer.ts';
import type { Point } from '../types/point.ts';
import type { Shape } from '../types/shape.ts';


function detail( element : Element ) : string
{
	const field = FIELDS[ element.kind ];

	if( element.kind === Kind.Switch ) return `, "closed" : ${ element.value === 1 }`;

	if( element.kind === Kind.Symbol ) return `, "layer" : ${ element.value }, "rotation" : ${ element.rotation }${ element.label ? `, "label" : ${ JSON.stringify( element.label ) }` : '' }`;

	if( element.kind === Kind.Toggle ) return element.value ? ', "level" : 1' : '';

	if( element.kind === Kind.Clock ) return `, "hertz" : ${ element.value }${ element.on === false ? ', "level" : 0' : '' }`;

	if( element.kind === Kind.Led ) return element.value ? `, "color" : "${ COLORS[ element.value as Color ].toLowerCase() }"` : '';

	if( PINS.includes( element.kind ) ) return [ element.label ? `, "label" : ${ JSON.stringify( element.label ) }` : '', element.value ? ', "level" : 1' : '' ].join( '' );

	if( field ) return `, "${ field }" : ${ element.value }`;

	return element.mirrored ? ', "mirrored" : true' : '';
}

function part( element : Element ) : string
{
	return `{ "kind" : "${ NAMES[ element.kind ].toLowerCase() }"${ detail( element ) }, "from" : [ ${ element.from.x }, ${ element.from.y } ], "to" : [ ${ element.to.x }, ${ element.to.y } ] }`;
}

function list( elements : Element[], indent : string ) : string
{
	if( elements.length === 0 ) return '[]';

	return `[\n${ indent }\t${ elements.map( part ).join( `,\n${ indent }\t` ) }\n${ indent }]`;
}


function coordinates( point : Point ) : string
{
	return `[ ${ point.x }, ${ point.y } ]`;
}

function figure( shape : Shape ) : string
{
	if( shape.figure === Figure.Text ) return `"at" : ${ coordinates( shape.at ) }, "content" : ${ JSON.stringify( shape.content ) }`;

	if( shape.figure === Figure.Circle ) return `"center" : ${ coordinates( shape.center ) }, "radius" : ${ shape.radius }`;

	if( shape.figure === Figure.Curve ) return `"from" : ${ coordinates( shape.from ) }, "through" : ${ coordinates( shape.through ) }, "to" : ${ coordinates( shape.to ) }`;

	return `"from" : ${ coordinates( shape.from ) }, "to" : ${ coordinates( shape.to ) }`;
}

function heading( layer : Layer ) : string
{
	const { name, drawing, layout } = layer.definition;
	const shapes = drawing.map( shape => `{ "shape" : "${ FIGURES[ shape.figure ].toLowerCase() }", ${ figure( shape ) } }` );
	const lines = [ `"layer" : ${ layer.number }`, name ? `"name" : ${ JSON.stringify( name ) }` : '', shapes.length === 0 ? '' : `"drawing" : [ ${ shapes.join( ', ' ) } ]`, layout.length === 0 ? '' : `"layout" : [ ${ layout.map( coordinates ).join( ', ' ) } ]` ];

	return lines.filter( Boolean ).map( line => `\t\t\t${ line },\n` ).join( '' );
}


export function encode( elements : Element[], layers : Layer[] = [] ) : string
{
	const definitions = layers.map( layer => `\t\t{\n${ heading( layer ) }\t\t\t"parts" : ${ list( layer.parts, '\t\t\t' ) }\n\t\t}` );
	const tail = definitions.length === 0 ? '' : `,\n\t"symbols" : [\n${ definitions.join( ',\n' ) }\n\t]`;

	return `{\n\t"version" : 1,\n\t"parts" : ${ list( elements, '\t' ) }${ tail }\n}\n`;
}
