// Generated file; do not edit. See `../rust/schema-gen` crate.

import { CodeStatic } from './CodeStatic';
import { Cord } from './Cord';

// A code block.
export class CodeBlock extends CodeStatic {
  type = "CodeBlock";

  constructor(code: Cord, options?: CodeBlock) {
    super(code)
    if (options) Object.assign(this, options)
    this.code = code;
  }
}
