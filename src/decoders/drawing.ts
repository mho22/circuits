import { Figure } from '../enums/figure.ts';
import type { Shape } from '../types/shape.ts';


export function decode( data : Int32Array ) : Shape[]
{
	const shapes : Shape[] = [];
	let index = 0;

	while( index < data.length )
	{
		const figure = data[ index ];

		if( figure === Figure.Circle )
		{
			const [ x, y, radius ] = data.subarray( index + 1, index + 4 );

			shapes.push( { figure, center : { x, y }, radius } );
			index += 4;

			continue;
		}

		if( figure === Figure.Text )
		{
			const [ x, y, length ] = data.subarray( index + 1, index + 4 );

			shapes.push( { figure, at : { x, y }, content : String.fromCodePoint( ... data.subarray( index + 4, index + 4 + length ) ) } );
			index += 4 + length;

			continue;
		}

		if( figure === Figure.Curve )
		{
			const [ fromX, fromY, throughX, throughY, toX, toY ] = data.subarray( index + 1, index + 7 );

			shapes.push( { figure, from : { x : fromX, y : fromY }, through : { x : throughX, y : throughY }, to : { x : toX, y : toY } } );
			index += 7;

			continue;
		}

		const [ fromX, fromY, toX, toY ] = data.subarray( index + 1, index + 5 );

		shapes.push( { figure : figure as Figure.Line, from : { x : fromX, y : fromY }, to : { x : toX, y : toY } } );
		index += 5;
	}

	return shapes;
}
