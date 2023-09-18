# Quote

**Inline, quoted content.**

**`@id`**: `stencila:Quote`

## Properties

The `Quote` type has these properties:

| Name    | `@id`                                | Type                                                                                                                                                                                           | Description                  | Inherited from                                                                                   |
| ------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- | ------------------------------------------------------------------------------------------------ |
| id      | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                                                                                | The identifier for this item | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| content | `stencila:content`                   | [`Inline`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/inline.md)*                                                                                              | The content that is marked.  | [`Mark`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/mark.md)     |
| cite    | `stencila:cite`                      | [`Cite`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/cite.md) \| [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The source of the quote.     | [`Quote`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/quote.md)   |

## Related

The `Quote` type is related to these types:

- Parents: [`Mark`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/mark.md)
- Children: none

## Formats

The `Quote` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                            | Encoding       | Decoding     | Status                 | Notes                                                                               |
| ------------------------------------------------------------------------------------------------- | -------------- | ------------ | ---------------------- | ----------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/HTML.md)             | 🔷 Low loss     |              | 🚧 Under development    | Encoded to tag [`<q>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/q) |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JATS.md)             | 🔷 Low loss     |              | 🚧 Under development    |                                                                                     |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Markdown.md)     | 🟥 High loss    |              | 🚧 Under development    | Encoded using template `<q>{content}</q>`                                           |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Plain text.md) | 🟥 High loss    |              | 🟥 Alpha                |                                                                                     |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                     |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON5.md)           | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                     |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/YAML.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                     |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Debug.md)           | 🔷 Low loss     |              | 🟢 Stable               |                                                                                     |

## Bindings

The `Quote` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Quote.jsonld)
- [JSON Schema](https://stencila.dev/Quote.schema.json)
- Python class [`Quote`](https://github.com/stencila/stencila/blob/main/python/stencila/types/quote.py)
- Rust struct [`Quote`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/quote.rs)
- TypeScript class [`Quote`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Quote.ts)

## Source

This documentation was generated from [`Quote.yaml`](https://github.com/stencila/stencila/blob/main/schema/Quote.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).