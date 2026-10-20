import { readFileSync } from 'fs';
import { initSync } from '../../bindings/web/pkg/web.js';


export function load() : void
{
	initSync( { module : readFileSync( new URL( '../../bindings/web/pkg/web_bg.wasm', import.meta.url ) ) } );
}
