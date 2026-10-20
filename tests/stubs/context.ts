export function record() : { context : CanvasRenderingContext2D, calls : unknown[][] }
{
	const calls : unknown[][] = [];

	const context : object = new Proxy( {}, {
		get : ( _, name ) => ( ... args : unknown[] ) => { calls.push( [ name, ... args ] ); return context; },
		set : ( _, name, value ) => { calls.push( [ 'set', name, value ] ); return true; },
	} );

	return { context : context as CanvasRenderingContext2D, calls };
}
