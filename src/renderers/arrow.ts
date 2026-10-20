const SIZE = 5;


export function draw( context : CanvasRenderingContext2D, style : string, x : number, y : number, directionX : number, directionY : number ) : void
{
	context.fillStyle = style;
	context.beginPath();
	context.moveTo( x + directionX * SIZE, y + directionY * SIZE );
	context.lineTo( x - directionX * SIZE - directionY * SIZE, y - directionY * SIZE + directionX * SIZE );
	context.lineTo( x - directionX * SIZE + directionY * SIZE, y - directionY * SIZE - directionX * SIZE );
	context.closePath();
	context.fill();
}
