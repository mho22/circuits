import { FIELDS, Kind, NAMES } from '../enums/kind.ts';
import type { Element } from '../types/element.ts';


function part( element : Element ) : string
{
	const field = FIELDS[ element.kind ];
	const detail = element.kind === Kind.Switch ? `, "closed" : ${ element.value === 1 }` : field ? `, "${ field }" : ${ element.value }` : element.mirrored ? ', "mirrored" : true' : '';

	return `{ "kind" : "${ NAMES[ element.kind ].toLowerCase() }"${ detail }, "from" : [ ${ element.from.x }, ${ element.from.y } ], "to" : [ ${ element.to.x }, ${ element.to.y } ] }`;
}


export function encode( elements : Element[] ) : string
{
	if( elements.length === 0 ) return '{\n\t"version" : 1,\n\t"parts" : []\n}\n';

	return `{\n\t"version" : 1,\n\t"parts" : [\n\t\t${ elements.map( part ).join( ',\n\t\t' ) }\n\t]\n}\n`;
}
