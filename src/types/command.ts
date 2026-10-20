import type { Kind } from '../enums/kind.ts';


export type Command = Kind | 'undo' | 'redo' | 'save' | 'open';
