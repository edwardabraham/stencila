// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Cord } from "./Cord.js";
import { Math } from "./Math.js";

/**
 * A block of math, e.g an equation, to be treated as block content.
 */
export class MathBlock extends Math {
  type = "MathBlock";

  /**
   * A short label for the math block.
   */
  label?: string;

  constructor(mathLanguage: string, code: Cord, options?: Partial<MathBlock>) {
    super(mathLanguage, code);
    if (options) Object.assign(this, options);
    this.mathLanguage = mathLanguage;
    this.code = code;
  }
}

/**
* Create a new `MathBlock`
*/
export function mathBlock(mathLanguage: string, code: Cord, options?: Partial<MathBlock>): MathBlock {
  return new MathBlock(mathLanguage, code, options);
}
