import type { Branch } from './types/branch.ts';


const SPEED = 80;
const SPAN = 2 ** 32;
const WIDTH = 4;


type Path = Pick<Branch, 'from' | 'to' | 'current'>;


function layout( paths : Path[] ) : number[]
{
	const shape : number[] = [];

	for( const path of paths ) shape.push( path.from.x, path.from.y, path.to.x, path.to.y );

	return shape;
}

function lasts( shape : number[] ) : Map<number, Map<number, number>>
{
	const found = new Map<number, Map<number, number>>();

	for( let index = 0; index < shape.length / WIDTH; index++ )
	{
		const at = index * WIDTH;
		const from = shape[ at ] * SPAN + shape[ at + 1 ];
		const targets = found.get( from ) ?? new Map<number, number>();

		targets.set( shape[ at + 2 ] * SPAN + shape[ at + 3 ], index );
		found.set( from, targets );
	}

	return found;
}

function sources( shape : number[], found : Map<number, Map<number, number>> ) : number[]
{
	const indexes : number[] = [];

	for( let at = 0; at < shape.length; at += WIDTH ) indexes.push( found.get( shape[ at ] * SPAN + shape[ at + 1 ] )?.get( shape[ at + 2 ] * SPAN + shape[ at + 3 ] ) ?? -1 );

	return indexes;
}


export class Drift
{
	private shape : number[] = [];

	private own : number[] = [];

	private distances : number[] = [];


	advance( paths : Path[], scale : number, seconds : number ) : number[]
	{
		const shape = layout( paths );
		const same = shape.length === this.shape.length && shape.every( ( value, index ) => value === this.shape[ index ] );
		const before = same ? this.own : sources( shape, lasts( this.shape ) );

		if( ! same ) { this.shape = shape; this.own = sources( shape, lasts( shape ) ); }

		const moved = paths.map( ( path, index ) =>
		{
			const speed = scale > 0 && Number.isFinite( path.current ) ? SPEED * path.current / scale : 0;

			return ( before[ index ] < 0 ? 0 : this.distances[ before[ index ] ] ) + speed * seconds;
		} );

		this.distances = moved;

		return moved;
	}
}
