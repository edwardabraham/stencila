// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Inline } from "./Inline.js";
import { Suggestion } from "./Suggestion.js";

/**
 * A suggestion to delete some inline content.
 */
export class Delete extends Suggestion {
  type = "Delete";

  constructor(content: Inline[], options?: Partial<Delete>) {
    super(content);
    if (options) Object.assign(this, options);
    this.content = content;
  }
}

/**
* Create a new `Delete`
*/
export function delete_(content: Inline[], options?: Partial<Delete>): Delete {
  return new Delete(content, options);
}
