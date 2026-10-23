import { Figure } from '../enums/figure.ts';
import type { Shape } from '../types/shape.ts';


function row( shape : Shape ) : number[]
{
	if( shape.figure === Figure.Text ) { const codes = Array.from( shape.content, character => character.codePointAt( 0 )! ); return [ Figure.Text, shape.at.x, shape.at.y, codes.length, ... codes ]; }

	if( shape.figure === Figure.Circle ) return [ Figure.Circle, shape.center.x, shape.center.y, shape.radius ];

	if( shape.figure === Figure.Curve ) return [ Figure.Curve, shape.from.x, shape.from.y, shape.through.x, shape.through.y, shape.to.x, shape.to.y ];

	return [ shape.figure, shape.from.x, shape.from.y, shape.to.x, shape.to.y ];
}


export function encode( shapes : Shape[] ) : Int32Array
{
	return new Int32Array( shapes.flatMap( row ) );
}
