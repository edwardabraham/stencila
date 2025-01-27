// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Entity } from "./Entity.js";

/**
 * A validator specifying the constraints on a numeric node.
 */
export class NumberValidator extends Entity {
  type = "NumberValidator";

  /**
   * The inclusive lower limit for a numeric node.
   */
  minimum?: number;

  /**
   * The exclusive lower limit for a numeric node.
   */
  exclusiveMinimum?: number;

  /**
   * The inclusive upper limit for a numeric node.
   */
  maximum?: number;

  /**
   * The exclusive upper limit for a numeric node.
   */
  exclusiveMaximum?: number;

  /**
   * A number that a numeric node must be a multiple of.
   */
  multipleOf?: number;

  constructor(options?: Partial<NumberValidator>) {
    super();
    if (options) Object.assign(this, options);
    
  }
}

/**
* Create a new `NumberValidator`
*/
export function numberValidator(options?: Partial<NumberValidator>): NumberValidator {
  return new NumberValidator(options);
}
