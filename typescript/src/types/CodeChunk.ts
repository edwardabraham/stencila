// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Block } from "./Block.js";
import { CodeExecutable } from "./CodeExecutable.js";
import { Cord } from "./Cord.js";
import { Node } from "./Node.js";

/**
 * A executable chunk of code.
 */
export class CodeChunk extends CodeExecutable {
  type = "CodeChunk";

  /**
   * Whether the code should be treated as side-effect free when executed.
   */
  executionPure?: boolean;

  /**
   * Outputs from executing the chunk.
   */
  outputs?: Node[];

  /**
   * A short label for the CodeChunk.
   */
  label?: string;

  /**
   * A caption for the CodeChunk.
   */
  caption?: Block[];

  constructor(code: Cord, options?: Partial<CodeChunk>) {
    super(code);
    if (options) Object.assign(this, options);
    this.code = code;
  }
}

/**
* Create a new `CodeChunk`
*/
export function codeChunk(code: Cord, options?: Partial<CodeChunk>): CodeChunk {
  return new CodeChunk(code, options);
}
