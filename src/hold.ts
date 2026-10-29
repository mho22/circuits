const DELAY = 500;


export class Hold
{
	private timer : ReturnType<typeof setTimeout> | undefined;


	wait( action : () => void ) : void
	{
		clearTimeout( this.timer );

		this.timer = setTimeout( action, DELAY );
	}

	cancel() : void
	{
		clearTimeout( this.timer );
	}
}
