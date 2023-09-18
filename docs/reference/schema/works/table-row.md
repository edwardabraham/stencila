# Table Row

**A row within a Table.**

**`@id`**: `stencila:TableRow`

## Properties

The `TableRow` type has these properties:

| Name    | `@id`                                | Type                                                                                                           | Description                   | Inherited from                                                                                        |
| ------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------- |
| id      | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                | The identifier for this item  | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)      |
| cells   | `stencila:cells`                     | [`TableCell`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-cell.md)*       | An array of cells in the row. | [`TableRow`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-row.md) |
| rowType | `stencila:rowType`                   | [`TableRowType`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-row-type.md) | The type of row.              | [`TableRow`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-row.md) |

## Related

The `TableRow` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `TableRow` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                            | Encoding       | Decoding     | Status                 | Notes                                                                                 |
| ------------------------------------------------------------------------------------------------- | -------------- | ------------ | ---------------------- | ------------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/HTML.md)             | 🔷 Low loss     |              | 🚧 Under development    | Encoded to tag [`<tr>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tr) |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JATS.md)             | 🔷 Low loss     |              | 🚧 Under development    |                                                                                       |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Markdown.md)     | 🔷 Low loss     |              | 🚧 Under development    |                                                                                       |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Plain text.md) | 🟥 High loss    |              | 🟥 Alpha                |                                                                                       |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                       |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON5.md)           | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                       |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/YAML.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                       |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Debug.md)           | 🔷 Low loss     |              | 🟢 Stable               |                                                                                       |

## Bindings

The `TableRow` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/TableRow.jsonld)
- [JSON Schema](https://stencila.dev/TableRow.schema.json)
- Python class [`TableRow`](https://github.com/stencila/stencila/blob/main/python/stencila/types/table_row.py)
- Rust struct [`TableRow`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/table_row.rs)
- TypeScript class [`TableRow`](https://github.com/stencila/stencila/blob/main/typescript/src/types/TableRow.ts)

## Source

This documentation was generated from [`TableRow.yaml`](https://github.com/stencila/stencila/blob/main/schema/TableRow.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).