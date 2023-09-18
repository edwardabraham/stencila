# Table Cell

**A cell within a `Table`.**

**`@id`**: `stencila:TableCell`

## Properties

The `TableCell` type has these properties:

| Name       | `@id`                                    | Type                                                                                                                                                                                                | Description                         | Inherited from                                                                                          |
| ---------- | ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------- | ------------------------------------------------------------------------------------------------------- |
| id         | [`schema:id`](https://schema.org/id)     | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                                                                                     | The identifier for this item        | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)        |
| cellType   | `stencila:cellType`                      | [`TableCellType`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-cell-type.md)                                                                                    | The type of cell.                   | [`TableCell`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-cell.md) |
| name       | [`schema:name`](https://schema.org/name) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                                                                                     | The name of the cell.               | [`TableCell`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-cell.md) |
| columnSpan | `stencila:colspan`                       | [`Integer`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md)                                                                                                   | How many columns the cell extends.  | [`TableCell`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-cell.md) |
| rowSpan    | `stencila:rowspan`                       | [`Integer`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md)                                                                                                   | How many columns the cell extends.  | [`TableCell`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-cell.md) |
| content    | `stencila:content`                       | [`Block`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/block.md)* \| [`Inline`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/inline.md)* | Contents of the table cell.         | [`TableCell`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-cell.md) |

## Related

The `TableCell` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `TableCell` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                            | Encoding       | Decoding     | Status                 | Notes                                                                                 |
| ------------------------------------------------------------------------------------------------- | -------------- | ------------ | ---------------------- | ------------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/HTML.md)             | 🔷 Low loss     |              | 🚧 Under development    | Encoded to tag [`<td>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td) |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JATS.md)             | 🔷 Low loss     |              | 🚧 Under development    |                                                                                       |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Markdown.md)     | 🔷 Low loss     |              | 🚧 Under development    |                                                                                       |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Plain text.md) | 🟥 High loss    |              | 🟥 Alpha                |                                                                                       |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                       |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON5.md)           | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                       |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/YAML.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                       |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Debug.md)           | 🔷 Low loss     |              | 🟢 Stable               |                                                                                       |

## Bindings

The `TableCell` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/TableCell.jsonld)
- [JSON Schema](https://stencila.dev/TableCell.schema.json)
- Python class [`TableCell`](https://github.com/stencila/stencila/blob/main/python/stencila/types/table_cell.py)
- Rust struct [`TableCell`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/table_cell.rs)
- TypeScript class [`TableCell`](https://github.com/stencila/stencila/blob/main/typescript/src/types/TableCell.ts)

## Source

This documentation was generated from [`TableCell.yaml`](https://github.com/stencila/stencila/blob/main/schema/TableCell.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).