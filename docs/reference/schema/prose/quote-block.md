# Quote Block

**A section quoted from somewhere else.
**

**`@id`**: `stencila:QuoteBlock`

## Properties

The `QuoteBlock` type has these properties:

| Name    | `@id`                                | Type                                                                                                                                                                                           | Description                  | Inherited from                                                                                            |
| ------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- | --------------------------------------------------------------------------------------------------------- |
| id      | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                                                                                | The identifier for this item | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)          |
| cite    | `stencila:cite`                      | [`Cite`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/cite.md) \| [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The source of the quote.     | [`QuoteBlock`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/quote-block.md) |
| content | `stencila:content`                   | [`Block`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/block.md)*                                                                                                | The content of the quote.    | [`QuoteBlock`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/quote-block.md) |

## Related

The `QuoteBlock` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `QuoteBlock` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                            | Encoding       | Decoding     | Status                 | Notes |
| ------------------------------------------------------------------------------------------------- | -------------- | ------------ | ---------------------- | ----- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/HTML.md)             | 🔷 Low loss     |              | 🚧 Under development    |       |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JATS.md)             | 🔷 Low loss     |              | 🚧 Under development    |       |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Markdown.md)     | 🟥 High loss    |              | 🚧 Under development    |       |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Plain text.md) | 🟥 High loss    |              | 🟥 Alpha                |       |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON5.md)           | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/YAML.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Debug.md)           | 🔷 Low loss     |              | 🟢 Stable               |       |

## Bindings

The `QuoteBlock` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/QuoteBlock.jsonld)
- [JSON Schema](https://stencila.dev/QuoteBlock.schema.json)
- Python class [`QuoteBlock`](https://github.com/stencila/stencila/blob/main/python/stencila/types/quote_block.py)
- Rust struct [`QuoteBlock`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/quote_block.rs)
- TypeScript class [`QuoteBlock`](https://github.com/stencila/stencila/blob/main/typescript/src/types/QuoteBlock.ts)

## Source

This documentation was generated from [`QuoteBlock.yaml`](https://github.com/stencila/stencila/blob/main/schema/QuoteBlock.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).