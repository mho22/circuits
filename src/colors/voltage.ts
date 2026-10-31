const FROZEN = 'rgb(24, 48, 128)';
const GRAY = 'rgb(128, 128, 128)';
const GREEN = 'rgb(0, 255, 0)';


export function color( voltage : number, scale : number ) : string
{
	if( Number.isNaN( scale ) ) return FROZEN;

	const ratio = scale > 0 && Number.isFinite( voltage ) ? Math.max( -1, Math.min( 1, voltage / scale ) ) : 0;

	const red = ratio < 0 ? 128 - 127 * ratio : 128 - 128 * ratio;
	const green = ratio > 0 ? 128 + 127 * ratio : 128 + 128 * ratio;
	const blue = 128 - 128 * Math.abs( ratio );

	return `rgb(${ Math.round( red ) }, ${ Math.round( green ) }, ${ Math.round( blue ) })`;
}


export function bus( voltages : number[], scale : number ) : string
{
	if( Number.isNaN( scale ) ) return FROZEN;

	return voltages.some( voltage => color( voltage, scale ) !== GRAY ) ? GREEN : GRAY;
}
