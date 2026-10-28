import { Kind, PINS } from './enums/kind.ts';
import { seen } from './sight.ts';
import type { Area } from './types/area.ts';
import type { Scene } from './types/scene.ts';


const CAPTIONED = [ Kind.Symbol, Kind.Toggle, Kind.Slider, ... PINS ];


type Found = Pick<Scene, 'junctions'> & { indexes : number[] };


export class Watch
{
	private scene : Scene | null = null;

	private key = '';

	private found : Found = { indexes : [], junctions : [] };


	look( scene : Scene, range : Area ) : Found
	{
		const key = `${ range.low.x },${ range.low.y },${ range.high.x },${ range.high.y }`;

		if( scene === this.scene && key === this.key ) return this.found;

		const indexes : number[] = [];

		scene.elements.forEach( ( item, index ) => { if( CAPTIONED.includes( item.kind ) || seen( range, item.from, item.to ) ) indexes.push( index ); } );

		this.scene = scene;
		this.key = key;
		this.found = { indexes, junctions : scene.junctions.filter( ( { point } ) => seen( range, point, point ) ) };

		return this.found;
	}
}
