const THRESHOLD = 2.5;


export function bit( potential : number ) : number
{
	return potential > THRESHOLD ? 1 : 0;
}
