export function record() : { context : CanvasRenderingContext2D, calls : unknown[][] }
{
	const calls : unknown[][] = [];

	const context = new Proxy( {}, {
		get : ( _, name ) => ( ... args : unknown[] ) => calls.push( [ name, ... args ] ),
		set : ( _, name, value ) => calls.push( [ 'set', name, value ] ) > 0,
	} );

	return { context : context as CanvasRenderingContext2D, calls };
}
