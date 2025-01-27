// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Block } from "./Block.js";
import { Integer } from "./Integer.js";
import { Node } from "./Node.js";
import { Thing } from "./Thing.js";

/**
 * A single item in a list.
 */
export class ListItem extends Thing {
  type = "ListItem";

  /**
   * The content of the list item.
   */
  content: Block[];

  /**
   * The item represented by this list item.
   */
  item?: Node;

  /**
   * A flag to indicate if this list item is checked.
   */
  isChecked?: boolean;

  /**
   * The position of the item in a series or sequence of items.
   */
  position?: Integer;

  constructor(content: Block[], options?: Partial<ListItem>) {
    super();
    if (options) Object.assign(this, options);
    this.content = content;
  }
}

/**
* Create a new `ListItem`
*/
export function listItem(content: Block[], options?: Partial<ListItem>): ListItem {
  return new ListItem(content, options);
}
