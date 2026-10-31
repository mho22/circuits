import { Figure } from './enums/figure.ts';
import { Kind } from './enums/kind.ts';
import { FINE } from './grid.ts';
import { hit } from './hit.ts';
import type { Element } from './types/element.ts';
import type { Point } from './types/point.ts';


const REACH = 1;


function along( wire : Element, point : Point ) : boolean
{
	const from = { x : wire.from.x * FINE, y : wire.from.y * FINE };
	const to = { x : wire.to.x * FINE, y : wire.to.y * FINE };
	const away = ( end : Point ) => Math.hypot( point.x - end.x, point.y - end.y ) > REACH;

	return hit( { figure : Figure.Line, from, to }, point ) && away( from ) && away( to );
}


export function under( elements : Element[], point : Point ) : Element | undefined
{
	return elements.find( item => item.kind === Kind.Wire && along( item, point ) ) ?? elements.find( item => item.kind !== Kind.Wire ) ?? elements[ 0 ];
}
