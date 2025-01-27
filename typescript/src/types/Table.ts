// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Block } from "./Block.js";
import { CreativeWork } from "./CreativeWork.js";
import { TableRow } from "./TableRow.js";

/**
 * A table.
 */
export class Table extends CreativeWork {
  type = "Table";

  /**
   * A caption for the table.
   */
  caption?: Block[];

  /**
   * A short label for the table.
   */
  label?: string;

  /**
   * Rows of cells in the table.
   */
  rows: TableRow[];

  constructor(rows: TableRow[], options?: Partial<Table>) {
    super();
    if (options) Object.assign(this, options);
    this.rows = rows;
  }
}

/**
* Create a new `Table`
*/
export function table(rows: TableRow[], options?: Partial<Table>): Table {
  return new Table(rows, options);
}
