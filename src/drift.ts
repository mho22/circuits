import type { Branch } from './types/branch.ts';


const SPEED = 80;


export class Drift
{
	private distances = new Map<string, number>();


	advance( paths : Pick<Branch, 'from' | 'to' | 'current'>[], scale : number, seconds : number ) : number[]
	{
		const distances = new Map<string, number>();

		const moved = paths.map( path =>
		{
			const key = `${ path.from.x },${ path.from.y },${ path.to.x },${ path.to.y }`;
			const speed = scale > 0 && Number.isFinite( path.current ) ? SPEED * path.current / scale : 0;
			const distance = ( this.distances.get( key ) ?? 0 ) + speed * seconds;

			distances.set( key, distance );

			return distance;
		} );

		this.distances = distances;

		return moved;
	}
}
