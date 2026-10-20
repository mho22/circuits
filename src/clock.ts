export class Clock
{
	private remainder = 0;


	constructor( private rate : number, private limit : number ) {}


	tick( seconds : number ) : number
	{
		const exact = this.remainder + seconds * this.rate;
		const steps = Math.min( Math.floor( exact ), this.limit );

		this.remainder = steps === this.limit ? 0 : exact - steps;

		return steps;
	}
}
