import type { Picker } from './types/picker.ts';


function anchor( text : string, name : string ) : void
{
	const link = document.createElement( 'a' );

	link.href = URL.createObjectURL( new Blob( [ text ], { type : 'application/json' } ) );
	link.download = name;
	link.click();

	URL.revokeObjectURL( link.href );
}


export async function download( text : string, name : string ) : Promise<string>
{
	const system = window as unknown as { showSaveFilePicker? : Picker };

	if( ! system.showSaveFilePicker ) { anchor( text, name ); return name; }

	const handle = await system.showSaveFilePicker( { suggestedName : name, types : [ { description : 'Circuit', accept : { 'application/json' : [ '.circuit' ] } } ] } );
	const stream = await handle.createWritable();

	await stream.write( text );
	await stream.close();

	return handle.name;
}
