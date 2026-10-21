export class Clock
{
	private remainder = 0;


	constructor( private limit : number ) {}


	tick( seconds : number, rate : number ) : number
	{
		const exact = this.remainder + seconds * rate;
		const steps = Math.min( Math.floor( exact ), this.limit );

		this.remainder = steps === this.limit ? 0 : exact - steps;

		return steps;
	}
}
