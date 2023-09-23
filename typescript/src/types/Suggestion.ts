// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Entity } from './Entity';
import { Inline } from './Inline';

// Abstract base class for nodes that indicate a suggested change to inline content.
export class Suggestion extends Entity {
  type = "Suggestion";

  // The content that is suggested to be inserted or deleted.
  content: Inline[];

  constructor(content: Inline[], options?: Suggestion) {
    super()
    if (options) Object.assign(this, options)
    this.content = content;
  }
}
