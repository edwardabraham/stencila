# Strong

**Strongly emphasized content.**

**`@id`**: `stencila:Strong`

## Properties

The `Strong` type has these properties:

| Name    | `@id`                                | Type                                                                                              | Description                  | Inherited from                                                                                   |
| ------- | ------------------------------------ | ------------------------------------------------------------------------------------------------- | ---------------------------- | ------------------------------------------------------------------------------------------------ |
| id      | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)   | The identifier for this item | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| content | `stencila:content`                   | [`Inline`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/inline.md)* | The content that is marked.  | [`Mark`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/mark.md)     |

## Related

The `Strong` type is related to these types:

- Parents: [`Mark`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/mark.md)
- Children: none

## Formats

The `Strong` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                            | Encoding       | Decoding     | Status                 | Notes                                                                                             |
| ------------------------------------------------------------------------------------------------- | -------------- | ------------ | ---------------------- | ------------------------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/HTML.md)             | 🔷 Low loss     |              | 🚧 Under development    | Encoded to tag [`<strong>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/strong)     |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JATS.md)             | 🔷 Low loss     |              | 🚧 Under development    | Encoded to tag [`<bold>`](https://jats.nlm.nih.gov/articleauthoring/tag-library/1.3/element/bold) |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Markdown.md)     | 🟢 No loss      |              | 🚧 Under development    | Encoded using template `**{content}**`                                                            |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Plain text.md) | 🟥 High loss    |              | 🟥 Alpha                |                                                                                                   |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                                   |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON5.md)           | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                                   |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/YAML.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                                   |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Debug.md)           | 🔷 Low loss     |              | 🟢 Stable               |                                                                                                   |

## Bindings

The `Strong` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Strong.jsonld)
- [JSON Schema](https://stencila.dev/Strong.schema.json)
- Python class [`Strong`](https://github.com/stencila/stencila/blob/main/python/stencila/types/strong.py)
- Rust struct [`Strong`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/strong.rs)
- TypeScript class [`Strong`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Strong.ts)

## Source

This documentation was generated from [`Strong.yaml`](https://github.com/stencila/stencila/blob/main/schema/Strong.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).