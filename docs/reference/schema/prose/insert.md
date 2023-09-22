# Insert

**A suggestion to insert some inline content.**

**`@id`**: `stencila:Insert`

## Properties

The `Insert` type has these properties:

| Name    | `@id`                                | Type                                                                                              | Description                                              | Inherited from                                                                                           |
| ------- | ------------------------------------ | ------------------------------------------------------------------------------------------------- | -------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| id      | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)   | The identifier for this item                             | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)         |
| content | `stencila:content`                   | [`Inline`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/inline.md)* | The content that is suggested to be inserted or deleted. | [`Suggestion`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/suggestion.md) |

## Related

The `Insert` type is related to these types:

- Parents: [`Suggestion`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/suggestion.md)
- Children: none

## Formats

The `Insert` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding         | Decoding     | Status                 | Notes                                                                                   |
| --------------------------------------------------------------------------------------------- | ---------------- | ------------ | ---------------------- | --------------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🔷 Low loss       |              | 🚧 Under development    | Encoded to tag [`<ins>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ins) |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         |                  |              | 🚧 Under development    | Encoded using special function                                                          |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | ⚠️ High loss     |              | 🚧 Under development    | Encoded using template `<ins>{content}</ins>`                                           |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | ⚠️ High loss     |              | ⚠️ Alpha               |                                                                                         |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                         |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                         |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                         |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss       |              | 🟢 Stable               |                                                                                         |

## Bindings

The `Insert` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Insert.jsonld)
- [JSON Schema](https://stencila.dev/Insert.schema.json)
- Python class [`Insert`](https://github.com/stencila/stencila/blob/main/python/stencila/types/insert.py)
- Rust struct [`Insert`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/insert.rs)
- TypeScript class [`Insert`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Insert.ts)

## Source

This documentation was generated from [`Insert.yaml`](https://github.com/stencila/stencila/blob/main/schema/Insert.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).