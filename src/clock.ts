const SMOOTHING = 0.1;


export class Clock
{
	private remainder = 0;
	private limit = 1;

	speed = 1;


	constructor( private budget : number ) {}


	tick( seconds : number, rate : number ) : number
	{
		const exact = this.remainder + seconds * rate;
		const owed = Math.floor( exact );
		const steps = Math.min( owed, this.limit );

		this.remainder = steps === owed ? exact - steps : 0;

		if( owed > 0 ) this.speed += ( steps / owed - this.speed ) * SMOOTHING;

		return steps;
	}

	measure( steps : number, milliseconds : number ) : void
	{
		if( steps === 0 ) return;

		const fit = Math.max( 1, Math.floor( steps * this.budget / milliseconds ) );

		this.limit = steps < this.limit ? Math.min( this.limit, fit ) : Math.min( 2 * steps, fit );
	}
}
