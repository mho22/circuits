const SMOOTHING = 0.1;


export class Clock
{
	private remainder = 0;
	private limit = 1;
	private cost : number | null = null;

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

		const cost = milliseconds / steps;

		this.cost = this.cost === null ? cost : this.cost + ( cost - this.cost ) * SMOOTHING;

		const fit = Math.max( 1, Math.floor( this.budget / this.cost ) );

		this.limit = steps < this.limit ? Math.min( this.limit, fit ) : Math.min( 2 * steps, fit );
	}
}
