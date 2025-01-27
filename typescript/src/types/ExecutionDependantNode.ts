// Generated file; do not edit. See `../rust/schema-gen` crate.

import { hydrate } from "../hydrate.js";

import { type Button } from "./Button.js";
import { type Call } from "./Call.js";
import { type CodeChunk } from "./CodeChunk.js";
import { type CodeExpression } from "./CodeExpression.js";
import { type Division } from "./Division.js";
import { type File } from "./File.js";
import { type Parameter } from "./Parameter.js";
import { type Span } from "./Span.js";
import { type Variable } from "./Variable.js";

/**
 * Node types that can be execution dependencies.
 */
export type ExecutionDependantNode =
  Button |
  Call |
  CodeChunk |
  CodeExpression |
  Division |
  File |
  Parameter |
  Span |
  Variable;

/**
 * Create a `ExecutionDependantNode` from an object
 */
export function executionDependantNode(other: ExecutionDependantNode): ExecutionDependantNode {
  switch(other.type) {
    case "Button":
    case "Call":
    case "CodeChunk":
    case "CodeExpression":
    case "Division":
    case "File":
    case "Parameter":
    case "Span":
    case "Variable":
      return hydrate(other) as ExecutionDependantNode
    default:
      throw new Error(`Unexpected type for ExecutionDependantNode: ${other.type}`);
  }
}
