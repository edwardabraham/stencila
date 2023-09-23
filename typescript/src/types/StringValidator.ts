// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Entity } from './Entity';
import { Integer } from './Integer';

// A schema specifying constraints on a string node.
export class StringValidator extends Entity {
  type = "StringValidator";

  // The minimum length for a string node.
  minLength?: Integer;

  // The maximum length for a string node.
  maxLength?: Integer;

  // A regular expression that a string node must match.
  pattern?: string;

  constructor(options?: StringValidator) {
    super()
    if (options) Object.assign(this, options)
    
  }
}
