// Generated file; do not edit. See `../rust/schema-gen` crate.

import { CiteOrString } from './CiteOrString';
import { Inline } from './Inline';
import { Mark } from './Mark';

// Inline, quoted content.
export class Quote extends Mark {
  type = "Quote";

  // The source of the quote.
  cite?: CiteOrString;

  constructor(content: Inline[], options?: Quote) {
    super(content)
    if (options) Object.assign(this, options)
    this.content = content;
  }
}
