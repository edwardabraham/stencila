// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Entity } from "./Entity.js";
import { Inline } from "./Inline.js";

/**
 * A paragraph.
 */
export class Paragraph extends Entity {
  type = "Paragraph";

  /**
   * The contents of the paragraph.
   */
  content: Inline[];

  constructor(content: Inline[], options?: Partial<Paragraph>) {
    super();
    if (options) Object.assign(this, options);
    this.content = content;
  }
}

/**
* Create a new `Paragraph`
*/
export function paragraph(content: Inline[], options?: Partial<Paragraph>): Paragraph {
  return new Paragraph(content, options);
}
