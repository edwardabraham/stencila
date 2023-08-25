// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Block } from './Block';
import { NoteType } from './NoteType';

// Additional content which is not part of the main content of a document.
export class Note {
  type = "Note";

  // The identifier for this item
  id?: string;

  // Determines where the note content is displayed within the document.
  noteType: NoteType;

  // Content of the note, usually a paragraph.
  content: Block[];

  constructor(noteType: NoteType, content: Block[], options?: Note) {
    if (options) Object.assign(this, options)
    this.noteType = noteType;
    this.content = content;
  }
}