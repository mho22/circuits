import type { Point } from './types/point.ts';


export function turn( point : Point, rotation : number ) : Point
{
	let turned = point;

	for( let step = 0; step < rotation; step++ ) turned = { x : - turned.y, y : turned.x };

	return turned;
}
