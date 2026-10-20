export function draw( context : CanvasRenderingContext2D, style : string, width : number, fromX : number, fromY : number, toX : number, toY : number ) : void
{
	context.strokeStyle = style;
	context.lineWidth = width;
	context.beginPath();
	context.moveTo( fromX, fromY );
	context.lineTo( toX, toY );
	context.stroke();
}
